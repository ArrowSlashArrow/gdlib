//! Levels, and the CCLocalLevels.dat savefile that contains them.
//!
//! [`GDLevel`] is one level, and [`CCLocalLevels`] is the whole savefile.
//!
//! # Loading and creating levels
//!
//! - From the savefile: [`CCLocalLevels::from_local`], then look through [`CCLocalLevels::levels`].
//! - From a file: [`GDLevel::from_gmd`] or [`GDLevel::from_gdlvl`].
//! - From scratch: [`GDLevel::new_latest_version`] or [`GDLevel::with_level_data`].
//!
//! # Editing objects
//!
//! Level data is left compressed when a level is loaded, because decompressing it is slow. Decompress it before working with objects:
//!
//! ```no_run
//! use gdlib::cclocallevels::gdlevel::GDLevel;
//!
//! # fn main() -> Result<(), gdlib::core::GDError> {
//! let mut level = GDLevel::from_gmd("level.gmd")?;
//!
//! // Decompresses and parses the level data on first use.
//! if let Some(data) = level.get_decrypted_data_ref() {
//!     for object in data.objects.iter_mut() {
//!         object.config.add_group(42);
//!     }
//! }
//!
//! level.export_to_gmd("edited.gmd")?;
//! # Ok(())
//! # }
//! ```
//!
//! # How fields are organised
//!
//! [`GDLevel`] splits a level's properties into groups: [`identity`](GDLevel::identity), [`content`](GDLevel::content), [`ratings`](GDLevel::ratings), [`coins`](GDLevel::coins), [`player_stats`](GDLevel::player_stats), [`flags`](GDLevel::flags), [`editor_state`](GDLevel::editor_state), [`meta`](GDLevel::meta), [`integrity`](GDLevel::integrity) and [`unknowns`](GDLevel::unknowns).
//!
//! Most field docs name the key they're stored under (such as `k2`). Keys that gdlib doesn't recognise are kept in [`GDLevelUnknowns::other`] and written back unchanged, so they survive a round trip.
//!
//! # Common pitfalls
//!
//! - **Use [`GDLevel::new_latest_version`] for new levels.** [`GDLevel::default`] leaves the binary version at the oldest known game version, and GD needs a valid value for that key to open the level.
//! - **Object methods can do nothing on compressed data.** [`GDLevel::add_object`] and [`GDLevel::add_objects`] silently do nothing, and [`GDLevel::get_objects_ref`] returns `None`, until the data is decompressed. Call [`GDLevel::decrypt_level_data`] first.
//! - **One bad level fails the whole savefile.** If any level can't be parsed, [`CCLocalLevels::from_decrypted`] returns an error.
//! - **Writing to the real savefile overwrites it.** Prefer [`CCLocalLevels::export_to_savefile_with_backup`] over [`CCLocalLevels::export_to_savefile`], and close GD first, since it may overwrite your changes when it saves.
//!
//! # References
//!
//! - <https://boomlings.dev/resources/client/level>

use std::{
    collections::HashMap,
    fmt::Display,
    fs::{read, write},
    io::Cursor,
    path::PathBuf,
};

use plist::{Dictionary, Value};

use crate::{
    cclocallevels::{
        gdlevel::{
            enums::{
                BooleanToggle, DemonType, DifficultyRating, EpicRating, GDLevelDifficulty,
                GDLevelType, Length, OfficialSong,
            },
            leveldata::{
                DEFAULT_LEVEL_HEADERS, GDEncryptedLevelData, GDLevelData, GDLevelState,
                parse_raw_level_headers,
            },
            version::GDVersion,
        },
        gdlist::GDList,
        gdobj::GDObject,
    },
    core::{
        GDError, b64_decode, b64_encode, get_cclocallevels_path,
        io::{
            atomic_export, decompress, decrypt_file, encrypt_level_str, encrypt_savefile_str,
            stringify_xml, vec_as_str, vec_as_str_lossy,
        },
        proper_plist_tags,
        structs::KCEKValue,
    },
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub mod enums;
pub mod leveldata;
pub mod version;

/// Keys that are stripped from in-game .gdlvl exports of levels.
///
/// Note: All keys here are stored as their numerical form. For each key, its string counterpart is `k{id}`.
pub static GDLVL_NOT_EXPORTED_KEYS: &[u16] = &[
    1, 5, 6, 60, 15, 16, 98, 99, 41, 42, 47, 67, 14, 65, 61, 62, 63, 9, 10, 11, 22, 24, 27, 75, 7,
    26, 25, 76, 74, 77, 78, 82, 84, 83, 18, 154, 36, 85, 86, 87, 88, 89, 19, 71, 90, 20, 68, 107,
    115, 117, 121, 119, 108, 116, 118, 122, 120, 111, 123, 124, 128, 130, 132, 131, 129, 109, 110,
    133, 134, 135, 136, 137, 138, 139, 142, 143, 145, 146, 140, 141, 151, 152, 69, 93, 73, 112,
    126, 127,
];

/// Standard header of a GD plist.
pub const PLIST_HEADER: &str = "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\">";
/// Standard footer of a GD plist.
pub const PLIST_FOOTER: &str = "</plist>";

/// Container struct for the CCLocalLevels savefile.
#[derive(Debug, Clone)]
pub struct CCLocalLevels {
    /// All levels in the savefile in order from newest to oldest.
    pub levels: Vec<GDLevel>,
    /// The version of the GD binary that this savefile is from. This value is **CRUCIAL** for `CCLocalLevels.dat` and ***<u>MUST NOT BE REMOVED!</u>***
    pub binary_version: GDVersion,
    /// All lists in the savefile in order from newest to oldest.
    pub lists: Vec<GDList>,
    /// Other keys in CCLocalLevels.dat that weren't parsed.
    pub other: HashMap<String, Value>,
}

impl CCLocalLevels {
    /// Returns the levels in CCLocalLevels.dat if retrievable
    #[inline]
    pub fn from_local() -> Result<Self, GDError> {
        CCLocalLevels::from_decrypted(decrypt_file(
            get_cclocallevels_path().ok_or(GDError::MissingSavefile)?,
        )?)
    }

    /// Parses raw savefile string into this struct
    pub fn from_decrypted(s: String) -> Result<Self, GDError> {
        if !s.starts_with(PLIST_HEADER) {
            return Err(GDError::CorruptedSavefile("Savefile header does not match the expected header. This may be due to a corrupted savefile or a savefile from a previous version of GD.".into()));
        };

        let mut xmltree = Value::from_reader_xml(Cursor::new(proper_plist_tags(s)?.as_bytes()))?
            .into_dictionary()
            .ok_or(GDError::CorruptedSavefile(
                "CCLocalLevels.dat is not a dict".into(),
            ))?;

        let levels_dict = xmltree
            .remove("LLM_01")
            .to_owned()
            .ok_or(GDError::CorruptedSavefile("No LLM_01 (levels)".into()))?
            .into_dictionary()
            .ok_or(GDError::CorruptedSavefile("LLM_01 is not a dict".into()))?;

        let llm_02 = xmltree
            .remove("LLM_02")
            .ok_or(GDError::CorruptedSavefile(
                "No LLM_02 (binary version)".into(),
            ))?
            .as_signed_integer()
            .ok_or(GDError::CorruptedSavefile(
                "GDVersion is not an integer.".into(),
            ))? as i32;
        let llm_03 = xmltree
            .remove("LLM_03")
            .unwrap_or_else(|| Value::Dictionary(Dictionary::default()));

        // these are stored as "k_0": <level>, "k_1": <level>, etc. in the savefile,
        // the vec prserves that order.
        let collected_dicts = levels_dict
            .into_iter()
            .filter_map(|(k, v)| match k.as_str() {
                "_isArr" => None,
                _ => v.into_dictionary(),
            })
            .collect::<Vec<Dictionary>>();

        #[cfg(feature = "parallel")]
        let levels_parsed: Vec<GDLevel> = collected_dicts
            .par_iter()
            .map(GDLevel::from_dict)
            .collect::<Option<_>>()
            .ok_or(GDError::CorruptedSavefile("Unable to parse levels".into()))?;

        #[cfg(not(feature = "parallel"))]
        let levels_parsed: Vec<GDLevel> = collected_dicts
            .iter()
            .map(GDLevel::from_dict)
            .collect::<Option<_>>()
            .ok_or(GDError::CorruptedSavefile("Unable to parse levels".into()))?;

        let lists = CCLocalLevels::parse_lists_from_value(&llm_03)?;

        let levels = CCLocalLevels {
            levels: levels_parsed,
            binary_version: GDVersion::from_binary_version(llm_02),
            lists,
            other: xmltree.into_iter().collect(),
        };

        Ok(levels)
    }

    /// Parses the savefile's `LLM_03` dictionary (`k_0`, `k_1`, ...) into its lists. Returns an error if the value is not a dictionary or any list fails to parse.
    pub(crate) fn parse_lists_from_value(v: &Value) -> Result<Vec<GDList>, GDError> {
        /* Input dict structure
         * "_isArr": always Boolean(true)
         * k_X: list at index X
         */
        let dict = v.as_dictionary().ok_or(GDError::CorruptedSavefile(
            "Input value is not a dictionary.".into(),
        ))?;
        let mut lists = Vec::with_capacity(dict.len());

        for (_, v) in dict {
            if let Value::Dictionary(d) = v {
                lists.push(GDList::from_dictionary(d).ok_or(GDError::CorruptedSavefile(
                    "Unable to parse into a valid GD list.".into(),
                ))?)
            }
        }

        Ok(lists)
    }

    /// Adds a level to the beginning of `self.levels`
    pub fn add_level(&mut self, level: GDLevel) {
        self.levels.insert(0, level);
    }

    /// Serialises this object to a formatted .plist structure.
    ///
    /// **NOTE:** The caller *MUST* ensure that `self.binary_version` is a value that will return `Some`.
    pub fn serialise_to_string(&mut self) -> String {
        let mut dict = Dictionary::new();

        /* levels */

        let mut levels_dict = Dictionary::new();
        levels_dict.insert("_isArr".to_string(), Value::from(true));

        #[cfg(feature = "parallel")]
        let level_dict_entries = self
            .levels
            .par_iter()
            .enumerate()
            .map(|(idx, level)| (format!("k_{idx}"), Value::from(level.to_dict())))
            .collect::<Vec<(String, Value)>>();

        #[cfg(not(feature = "parallel"))]
        let level_dict_entries = self
            .levels
            .iter()
            .enumerate()
            .map(|(idx, level)| (format!("k_{idx}"), Value::from(level.to_dict())))
            .collect::<Vec<(String, Value)>>();

        levels_dict.extend(level_dict_entries.into_iter());

        /* lists */

        let mut lists_dict = Dictionary::new();
        lists_dict.insert("_isArr".to_string(), Value::from(true));

        #[cfg(feature = "parallel")]
        let list_dict_entries = self
            .lists
            .par_iter()
            .enumerate()
            .map(|(idx, level)| (format!("k_{idx}"), Value::from(level.to_dict())))
            .collect::<Vec<(String, Value)>>();

        #[cfg(not(feature = "parallel"))]
        let list_dict_entries = self
            .lists
            .iter()
            .enumerate()
            .map(|(idx, level)| (format!("k_{idx}"), Value::from(level.to_dict())))
            .collect::<Vec<(String, Value)>>();

        lists_dict.extend(list_dict_entries.into_iter());

        dict.insert("LLM_01".to_string(), Value::Dictionary(levels_dict));
        dict.insert(
            "LLM_02".to_string(),
            Value::from(
                self.binary_version
                    .to_binary_version_id()
                    .unwrap_or_default(), // danger! binaryVersion 0 may cause side effects on newer versions!
            ),
        );
        dict.insert("LLM_03".to_string(), Value::Dictionary(lists_dict));

        format!("{PLIST_HEADER}{}{PLIST_FOOTER}", stringify_xml(&dict, true))
    }

    /// Exports this struct as encrypted XML to the standard CCLocalLevels.dat location.
    /// This function will return an error if it is unable to find the savefile at the standard location.
    ///
    /// Standard location on windows: %LOCALAPPDATA%\GeometryDash
    /// Standard location on linux: [`crate::core::io::linux_gd_files_path`]
    pub fn export_to_savefile(&mut self) -> Result<(), GDError> {
        let savefile = get_cclocallevels_path().ok_or(GDError::MissingSavefile)?;
        let export_str = encrypt_savefile_str(&&self.serialise_to_string());
        atomic_export(savefile, &export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to a given file
    pub fn export_to_file(&mut self, file: PathBuf) -> Result<(), GDError> {
        let export_str = encrypt_savefile_str(self.serialise_to_string().as_str());
        write(file, export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat and creates a backup, CCLocalLevels.dat.bak
    pub fn export_to_savefile_with_backup(&mut self) -> Result<(), GDError> {
        let savefile = get_cclocallevels_path().ok_or(GDError::MissingSavefile)?;
        let backup_path = format!("{}.bak", savefile.to_string_lossy());
        write(backup_path, read(&savefile)?)?;

        let export_str = encrypt_savefile_str(&self.serialise_to_string());
        atomic_export(savefile, &export_str)?;
        Ok(())
    }
}

/* Notes for maintainers
 * 1. There are some foreign types used, all of which should implement ToString for serialisation. If they do not, they should be left as-is
 * 2. There are some differences between GDLevel and the way it is serialised. Those differences should be marked with comments above the field:
 *      PARSE: what to do when parsing this field
 *      STORE: what to do when serializing this field
 * 3. Some fields are optional, while some fields are not. Option<T> indicates that there is a difference between Some(T::default()) and None, usually that the field is irrelevant in the case that it is parsed as None.
 * 4. The corresponding keys of each value have been marked in a comment beside each field.
 * 5. Struct fields that are unknown, unused, or unclear as annotated as such with comments.
 */

#[derive(Debug, Clone)]
/// The descriptor struct for GD levels which contains all known properties.
///
/// Reference: <https://boomlings.dev/resources/client/level>
pub struct GDLevel {
    /// Identity info: ID, name, descriptior, creator, version, level type, password
    pub identity: GDLevelIdentity,
    /// The level's data: object data, song list, used song ID, length, is platformer/2-player
    pub content: GDLevelContents,
    /// Rating info: downloads, likes, stars, requested stars, epic rate, difficulty, type of demon, is auto
    pub ratings: GDLevelRatings,
    /// Coin info: required coins, obtainment status of coins
    pub coins: GDLevelCoins,
    /// Player-obtained stats: attempts, jump, percentage, best attempt time, completions, leaderboard standing, verification time, level progresses
    pub player_stats: GDLevelPlayerStats,
    /// Boolean flags: is editable, is verified, is uploaded, is unlisted, etc.
    pub flags: GDLevelFlags,
    /// State of level in editor: camera position and zoom, build tab pages, last selected layer
    pub editor_state: GDLevelEditorState,
    /// Internal data: kCEK, folder, seconds spent editing, level size, batch node info, capacity string
    pub meta: GDLevelMeta,
    /// Integrity/verification info: level seed, was anticheat triggered, replay data, vFDCHk
    pub integrity: GDLevelIntegrity,
    /// Properties which are unaccounted for in the struct but are present in the level nonetheless.
    /// Values here are not mutated at all, and are instead stored as they were found in the level dictionary.
    pub unknowns: HashMap<String, Value>,
}

// For maintainers - this struct contains identifying information about the level. Things that can either
// identify (such as the ID or creator) or properties of all levels (e.g. level version, though it isn't necessarily unique)
#[derive(Debug, Clone, Default)]
/// Identity of the level and data surrounding it. Notably, the level name, id, description and creator (among others).
pub struct GDLevelIdentity {
    /// Internal key: `k1`
    pub id: i32,
    /// Internal key: `k2`
    pub name: String,
    // PARSE: base64 decode
    // STORE: base64 encode
    /// Internal key: `k3`
    pub description: Option<String>,
    /// Username of the creator.
    ///
    /// Internal key: `k5`
    pub creator: String,
    /// User ID of the creator.
    ///
    /// Internal key: `k6`
    pub user_id: i32,
    /// Account ID of the creator.
    ///
    /// Internal key: `k60`
    pub account_id: i32,
    /// Internal key: `k16`
    pub level_version: i32,
    /// Internal key: `k46`
    pub level_revision: Option<i32>,
    /// Version of the game that this level was created on.
    ///
    /// Internal key: `k17`
    pub game_version: i32,
    /// Internal key: `k40`
    pub build_version: i32,
    // PARSE: `GDVersion::from_binary_version`
    /// Hardcoded value. LLM_02 is also hardcoded to this value. This value is required for GD to open the level.
    ///
    /// Internal key: `k50`
    pub binary_version: GDVersion,
    /// Type of this level: [`GDLevelType`]
    ///
    /// Internal key: `k21`
    pub level_type: GDLevelType,
    /// ID of the daily/weekly/event level.
    ///
    /// Internal key: `k74`
    pub timely_id: i32,
    /// Original ID of the level this one was copied from
    ///
    /// Internal key: `k42`
    pub original_id: Option<i32>,
    /// Password to copy the level
    ///
    /// Internal key: `k41`
    pub password: Option<i32>,
    /// UNIX timestamp of the level's upload time in seconds. None for local levels that aren't uploaded
    ///
    /// Internal key: `k98`
    pub upload_date: Option<i32>,
    /// UNIX timestamp of the level's update time in seconds. None for local levels that haven't been updated or uploaded.
    ///
    /// Internal key: `k99`
    pub update_date: Option<i32>,
}

#[derive(Debug, Clone)]
/// Values to do with the level data / gameplay. Notably, the level data itself, object count, length, etc.
///
/// This struct also contains data for the platformer saves and level backups.
pub struct GDLevelContents {
    /// The data of the level. See [`GDLevelState`]
    ///
    /// Internal key: `k4`
    pub data: Option<GDLevelState>,
    /// Internal key: `k48`
    pub object_count: i32,
    /// Internal key: `k69`
    pub high_object_count: bool,
    // PARSE: split at each comma and cast to i32
    // STORE: convert back to string and join with commas
    /// List of songs used in the level.
    ///
    /// Internal key: `k104`
    pub song_list: Vec<i32>,
    // PARSE: split at each comma and cast to i32
    // STORE: convert back to string and join with commas
    /// List of sound effects used in this level.
    ///
    /// Internal key: `k105`
    pub sfx_list: Option<Vec<i32>>,
    // note: use enum GDSong
    /// Internal key: `k8`
    pub official_song_id: Option<OfficialSong>,
    /// Internal key: `k45`
    pub custom_song_id: Option<i32>,
    /// Length bracket of this level. See [`Length`].
    ///
    /// Internal key: `k23`
    pub length: Length,
    /// Internal key: `k43`
    pub two_player_mode: bool,
    /// Internal key: `k94`
    pub is_platformer: bool,
    /// Interal key: `k126`
    pub autobackup_id: Option<String>,
    /// Interal key: `k127`
    pub last_autobackup_time: i32,
    // TODO: parse payload
    /// Interal key: `k128`
    pub platformer_save: Option<String>,
    /// Interal key: `k129`
    pub enable_platformer_save: bool,
    /// Interal key: `k130`
    pub platformer_save_level_version: i32,
    // TODO: parse replay string
    /// Interal key: `k131`
    pub platformer_save_replay: Option<Vec<u8>>,
    /// Interal key: `k132`
    pub platformer_save_format_version: i32,
}

impl Default for GDLevelData {
    fn default() -> Self {
        Self {
            headers: parse_raw_level_headers(DEFAULT_LEVEL_HEADERS),
            objects: vec![],
        }
    }
}

/// Returns a default decrypted `GDLevelState` ready for being written to. The value returned is of the same type as the `data` field in [`GDLevelContents`].
pub fn default_level_data() -> Option<GDLevelState> {
    Some(GDLevelState::Decrypted(GDLevelData::default()))
}

impl Default for GDLevelContents {
    fn default() -> Self {
        Self {
            data: default_level_data(),
            object_count: 0,
            high_object_count: false,
            song_list: vec![],
            sfx_list: None,
            official_song_id: None,
            custom_song_id: None,
            length: Length::default(),
            two_player_mode: false,
            is_platformer: false,
            autobackup_id: None,
            last_autobackup_time: 0,
            platformer_save: None,
            enable_platformer_save: false,
            platformer_save_level_version: 0,
            platformer_save_replay: None,
            platformer_save_format_version: 0,
        }
    }
}

#[derive(Debug, Clone)]
/// Miscellaneous metadata surrounding the level. Mostly internal or organizational fields.
pub struct GDLevelMeta {
    // PARSE: i32 -> kcekvalue
    // STORE: kcekvalue as i32
    /// This value is always [`KCEKValue::GJGameLevel`]. For the level to re-encode correctly and be recognized by GD as a level, this value **MUST** remain [`KCEKValue::GJGameLevel`].
    ///
    /// Internal key: `kCEK`
    pub kcek: KCEKValue,
    /// Internal key: `k83`
    pub level_order: i32,
    /// Internal key: `k84`
    pub level_folder: i32,
    /// Internal key: `k80`
    pub seconds_editing: i32,
    /// Internal key: `k81`
    pub seconds_editing_copies: i32,
    /// Approximated as `floor(this.level_string.length() * 0.152)`.
    ///
    /// Internal key: `k39`
    pub level_size: i32,
    /// Internal key: `k51`
    pub capacity_001: i32,
    /// Internal key: `k52`
    pub capacity_002: i32,
    /// Internal key: `k53`
    pub capacity_003: i32,
    /// Internal key: `k54`
    pub capacity_004: i32,
    /// Internal key: `k67`
    pub capacity_string: Option<String>,
}

impl Default for GDLevelMeta {
    fn default() -> Self {
        Self {
            kcek: KCEKValue::GJGameLevel,
            level_order: 0,
            level_folder: 0,
            seconds_editing: 0,
            seconds_editing_copies: 0,
            level_size: 0,
            capacity_001: 0,
            capacity_002: 0,
            capacity_003: 0,
            capacity_004: 0,
            capacity_string: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Rating data about a level (e.g. star rating, likes, downloads)
pub struct GDLevelRatings {
    /// Difficulty rating of this level. See [`DifficultyRating`]. Irrelevant if self.is_demon is true. In that case, self.demon_type determines the difficulty.
    ///
    /// Internal key: `k9`
    pub rating: DifficultyRating,
    // UNCLEAR
    /// Internal key: `k10`
    pub rating_sum: i32,
    /// Internal key: `k11`
    pub downloads: i32,
    /// Internal key: `k22`
    pub like_rating: i32,
    // UNUSED
    /// Number of dislikes the level has (unused).
    ///
    /// Internal key: `k24`
    pub dislikes: i32,
    /// Amount of stars you get for beating the level.
    ///
    /// Internal key: `k26`
    pub stars: i32,
    /// Internal key: `k66`
    pub requested_stars: Option<i32>,
    /// Internal key: `k27`
    pub feature_score: Option<i32>,
    /// See [`EpicRating`].
    ///
    /// Internal key: `k75`
    pub epic_rating: EpicRating,
    // PARSE: i32 -> enum
    // STORE: .to_num()
    /// Internal key: `k7`
    pub difficulty: GDLevelDifficulty,
    /// Internal key: `k25`
    pub is_demon: bool,
    /// See [`DemonType`].
    ///
    /// Internal key: `k76`
    pub demon_type: Option<DemonType>,
    /// Internal key: `k33`
    pub is_auto: bool,
}

#[derive(Debug, Clone, Default)]
/// Data about the coins in the level
pub struct GDLevelCoins {
    /// Internal key: `k64`
    pub total_coins: i32,
    /// For official levels.
    ///
    /// Internal key: `k37`
    pub required_coins: Option<i32>,
    /// Internal key: `k65`
    pub coins_verified: bool,
    /// Internal key: `k61`
    pub coin1_acquired: bool,
    /// Internal key: `k62`
    pub coin2_acquired: bool,
    /// Internal key: `k63`
    pub coin3_acquired: bool,
}

#[derive(Debug, Clone, Default)]
/// Variables that get accumulated due to player interaction with the level - attempts, jumps, best time, clicks, progress, etc.
pub struct GDLevelPlayerStats {
    /// Internal key: `k12`
    pub completions: i32,
    /// Internal key: `k18`
    pub attempts: i32,
    /// Internal key: `k19`
    pub normal_percentage: i32,
    /// Internal key: `k20`
    pub practice_percentage: i32,
    /// Internal key: `k36`
    pub jumps: i32,
    /// Clicks on the player's best attempt
    ///
    /// Internal key: `k85`
    pub clicks: i32,
    /// Amount of time spent on a player's best attempt
    ///
    /// Internal key: `k86`
    pub best_attempt_time: i32,
    /// Internal key: `k71`
    pub mana_orb_percentage: i32,
    /// Internal key: `k90`
    pub leaderboard_percentage: i32,
    /// Verification time in game ticks, where 1 game tick = 1s/240 = roughly 4.167ms
    ///
    /// Internal key: `k95`
    pub verification_time: Option<i32>,
    /// Internal key: `k107`
    pub best_time_ms: Option<i32>,
    /// Internal key: `k108`
    pub best_points: Option<i32>,
    // PARSE: split at every comma and convert to int
    // STORE: convert to string and join with commas
    /// Comma-separated values in ms.
    ///
    /// Internal key: `k109`
    pub local_best_times: Vec<i32>,
    // PARSE: split at every comma and convert to int
    // STORE: convert to string and join with commas
    /// Comma-separated scores.
    ///
    /// Internal key: `k110`
    pub local_best_points: Vec<i32>,
    // PARSE: split at every comma and convert to int
    // STORE: convert to string and join with commas
    /// Comma-separated high score diffs.
    ///
    /// Internal key: `k88`
    pub progress_diffs: Vec<i32>,
    /// Internal key: `k115`
    pub ticks_best_time: Option<i32>,
    /// Internal key: `k116`
    pub ticks_best_points: Option<i32>,
    /// Internal key: `k117`
    pub clicks_best_time: Option<i32>,
    /// Internal key: `k118`
    pub clicks_best_points: Option<i32>,
    // NOTE: has a similar (if not identical) structure to k34
    /// Internal key: `k119`
    pub replay_best_time: Option<Vec<u8>>,
    // NOTE: has a similar (if not identical) structure to k34
    /// Internal key: `k120`
    pub replay_best_points: Option<Vec<u8>>,
    /// Internal key: `k121`
    pub coins_best_time: Option<i32>,
    /// Internal key: `k122`
    pub coins_best_points: Option<i32>,
    /// Internal key: `k123`
    pub saved_best_time: bool,
    /// Internal key: `k124`
    pub saved_best_points: bool,
    /// Overrides the set "Click Between Steps" option for levels.
    /// Internal key: `k125`
    pub toggle_cbs: BooleanToggle,
}

// For maintainers - this struct contains most of the booleans in a level, and is where
// you'd put a boolean flag if it has nowhere else to be. In the case that it makes more sense
// for a flag to be declared elsewhere (e.g. coinX_acquired under GDLevelCoins), you
// should declare it there.
#[derive(Debug, Clone, Default)]
/// Various boolean flags for this level.
pub struct GDLevelFlags {
    /// Denotes if the level is editable; used in old version of the game, now unused.
    ///
    /// Internal key: `k13`
    pub is_editable: bool,
    /// Internal key: `k14`
    pub verified: bool,
    /// Internal key: `k15`
    pub uploaded: bool,
    /// Internal key: `k35`
    pub is_playable: bool,
    /// Used for official levels.
    ///
    /// Internal key: `k38`
    pub is_unlocked: bool,
    /// Internal key: `k79`
    pub unlisted: bool,
    /// If the level has been modified from outside the GD editor.
    ///
    /// Internal key: `k47`
    pub has_been_modified: bool,
    /// Internal key: `k72`
    pub has_ldm: bool,
    /// Internal key: `k73`
    pub ldm_enabled: bool,
    /// Internal key: `k77`
    pub is_gauntlet: bool,
    /// Whether the level was completed in one of the free games.
    ///
    /// Internal key: `k78`
    pub is_alt_game: bool,
    /// Internal key: `k82`
    pub is_favourited: bool,
    /// Internal key: `k93`
    pub unlimited_objects: bool,
    /// Internal key: `k112`
    pub no_shake: bool,
}

#[derive(Debug, Clone, Default)]
/// The state the level is in the editor. Contains information such as camera position and zoom, and currently viewed editor layer.
pub struct GDLevelEditorState {
    /// Internal key: `kI1`
    pub camera_x: f32,
    /// Internal key: `kI2`
    pub camera_y: f32,
    /// Internal key: `kI3`
    pub camera_zoom: f32,
    /// Internal key: `kI4`
    pub build_tab_page: i32,
    /// Internal key: `kI5`
    pub build_tab: i32,
    // PARSE: {a: b, c: d, ...} -> array[a] = b, array[c] = d, ...
    // STORE: enumerate arary elements and pack into: {idx: val, idx: val}
    // note: this array is usually 14 elements long. sometimes it is not. not known why.
    /// A list of indices for each editor tab that describes where the user is currently.
    /// For example, `build_tab_pages.get(&0)` is the page of the first tab that the user was last in.
    ///
    /// Internal key: `kI6`
    pub build_tab_pages: HashMap<i32, i32>,
    /// Internal key: `kI7`
    pub editor_layer: i32,
    /// Internal key: `k102`
    pub preview_lock_x: i32,
    /// Internal key: `k103`
    pub preview_lock_y: i32,
    /// Internal key: `k91`
    pub locked_layers: Vec<i32>,
}

#[derive(Debug, Clone, Default)]
/// Values GD uses to verify the level and the player's results on it
pub struct GDLevelIntegrity {
    // note: hashes are generated like this: <https://wyliemaster.github.io/gddocs/#/topics/encryption/chk?id=level-leaderboard>
    /// Internal key: `k87`
    pub level_seed: Option<i32>,
    /// Internal key: `k68`
    pub triggered_anti_cheat: bool,
    // UNKNOWN: used to verify level's completion but in some unknown way
    // presumably that true = good and false = bad
    /// Internal key: `k89`
    pub vfd_chk: bool,
    /// Internal key: `k111`
    pub platformer_seed: Option<i32>,
    /// Gzip-compressed replay data.
    ///
    /// Internal key: `k34`
    pub replay_data: Option<Vec<u8>>,
}

impl GDLevel {
    /// Creates a new instance of this object that is set with values necessary for the level to open in GD.
    ///
    /// Ensure that the game_version returns `Some(id)` when [`GDVersion::to_binary_version_id`] is called on it. The level may not open with its assigned data otherwise.
    pub fn new(game_version: GDVersion) -> Self {
        let mut this = GDLevel::default();
        this.identity.binary_version = game_version;
        this
    }

    /// Creates a new level for the latest version of GD ([`GDVersion::LATEST`]).
    pub fn new_latest_version() -> Self {
        Self::new(GDVersion::LATEST)
    }

    /// Creates a new GD level with level data.
    ///
    /// Ensure that the game_version returns `Some(id)` when [`GDVersion::to_binary_version_id`] is called on it. The level may not open with its assigned data otherwise.
    pub fn with_level_data(game_version: GDVersion, leveldata: GDLevelData) -> Self {
        let mut this = GDLevel::default();
        this.identity.binary_version = game_version;
        this.content.data = Some(GDLevelState::Decrypted(leveldata));
        this
    }

    /// Parses a .gmd file to a GDLevel. This function will fail if:
    /// 1. The file could not be read
    /// 2. The file is not valid XML
    /// 3. The level file itself is not a valid level.
    ///
    /// **NOTE:** Levels modified by Geode mods may be parsed **incorrectly** or may fail to be parsed at all!
    pub fn from_gmd<T: Into<PathBuf>>(path: T) -> Result<Self, GDError> {
        let file = proper_plist_tags(vec_as_str_lossy(&read(path.into())?))?;
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?;

        Ok(
            Self::from_dict(xmltree.as_dictionary().ok_or(GDError::CorruptedSavefile(
                "XML tree is not a dictionary".into(),
            ))?)
            .ok_or(GDError::CorruptedSavefile(
                "Unable to parse file to valid level.".into(),
            ))?,
        )
    }

    /// Parses a .gdlvl file to a GDLevel. This function will fail if:
    /// 1. The file could not be read
    /// 2. The file is not valid base64-encoded gzip data
    /// 3. The file is not valid XML
    /// 4. The level file itself is not a valid level.
    ///
    /// **NOTE:** Levels modified by Geode mods may be parsed **incorrectly** or may fail to be parsed at all!
    pub fn from_gdlvl<T: Into<PathBuf>>(path: T) -> Result<Self, GDError> {
        let file = proper_plist_tags(vec_as_str_lossy(&decompress(read(path.into())?)?))?;
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?;

        Ok(
            Self::from_dict(xmltree.as_dictionary().ok_or(GDError::CorruptedSavefile(
                "XML tree is not a dictionary".into(),
            ))?)
            .ok_or(GDError::CorruptedSavefile(
                "Unable to parse file to valid level.".into(),
            ))?,
        )
    }

    /// Exports the level to a .gmd file
    #[inline]
    pub fn export_to_gmd<T: Into<PathBuf>>(&self, path: T) -> Result<(), GDError> {
        write(path.into(), self.serialise_to_string())?;
        Ok(())
    }

    /// Exports the level to a .gdlvl file
    ///
    /// Every key is kept, unlike GD's own .gdlvl exports. For a payload without the keys GD strips, see [`Self::serialise_to_gdlvl_stripped_keys`].
    #[inline]
    pub fn export_to_gdlvl<T: Into<PathBuf>>(&self, path: T) -> Result<(), GDError> {
        write(
            path.into(),
            encrypt_level_str(&Self::format_to_gmd(&self.stringified())),
        )?;
        Ok(())
    }

    #[inline]
    /// Wraps a stringified level dictionary (such as the output of [`Self::stringified`]) in [`PLIST_HEADER`] and [`PLIST_FOOTER`], producing the contents of a .gmd file.
    pub fn format_to_gmd(data: &str) -> String {
        format!("{PLIST_HEADER}{data}{PLIST_FOOTER}")
    }

    #[inline]
    /// Formats a stringified level dictionary (such as the output of [`Self::stringified`]) as the contents of a .gdlvl file: [`Self::format_to_gmd`], then [`encrypt_level_str`].
    pub fn format_to_gdlvl(data: &str) -> String {
        vec_as_str(&encrypt_level_str(&Self::format_to_gmd(data)))
    }

    #[inline]
    /// Formats a payload of raw level data to .gdlvl format excluding all keys in [`GDLVL_NOT_EXPORTED_KEYS`].
    ///
    /// When exporting a level to .gdlvl, GD strips some keys. This function returns the level serialised the same way, ready to be written to a .gdlvl file.
    pub fn serialise_to_gdlvl_stripped_keys(&self) -> String {
        Self::format_to_gdlvl(&stringify_xml(&self.to_dict_gdlvl_keys(), true))
    }

    /// Serialize this object to a raw string. The returned string is in the format of a .gmd file and is not encrypted. This is the same format that is required for the level to be serialised as into the savefile.
    #[inline]
    pub fn serialise_to_string(&self) -> String {
        Self::format_to_gmd(&self.stringified())
    }

    /// Returns this level as stringified XML in GD's plist format.
    #[inline]
    pub fn stringified(&self) -> String {
        stringify_xml(&self.to_dict(), true)
    }

    /// Parse a raw level as a plist (such as the kind stored in a .gmd file) to a GDLevel. The given parameter must be **unencrypted.**
    pub fn from_plist_string<T: Into<String>>(plist: T) -> Result<Self, GDError> {
        let file = proper_plist_tags(plist.into())?;
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?;

        Ok(
            Self::from_dict(xmltree.as_dictionary().ok_or(GDError::CorruptedSavefile(
                "XML tree is not a dictionary".into(),
            ))?)
            .ok_or(GDError::CorruptedSavefile(
                "Unable to parse file to valid level.".into(),
            ))?,
        )
    }

    /// Generates the leaderboard seed for a classic level (property `k87` found at `GDLevel.integrity.level_seed`).
    ///
    /// Parameter info:
    /// * `has_played`: whether the player has played the level. Default: true
    /// * `seconds`: Presumably, the amount of seconds spent on the player's best attempt (found at `GDLevel.player_stats.best_attempt_time`).
    ///
    /// Note that an additional 1482 must be added to this value when submitting the level score.
    ///
    /// Reference: <https://boomlings.dev/resources/client/level>
    pub fn generate_level_seed_classic(&self, has_played: bool, seconds: i32) -> i32 {
        let temp1 = seconds + 4085;
        return 1482 * (has_played as i32)
            + (self.player_stats.jumps + 3991) * (self.player_stats.normal_percentage * 8354)
            + (temp1 * temp1)
            - 50028039;
    }

    /// Generates the leaderboard seed for a platformer level (property `k111`).
    ///
    /// Reference: <https://boomlings.dev/resources/client/level>
    pub fn generate_level_seed_platformer(&self) -> i32 {
        let number = (((self.player_stats.best_attempt_time + 7890) % 34567) * 601
            + ((self.player_stats.best_points.unwrap_or(0).abs() + 3456) % 78901) * 967
            + 94819)
            % 94433;
        return ((number ^ number >> 16) * 829) % 77849;
    }

    /// Parses a `plist::Dictionary` into a GDLevel object.
    ///
    /// This method is intended to convert all valid keys (that is, keys that are known and whose value is of its corresponding type).
    /// Known keys are parsed, while unknown keys are stored as-is.
    pub(crate) fn from_dict(d: &Dictionary) -> Option<Self> {
        let mut level = Self::default();
        for (k, v) in d {
            level.add_raw_key(k, v);
        }

        Some(level)
    }

    /// Adds a raw key to the level.
    ///
    /// The key must be formatted exactly as is found on a GD level - `kXX`, `kIX`, `kCEK`. The value must match the type of the property that is being set.
    /// The function will return `None` when the property's type mismatches or the value fails parsing.
    ///
    /// References: <https://boomlings.dev/resources/client/level>, <https://github.com/UHDanke/gmdkit/blob/main/src/gmdkit/casting/level_loader.py>
    pub fn add_raw_key(&mut self, k: &String, v: &Value) -> Option<()> {
        // keys come in a few shapes:
        // 1. kCEK: special kCEK key
        // 2. kXX: general keys
        // 3. kIX: some specific keys (no difference from kXX other than that you can't pase key[1..] to int)
        // 4. possibly unknown keys that do not necessarily follow this shape

        // structure:
        // - try parse key and value and set the corresponding field
        // - if the key (or its value) was not parsed successfully, try the next branch

        if k == "kCEK" {
            self.meta.kcek = KCEKValue::try_from(v.as_signed_integer().unwrap_or_default() as i32)
                .unwrap_or(KCEKValue::GJGameLevel);
            return Some(());
        }
        if k.starts_with("kI")
            && let Ok(key_id) = k[2..].parse::<i32>()
        {
            match key_id {
                1 => self.editor_state.camera_x = to_float(v).unwrap_or_default(),
                2 => self.editor_state.camera_y = to_float(v).unwrap_or_default(),
                3 => self.editor_state.camera_zoom = to_float(v).unwrap_or_default(),
                4 => {
                    self.editor_state.build_tab_page =
                        v.as_signed_integer().unwrap_or_default() as i32
                }
                5 => self.editor_state.build_tab = v.as_signed_integer().unwrap_or_default() as i32,
                6 => {
                    self.editor_state.build_tab_pages = v
                        .as_dictionary()
                        .unwrap_or(&Dictionary::default())
                        .iter()
                        .map(|(tab, page)| {
                            (
                                tab.parse::<i32>().ok().unwrap_or_default(),
                                page.as_string()
                                    .unwrap_or_default()
                                    .to_string()
                                    .parse::<i32>()
                                    .ok()
                                    .unwrap_or_default(),
                            )
                        })
                        .collect::<HashMap<_, _>>()
                }
                7 => {
                    self.editor_state.editor_layer =
                        v.as_signed_integer().unwrap_or_default() as i32
                }
                _ => {
                    self.unknowns.insert(k.clone(), v.clone());
                }
            }
            return Some(());
        }
        if k.starts_with("k")
            && let Ok(key_id) = k[1..].parse::<i32>()
        {
            match key_id {
                1 => self.identity.id = v.as_signed_integer()? as i32,
                2 => self.identity.name = v.as_string()?.to_string(),
                3 => {
                    // special case: b64-encoded data
                    self.identity.description =
                        Some(vec_as_str_lossy(&b64_decode(&v.as_string()?[..]).ok()?[..]))
                }
                4 => {
                    // special case: self data
                    let data = v.as_string()?.to_string();
                    self.content.data = Some(GDLevelState::Encrypted(GDEncryptedLevelData { data }))
                }
                5 => self.identity.creator = v.as_string()?.to_string(),
                6 => self.identity.user_id = v.as_signed_integer()? as i32,
                7 => {
                    self.ratings.difficulty = GDLevelDifficulty::from(v.as_signed_integer()? as i32)
                }
                8 => {
                    self.content.official_song_id =
                        Some(OfficialSong::from(v.as_signed_integer()? as i32))
                }
                9 => self.ratings.rating = DifficultyRating::from(v.as_signed_integer()? as i32),
                10 => self.ratings.rating_sum = v.as_signed_integer()? as i32,
                11 => self.ratings.downloads = v.as_signed_integer()? as i32,
                12 => self.player_stats.completions = v.as_signed_integer()? as i32,
                13 => self.flags.is_editable = v.as_boolean()?,
                14 => self.flags.verified = v.as_boolean()?,
                15 => self.flags.uploaded = v.as_boolean()?,
                16 => self.identity.level_version = v.as_signed_integer()? as i32,
                17 => self.identity.game_version = v.as_signed_integer()? as i32,
                18 => self.player_stats.attempts = v.as_signed_integer()? as i32,
                19 => self.player_stats.normal_percentage = v.as_signed_integer()? as i32,
                20 => self.player_stats.practice_percentage = v.as_signed_integer()? as i32,
                21 => {
                    // special case: GDselfType enum
                    self.identity.level_type = GDLevelType::try_from(v.as_signed_integer()? as i32)
                        .ok()
                        .unwrap_or_default()
                }
                22 => self.ratings.like_rating = v.as_signed_integer()? as i32,
                23 => self.content.length = Length::from(v.as_signed_integer()? as i32),
                24 => self.ratings.dislikes = v.as_signed_integer()? as i32,
                25 => self.ratings.is_demon = v.as_boolean()?,
                26 => self.ratings.stars = v.as_signed_integer()? as i32,
                27 => self.ratings.feature_score = Some(v.as_signed_integer()? as i32),
                /* 28 - 32 are unused */
                33 => self.ratings.is_auto = v.as_boolean()?,
                34 => self.integrity.replay_data = Some(v.as_string()?.as_bytes().to_vec()),
                35 => self.flags.is_playable = v.as_boolean()?,
                36 => self.player_stats.jumps = v.as_signed_integer()? as i32,
                37 => self.coins.required_coins = Some(v.as_signed_integer()? as i32),
                38 => self.flags.is_unlocked = v.as_boolean()?,
                39 => self.meta.level_size = v.as_signed_integer()? as i32,
                40 => self.identity.build_version = v.as_signed_integer()? as i32,
                41 => self.identity.password = Some(v.as_signed_integer()? as i32),
                42 => self.identity.original_id = Some(v.as_signed_integer()? as i32),
                43 => self.content.two_player_mode = v.as_boolean()?,
                /* 44 is unused */
                45 => self.content.custom_song_id = Some(v.as_signed_integer()? as i32),
                46 => self.identity.level_revision = Some(v.as_signed_integer()? as i32),
                47 => self.flags.has_been_modified = v.as_boolean()?,
                48 => self.content.object_count = v.as_signed_integer()? as i32,
                /* 49 is unused */
                50 => {
                    self.identity.binary_version =
                        GDVersion::from_binary_version(v.as_signed_integer()? as i32)
                }
                51 => self.meta.capacity_001 = v.as_signed_integer()? as i32,
                52 => self.meta.capacity_002 = v.as_signed_integer()? as i32,
                53 => self.meta.capacity_003 = v.as_signed_integer()? as i32,
                54 => self.meta.capacity_004 = v.as_signed_integer()? as i32,
                /* 55 - 59 as unused */
                60 => self.identity.account_id = v.as_signed_integer()? as i32,
                61 => self.coins.coin1_acquired = v.as_boolean()?,
                62 => self.coins.coin2_acquired = v.as_boolean()?,
                63 => self.coins.coin3_acquired = v.as_boolean()?,
                64 => self.coins.total_coins = v.as_signed_integer()? as i32,
                65 => self.coins.coins_verified = v.as_boolean()?,
                66 => self.ratings.requested_stars = Some(v.as_signed_integer()? as i32),
                67 => self.meta.capacity_string = Some(v.as_string()?.to_string()),
                68 => self.integrity.triggered_anti_cheat = v.as_boolean()?,
                69 => self.content.high_object_count = v.as_boolean()?,
                /* 70 is unused */
                71 => self.player_stats.mana_orb_percentage = v.as_signed_integer()? as i32,
                72 => self.flags.has_ldm = v.as_boolean()?,
                73 => self.flags.ldm_enabled = v.as_boolean()?,
                74 => self.identity.timely_id = v.as_signed_integer()? as i32,
                75 => {
                    // special case: EpicRating
                    self.ratings.epic_rating = EpicRating::try_from(v.as_signed_integer()? as i32)
                        .ok()
                        .unwrap()
                }
                76 => {
                    // special case: DemonType
                    self.ratings.demon_type = Some(
                        DemonType::try_from(v.as_signed_integer()? as i32)
                            .ok()
                            .unwrap(),
                    )
                }
                77 => self.flags.is_gauntlet = v.as_boolean()?,
                78 => self.flags.is_alt_game = v.as_boolean()?,
                79 => self.flags.unlisted = v.as_boolean()?,
                80 => self.meta.seconds_editing = v.as_signed_integer()? as i32,
                81 => self.meta.seconds_editing_copies = v.as_signed_integer()? as i32,
                82 => self.flags.is_favourited = v.as_boolean()?,
                83 => self.meta.level_order = v.as_signed_integer()? as i32,
                84 => self.meta.level_folder = v.as_signed_integer()? as i32,
                85 => self.player_stats.clicks = v.as_signed_integer()? as i32,
                86 => self.player_stats.best_attempt_time = v.as_signed_integer()? as i32,
                87 => self.integrity.level_seed = Some(v.as_signed_integer()? as i32),
                // special case: comma-separated list
                88 => self.player_stats.progress_diffs = parse_csv(v),
                89 => self.integrity.vfd_chk = v.as_boolean()?,
                90 => self.player_stats.leaderboard_percentage = v.as_signed_integer()? as i32,
                91 => self.editor_state.locked_layers = parse_csv(v),
                /* k101 - confirmed to exist but unknown */
                93 => self.flags.unlimited_objects = v.as_boolean()?,
                94 => self.content.is_platformer = v.as_boolean()?,
                95 => self.player_stats.verification_time = Some(v.as_signed_integer()? as i32),
                /* 96, 97 are unused */
                98 => self.identity.upload_date = Some(v.as_signed_integer()? as i32),
                99 => self.identity.update_date = Some(v.as_signed_integer()? as i32),
                /* k101 - confirmed to exist but unknown */
                102 => self.editor_state.preview_lock_x = v.as_signed_integer()? as i32,
                103 => self.editor_state.preview_lock_y = v.as_signed_integer()? as i32,
                // special case: comma-separated list
                104 => self.content.song_list = parse_csv(v),
                // special case: comma-separated list
                105 => self.content.sfx_list = Some(parse_csv(v)),
                /* k106 - confirmed to exist but unknown */
                107 => self.player_stats.best_time_ms = Some(v.as_signed_integer()? as i32),
                108 => self.player_stats.best_points = Some(v.as_signed_integer()? as i32),
                // special case: comma-separated list
                109 => self.player_stats.local_best_times = parse_csv(v),
                // special case: comma-separated list
                110 => self.player_stats.local_best_points = parse_csv(v),
                111 => self.integrity.platformer_seed = Some(v.as_signed_integer()? as i32),
                112 => self.flags.no_shake = v.as_boolean()?,
                115 => self.player_stats.ticks_best_time = Some(v.as_signed_integer()? as i32),
                116 => self.player_stats.ticks_best_points = Some(v.as_signed_integer()? as i32),
                117 => self.player_stats.clicks_best_time = Some(v.as_signed_integer()? as i32),
                118 => self.player_stats.clicks_best_points = Some(v.as_signed_integer()? as i32),
                119 => {
                    self.player_stats.replay_best_time = Some(v.as_string()?.as_bytes().to_vec())
                }
                120 => {
                    self.player_stats.replay_best_points = Some(v.as_string()?.as_bytes().to_vec())
                }
                121 => self.player_stats.coins_best_time = Some(v.as_signed_integer()? as i32),
                122 => self.player_stats.coins_best_points = Some(v.as_signed_integer()? as i32),
                123 => self.player_stats.saved_best_time = v.as_boolean()?,
                124 => self.player_stats.saved_best_points = v.as_boolean()?,
                125 => {
                    self.player_stats.toggle_cbs =
                        BooleanToggle::from(v.as_signed_integer()? as i32)
                }
                126 => self.content.autobackup_id = Some(v.as_string()?.to_string()),
                127 => self.content.last_autobackup_time = v.as_signed_integer()? as i32,
                128 => self.content.platformer_save = Some(v.as_string()?.to_string()),
                129 => self.content.enable_platformer_save = v.as_boolean()?,
                130 => self.content.platformer_save_level_version = v.as_signed_integer()? as i32,
                131 => {
                    self.content.platformer_save_replay = Some(v.as_string()?.as_bytes().to_vec())
                }
                132 => self.content.platformer_save_format_version = v.as_signed_integer()? as i32,
                _ => {
                    self.unknowns.insert(k.clone(), v.clone());
                }
            }
            return Some(());
        }
        self.unknowns.insert(k.clone(), v.clone());

        Some(())
    }

    /// Returns this object as a `plist::Dictionary`
    pub(crate) fn to_dict(&self) -> Dictionary {
        /* Field types (kXX)
         * i32: 1, 6, 7, 9, 10, 11, 12, 16, 17, 18, 19, 20, 22, 23, 24, 26, 36, 39, 40, 48, 50, 51, 52, 53, 54, 60, 64, 71, 74, 80, 81, 83, 84, 85, 86, 90
         * i32 (optional): 8, 27, 37, 41, 42, 45, 46, 66, 87, 92, 95, 106, 107, 108, 111
         * string: 2, 5
         * string (optional): 67, 91, 101
         * bool: 13, 14, 15, 25, 33, 35, 38, 43, 47, 61, 62, 63, 65, 68, 69, 72, 73, 77, 78, 79, 82, 89, 93, 94, 112
         * comma separated list: 88, 104, 109, 110
         *
         * special cases:
         * b64 string: 3
         * level data: 4
         * level type: 21
         * Vec<u8>: 34
         * epic rating: 75
         * demon type: 76
         * optional comma separated list: 105
         * KCEKValue: kCEK
         * dict of 14 ints: kI6
         *
         * kIX keys:
         * floats: 1, 2, 3, 7
         * ints: 4, 5
         */

        let mut d = Dictionary::new();

        /* common cases */
        // ints
        serialise_fields(
            &mut d,
            &[
                ("k1", self.identity.id),
                ("k6", self.identity.user_id),
                ("k7", self.ratings.difficulty.to_num()),
                ("k9", self.ratings.rating.to_num()),
                ("k10", self.ratings.rating_sum),
                ("k11", self.ratings.downloads),
                ("k12", self.player_stats.completions),
                ("k16", self.identity.level_version),
                ("k17", self.identity.game_version),
                ("k18", self.player_stats.attempts),
                ("k19", self.player_stats.normal_percentage),
                ("k20", self.player_stats.practice_percentage),
                ("k21", self.identity.level_type.to_num()),
                ("k22", self.ratings.like_rating),
                ("k23", self.content.length.to_num()),
                ("k24", self.ratings.dislikes),
                ("k26", self.ratings.stars),
                ("k36", self.player_stats.jumps),
                ("k39", self.meta.level_size),
                ("k40", self.identity.build_version),
                ("k48", self.content.object_count),
                ("k51", self.meta.capacity_001),
                ("k52", self.meta.capacity_002),
                ("k53", self.meta.capacity_003),
                ("k54", self.meta.capacity_004),
                ("k60", self.identity.account_id),
                ("k64", self.coins.total_coins),
                ("k71", self.player_stats.mana_orb_percentage),
                ("k74", self.identity.timely_id),
                ("k75", self.ratings.epic_rating.to_num()),
                ("k80", self.meta.seconds_editing),
                ("k81", self.meta.seconds_editing_copies),
                ("k83", self.meta.level_order),
                ("k84", self.meta.level_folder),
                ("k85", self.player_stats.clicks),
                ("k86", self.player_stats.best_attempt_time),
                ("k90", self.player_stats.leaderboard_percentage),
                ("k102", self.editor_state.preview_lock_x),
                ("k103", self.editor_state.preview_lock_y),
                ("k127", self.content.last_autobackup_time),
                ("k130", self.content.platformer_save_level_version),
                ("k132", self.content.platformer_save_format_version),
                ("kI4", self.editor_state.build_tab_page),
                ("kI5", self.editor_state.build_tab),
                ("kI7", self.editor_state.editor_layer),
            ],
        );

        // optional i32
        serialise_optional_fields(
            &mut d,
            &[
                ("k8", self.content.official_song_id.map(|s| s.to_num())),
                ("k27", self.ratings.feature_score),
                ("k37", self.coins.required_coins),
                ("k41", self.identity.password),
                ("k42", self.identity.original_id),
                ("k45", self.content.custom_song_id),
                ("k46", self.identity.level_revision),
                ("k66", self.ratings.requested_stars),
                ("k87", self.integrity.level_seed),
                ("k98", self.identity.upload_date),
                ("k99", self.identity.update_date),
                ("k107", self.player_stats.best_time_ms),
                ("k108", self.player_stats.best_points),
                ("k111", self.integrity.platformer_seed),
                ("k115", self.player_stats.ticks_best_time),
                ("k116", self.player_stats.ticks_best_points),
                ("k117", self.player_stats.clicks_best_time),
                ("k118", self.player_stats.clicks_best_points),
                ("k121", self.player_stats.coins_best_time),
                ("k122", self.player_stats.coins_best_points),
            ],
        );

        // strings (int lists)
        serialise_fields(
            &mut d,
            &[
                ("k2", &self.identity.name[..]),
                ("k5", &self.identity.creator[..]),
                // lump in the comma separated lists in with this one because they are internally strings
                ("k88", to_csv(&self.player_stats.progress_diffs).as_str()),
                ("k91", to_csv(&self.editor_state.locked_layers).as_str()),
                ("k104", to_csv(&self.content.song_list).as_str()),
                ("k109", to_csv(&self.player_stats.local_best_times).as_str()),
                (
                    "k110",
                    to_csv(&self.player_stats.local_best_points).as_str(),
                ),
            ],
        );

        // option<string>
        serialise_optional_fields(
            &mut d,
            &[
                ("k67", self.meta.capacity_string.clone()),
                ("k126", self.content.autobackup_id.clone()),
                ("k128", self.content.platformer_save.clone()),
            ],
        );

        // bools
        serialise_bool_fields(
            &mut d,
            &[
                ("k13", self.flags.is_editable),
                ("k14", self.flags.verified),
                ("k15", self.flags.uploaded),
                ("k25", self.ratings.is_demon),
                ("k33", self.ratings.is_auto),
                ("k35", self.flags.is_playable),
                ("k38", self.flags.is_unlocked),
                ("k43", self.content.two_player_mode),
                ("k47", self.flags.has_been_modified),
                ("k61", self.coins.coin1_acquired),
                ("k62", self.coins.coin2_acquired),
                ("k63", self.coins.coin3_acquired),
                ("k65", self.coins.coins_verified),
                ("k68", self.integrity.triggered_anti_cheat),
                ("k69", self.content.high_object_count),
                ("k72", self.flags.has_ldm),
                ("k73", self.flags.ldm_enabled),
                ("k77", self.flags.is_gauntlet),
                ("k78", self.flags.is_alt_game),
                ("k79", self.flags.unlisted),
                ("k82", self.flags.is_favourited),
                ("k89", self.integrity.vfd_chk),
                ("k93", self.flags.unlimited_objects),
                ("k94", self.content.is_platformer),
                ("k112", self.flags.no_shake),
                ("k123", self.player_stats.saved_best_time),
                ("k124", self.player_stats.saved_best_points),
                ("k129", self.content.enable_platformer_save),
            ],
        );

        // floats
        serialise_fields(
            &mut d,
            &[
                ("kI1", self.editor_state.camera_x),
                ("kI2", self.editor_state.camera_y),
                ("kI3", self.editor_state.camera_zoom),
            ],
        );

        d.insert("kCEK".into(), Value::from(self.meta.kcek.to_num()));
        d.insert(
            "k50".into(),
            Value::from(
                self.identity
                    .binary_version
                    .to_binary_version_id()
                    .unwrap_or_default(),
            ),
        );

        if let Some(dtype) = self.ratings.demon_type {
            d.insert("k76".into(), Value::from(dtype.to_num()));
        }

        if let Some(ref desc) = self.identity.description {
            d.insert("k3".into(), Value::String(b64_encode(desc.as_bytes())));
        };

        if let Some(v) = self.content.data.clone() {
            let str = match v {
                GDLevelState::Decrypted(data) => data.serialise_to_string(),
                GDLevelState::Encrypted(data) => data.data,
            };
            d.insert("k4".into(), Value::from(str));
        };

        // Option<Vec<u8>>
        serialise_fields_by(
            &mut d,
            &[
                ("k34", &self.integrity.replay_data),
                ("k119", &self.player_stats.replay_best_time),
                ("k120", &self.player_stats.replay_best_points),
                ("k131", &self.content.platformer_save_replay),
            ],
            |v| {
                if let Some(str) = v {
                    Some(Value::String(vec_as_str(&str[..])))
                } else {
                    None
                }
            },
        );

        d.insert(
            "k125".into(),
            Value::from(self.player_stats.toggle_cbs.to_num()),
        );

        if let Some(ref csv) = self.content.sfx_list {
            d.insert(
                "k105".into(),
                Value::String(
                    csv.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            );
        }

        d.insert(
            "kI6".into(),
            Value::Dictionary(Dictionary::from_iter(
                self.editor_state
                    .build_tab_pages
                    .iter()
                    .map(|(tab_idx, page)| (tab_idx.to_string(), page.to_string())),
            )),
        );

        d.extend(self.unknowns.clone());

        d
    }

    /// Returns this object as a `plist::Dictionary`, without the keys in [`GDLVL_NOT_EXPORTED_KEYS`].
    pub(crate) fn to_dict_gdlvl_keys(&self) -> Dictionary {
        let mut d = self.to_dict();
        d.retain(|k, _| {
            !k.strip_prefix('k')
                .and_then(|id| id.parse::<u16>().ok())
                .is_some_and(|id| GDLVL_NOT_EXPORTED_KEYS.contains(&id))
        });
        d
    }

    /// Decrypts level data if the payload in `self.content.data` is valid, decryptable level data.
    ///
    /// When parsing a level from `CCLocalLevels::from_local`, `GDLevel::from_gmd`, `GDLevel::from_gdlvl`,
    /// or any other methods that handle reading a GD level, the level data itself is not automatically
    /// decrypted for the sake of not wasting time on potentially unwanted decryptions (i.e. not decrypting
    /// the entire savefile when all you need is one level). For this reason, you need to call this method
    /// on any parsed level to fetch its data, otherwise you will not read anything meaningful.
    ///
    /// **TL;DR**: Call this method on `GDLevel`s after using `CCLocalLevels::from_local`, `GDLevel::from_gmd`
    /// or `GDLevel::from_gdlvl` (or any other method in this crate that parses GD levels).
    pub fn decrypt_level_data(&mut self) -> Result<(), GDError> {
        let raw_data = if let Some(data) = &self.content.data
            && let GDLevelState::Encrypted(enc) = data
        {
            enc.data.as_str()
        } else {
            return Ok(());
        };

        self.content.data = Some(GDLevelState::Decrypted(
            leveldata::GDLevelData::parse(raw_data).ok_or(GDError::CorruptedSavefile(
                "Unable to parse level header".into(),
            ))?,
        ));
        Ok(())
    }

    /// Returns a copy of the decrypted level data if there is data, decrypting a copy if needed. `self` is not modified.
    ///
    /// Returns `None` if there is no level data or it cannot be decrypted.
    pub fn get_decrypted_data(&self) -> Option<GDLevelData> {
        match self.content.data.clone() {
            Some(data) => match data {
                GDLevelState::Encrypted(encrypted) => GDLevelData::parse(&encrypted.data),
                GDLevelState::Decrypted(d) => Some(d), // already decrypted
            },
            None => None, // no level data
        }
    }

    /// Returns a copy of the raw decrypted data. `self.content.data` must be `GDLevelState::Encrypted`.
    pub fn get_raw_decrypted_data(&self) -> Option<String> {
        match self.content.data.clone() {
            Some(data) => match data {
                GDLevelState::Encrypted(ref encrypted) => {
                    // parse level data
                    let raw_data = decompress(encrypted.data.as_bytes().to_vec()).ok()?;
                    let decrypted = std::str::from_utf8(&raw_data[..]).ok()?;
                    Some(decrypted.to_owned())
                }
                GDLevelState::Decrypted(_) => None, // already decrypted
            },
            None => None, // no level data
        }
    }

    /// Returns a mutable reference to the decrypted level data if there is data.
    /// This method calls [`Self::decrypt_level_data`] before attempting to return the decrypted level data, so the decrypted data is kept in `self`.
    ///
    /// Returns `None` if there is no level data or it cannot be decrypted.
    pub fn get_decrypted_data_ref(&mut self) -> Option<&mut GDLevelData> {
        self.decrypt_level_data().ok()?;
        match &mut self.content.data {
            Some(d) => {
                if let GDLevelState::Decrypted(data) = d {
                    Some(data)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Adds a `GDObject` to `self.content.data` only if self.content.data is already decrypted or empty, otherwise nothing happens.
    /// To decrypt the level data, see [`Self::decrypt_level_data`] or [`Self::get_decrypted_data_ref`].
    ///
    /// If `self.content.data` is `Some(GDLevelState::Decrypted)` it's `objects` field will be mutated.
    /// If `self.content.data` is `None`, a new object will be instantiated via [`default_level_data`] and this method is called on the new level data with the same parameter.
    pub fn add_object(&mut self, object: GDObject) {
        if let Some(data) = &mut self.content.data {
            match data {
                GDLevelState::Decrypted(state) => {
                    state.objects.push(object);
                }
                GDLevelState::Encrypted(_) => {}
            }
        } else {
            self.content.data = default_level_data();
            self.add_object(object);
        }
    }

    /// Adds an iterator of GDObjects to `self.content.data` only if self.content.data is already decrypted or empty, otherwise nothing happens.
    /// To decrypt the level data, see [`Self::decrypt_level_data`] or [`Self::get_decrypted_data_ref`].
    ///
    /// If `self.content.data` is `Some(GDLevelState::Decrypted)` it's `objects` field will be mutated.
    /// If `self.content.data` is `None`, a new object will be instantiated via [`default_level_data`] and this method is called on the new level data with the same parameter.
    pub fn add_objects<I: IntoIterator<Item = GDObject>>(&mut self, objects: I) {
        if let Some(data) = &mut self.content.data {
            match data {
                GDLevelState::Decrypted(state) => {
                    state.objects.extend(objects.into_iter());
                }
                GDLevelState::Encrypted(_) => (),
            };
        } else {
            self.content.data = default_level_data();
            self.add_objects(objects);
        }
    }

    /// Returns a reference to self.content.data.objects if this Level has level data and if the data is decrypted.
    pub fn get_objects_ref(&self) -> Option<&Vec<GDObject>> {
        if let Some(ref data) = self.content.data {
            match data {
                GDLevelState::Decrypted(state) => Some(&state.objects),
                GDLevelState::Encrypted(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns a mutable reference to self.content.data.objects if this Level has level data and if the data is decrypted.
    pub fn get_objects_mut(&mut self) -> Option<&mut Vec<GDObject>> {
        if let Some(data) = &mut self.content.data {
            match data {
                GDLevelState::Decrypted(state) => Some(&mut state.objects),
                GDLevelState::Encrypted(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns a copy of self.content.data.objects if this Level has level data and if the data is decrypted.
    pub fn get_objects_clone(&self) -> Option<Vec<GDObject>> {
        if let Some(ref data) = self.content.data {
            match data {
                GDLevelState::Decrypted(state) => Some(state.objects.clone()),
                GDLevelState::Encrypted(_) => None,
            }
        } else {
            None
        }
    }
}

impl Display for GDLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (num, suffix) = {
            match &self.content.data {
                Some(d) => match d {
                    GDLevelState::Encrypted(enc) => (enc.data.len().to_string(), " Bytes"),
                    GDLevelState::Decrypted(dec) => (dec.objects.len().to_string(), " Objects"),
                },
                None => ("Empty".to_owned(), ""),
            }
        };

        write!(
            f,
            "\"{}\" ({}) by {}; {num}{suffix}",
            self.identity.name,
            self.identity
                .description
                .clone()
                .unwrap_or("<No desciption>".to_string()),
            self.identity.creator,
        )
    }
}

impl Default for GDLevel {
    fn default() -> Self {
        Self {
            identity: GDLevelIdentity::default(),
            content: GDLevelContents::default(),
            ratings: GDLevelRatings::default(),
            coins: GDLevelCoins::default(),
            player_stats: GDLevelPlayerStats::default(),
            flags: GDLevelFlags {
                is_editable: true,
                ..Default::default()
            },
            editor_state: GDLevelEditorState {
                camera_x: 100.0,
                camera_y: 100.0,
                camera_zoom: 1.0,
                ..Default::default()
            },
            meta: GDLevelMeta::default(),
            integrity: GDLevelIntegrity::default(),
            unknowns: HashMap::default(),
        }
    }
}

/// `fields`: &[id, field]
///
/// Fields equal to `T::default()` are not written.
pub(crate) fn serialise_fields<T: Default + Clone + PartialEq>(
    d: &mut Dictionary,
    fields: &[(&str, T)],
) where
    Value: From<T>,
{
    let default = &T::default();
    for (id, value) in fields {
        if value != default {
            d.insert(id.to_string(), Value::from(value.clone()));
        }
    }
}

pub(crate) fn serialise_fields_by<T, U>(d: &mut Dictionary, fields: &[(&str, T)], f: U)
where
    U: Fn(&T) -> Option<Value>,
{
    for (id, value) in fields {
        if let Some(v) = f(value) {
            d.insert(id.to_string(), v);
        }
    }
}

/// `fields`: &[id, field]
///
/// `None` fields are not written.
pub(crate) fn serialise_optional_fields<T: Clone>(d: &mut Dictionary, fields: &[(&str, Option<T>)])
where
    Value: From<T>,
{
    for (id, value) in fields {
        if let Some(v) = value {
            d.insert(id.to_string(), Value::from(v.clone()));
        }
    }
}

/// `fields`: &[id, field]
///
/// booleans are handled separately for speed (specialization case). Only `true` fields are written.
pub(crate) fn serialise_bool_fields(d: &mut Dictionary, fields: &[(&str, bool)]) {
    for (id, value) in fields {
        if *value {
            d.insert(id.to_string(), Value::Boolean(true));
        }
    }
}

fn to_float(v: &Value) -> Option<f32> {
    if let Some(f) = v.as_real() {
        Some(f as f32)
    } else if let Some(i) = v.as_signed_integer() {
        Some(i as f32)
    } else {
        None
    }
}

/// replaces unparsable values with 0
pub(crate) fn parse_csv(v: &Value) -> Vec<i32> {
    v.as_string()
        .unwrap_or_default()
        .split(",")
        .map(|d| d.parse::<i32>().unwrap_or_default())
        .collect::<Vec<i32>>()
}

pub(crate) fn to_csv<T: ToString>(v: &[T]) -> String {
    v.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
