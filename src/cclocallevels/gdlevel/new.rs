//! Provisional new replacement struct for Self
#![allow(missing_docs)]
// ^ temp
use std::{collections::HashMap, fmt::Display, fs, io::Cursor, path::PathBuf};

use plist::{Dictionary, Value};

use crate::{
    cclocallevels::{
        gdlevel::{
            PLIST_FOOTER, PLIST_HEADER,
            leveldata::{EncryptedLevelData, LevelData, LevelState},
        },
        gdobj::GDObject,
    },
    core::{
        GDError, b64_decode, io::stringify_xml, proper_plist_tags, structs::KCEKValue, vec_as_str,
    },
};

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
/// Reference: <https://wyliemaster.github.io/gddocs/#/resources/client/level>
pub struct GDLevel {
    /// Identity info: ID, name, descriptior, creator, version, level type, password
    pub identity: GDLevelIdentity,
    /// The level's data: object data, song list, used song ID, length, is platformer/2-player
    pub content: GDLevelData,
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
    /// Unaccounted for/unknown keys: k91, k92, k101, k106 and all other keys that didn't fit in any other fields in this struct.
    pub unknowns: GDLevelUnknowns,
}

#[derive(Debug, Clone, Default)]
/// Identity of the level and data surrounding it. Notably, the level name, id, and creator (among others).
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
    /// Hardcoded value. LLM_02 is also hardcoded to this value.
    ///
    /// Internal key: `k50`
    pub binary_version: i32,
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
}

#[derive(Debug, Clone, Default)]
/// Values to do with the level data / gameplay. Notably, the level data itself, object count, length, etc.
pub struct GDLevelData {
    /// The data of the level. See [`LevelState`]
    /// Internal key: `k4`
    pub data: Option<LevelState>,
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
    pub official_song_id: Option<i32>,
    /// Internal key: `k45`
    pub custom_song_id: Option<i32>,
    // UNCLEAR: assuming that this means the level's length in gameticks
    /// Internal key: `k23`
    pub length: i32,
    /// Internal key: `k43`
    pub two_player_mode: bool,
    /// Internal key: `k94`
    pub is_platformer: bool,
}

#[derive(Debug, Clone)]
/// Miscellaneous metadata surrounding the level. Mostly internal or organizational fields.
pub struct GDLevelMeta {
    // PARSE: i32 -> kcekvalue
    // STORE: kcekvalue as i32
    /// This value is always [`KCEKValue::GJGameLevel`].
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
pub struct GDLevelRatings {
    // UNCLEAR
    /// Internal key: `k9`
    pub rating: i32,
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
    /// Internal key: `k7`
    pub difficulty: i32,
    /// Internal key: `k25`
    pub is_demon: bool,
    // todo
    // note: add demon type enum (not sure what that is)
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
/// Player's statistics on this level, notably, attempts.
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
    /// Internal key: `k86`
    pub best_attempt_time: i32,
    /// Internal key: `k71`
    pub mana_orb_percentage: i32,
    /// Internal key: `k90`
    pub leaderboard_percentage: i32,
    /// Verification time in game ticks, where 1 game tick = 1s/240 = roughly 4.167ms
    /// Internal key: `k95`
    pub verification_time: Option<i32>,
    /// Internal key: `k107`
    pub best_time_ms: Option<i32>,
    /// Internal key: `k108`
    pub best_points: Option<i32>,
    // PARSE: split at every comma and convert to int
    // STORE: convert to string and join with commas
    // TODO: parse
    /// Comma-separated values in ms.
    ///
    /// Internal key: `k109`
    pub local_best_times: Vec<i32>,
    // PARSE: split at every comma and convert to int
    // STORE: convert to string and join with commas
    // TODO: parse to Vec
    /// Comma-separated scores.
    ///
    /// Internal key: `k110`
    pub local_best_points: Vec<i32>,
    // PARSE: split at every comma and convert to int
    // STORE: convert to string and join with commas
    // TODO: parse to Vec
    /// Comma-separated high score diffs.
    ///
    /// Internal key: `k88`
    pub progress_diffs: Vec<i32>,
}

#[derive(Debug, Clone, Default)]
/// Various boolean flags for this level
pub struct GDLevelFlags {
    // UNUSED
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
/// Editor state variables for this level
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
    /// A list of indices for each editor tab that describes where the user is currently.
    /// For example, build_tab_pages[0] is the page of the first tab that the user was last in.
    ///
    /// Internal key: `kI6`
    pub build_tab_pages: [i32; 14],
    /// Internal key: `kI7`
    pub editor_layer: f32,
}

#[derive(Debug, Clone, Default)]
/// Variables that have to do with checking the integrity of the level's integrity
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

#[derive(Debug, Clone, Default)]
/// Keys that are undocumented or have no known purpose.
pub struct GDLevelUnknowns {
    pub k91: Option<String>, // unknown string
    pub k92: Option<i32>,    // unknown integer
    /// Observed as "0,0,...,0" (20 zeros); seems to correlate with k47 (has_been_modified)
    pub k101: Option<String>,
    pub k106: Option<i32>, // corresponds to key 54 on the servers
    /// Properties which are unaccounted for in the struct but are present in the level nonetheless.
    /// Values here are not mutated at all, and are instead stored as they were found in the level dictionary.
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpicRating {
    #[default]
    None = 0,
    Epic = 1,
    Legendary = 2,
    Mythic = 3,
}

impl TryFrom<i32> for EpicRating {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Epic),
            2 => Ok(Self::Legendary),
            3 => Ok(Self::Mythic),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GDLevelType {
    Official = 1,
    #[default]
    Local = 2,
    Saved = 3,
    Online = 4,
}

impl TryFrom<i32> for GDLevelType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Official),
            2 => Ok(Self::Local),
            3 => Ok(Self::Saved),
            4 => Ok(Self::Online),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DemonType {
    EasyDemon = 1,
    MediumDemon = 2,
    HardDemon = 3,
    InsaneDemon = 4,
    ExtremeDemon = 5,
}

impl TryFrom<i32> for DemonType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::EasyDemon),
            2 => Ok(Self::MediumDemon),
            3 => Ok(Self::HardDemon),
            4 => Ok(Self::InsaneDemon),
            5 => Ok(Self::ExtremeDemon),
            _ => Err(()),
        }
    }
}

impl GDLevel {
    /// Parses a .gmd file to a `Self` object
    pub fn from_gmd<T: Into<PathBuf>>(path: T) -> Result<Self, GDError> {
        let file = proper_plist_tags(vec_as_str(&fs::read(path.into())?));
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?;

        Ok(
            Self::from_dict(xmltree.as_dictionary().unwrap()).ok_or(GDError::CorruptedSavefile(
                "Unable to parse file to valid level.".into(),
            ))?,
        )
    }

    // /// Exports the level to a .gmd file
    // pub fn export_to_gmd<T: Into<PathBuf>>(&self, path: T) -> Result<(), GDError> {
    //     let export_str = format!(
    //         "{PLIST_HEADER}{}{PLIST_FOOTER}",
    //         stringify_xml(&self.to_dict(), true)
    //     );

    //     fs::write(path.into(), export_str)?;
    //     Ok(())
    // }

    /// Parses a `plist::Dictionary` into a GDLevel object.
    ///
    /// This method is intended to convert all valid keys (that is, keys that are known and whose value is of its corresponding type).
    /// Known keys are parsed, while unknown keys are stored as-is.
    ///
    /// This function panics if any of the documented keys have a value which does not fit its expected type. For instance, if the attempt count, which is expected to be an integer is instead found to be a string/float/array/etc.
    pub(crate) fn from_dict(d: &Dictionary) -> Option<Self> {
        let mut level = Self::default();
        for (k, v) in d {
            // keys come in a few shapes:
            // 1. kCEK: special kCEK key
            // 2. kXX: general keys
            // 3. kIX: some specific keys (no difference from kXX other than that you can't pase key[1..] to int)
            // 4. possibly unknown keys that do not necessarily follow this shape

            // structure:
            // - try parse key and value and set the corresponding field
            // - if the key (or its value) was not parsed successfully, try the next branch

            if k == "kCEK" {
                if let Ok(k) = KCEKValue::try_from(v.as_signed_integer()? as i32) {
                    level.meta.kcek = k;
                    continue;
                }
            }
            if k.starts_with("kI")
                && let Ok(key_id) = k[2..].parse::<i32>()
            {
                match key_id {
                    1 => level.editor_state.camera_x = v.as_real()? as f32,
                    2 => level.editor_state.camera_y = v.as_real()? as f32,
                    3 => level.editor_state.camera_zoom = v.as_real()? as f32,
                    4 => level.editor_state.build_tab_page = v.as_signed_integer()? as i32,
                    5 => level.editor_state.build_tab = v.as_signed_integer()? as i32,
                    6 => {
                        level.editor_state.build_tab_pages = {
                            let mut pages = [0; 14];
                            for (tab, page) in v.as_dictionary()? {
                                let tab_idx = tab.parse::<usize>().unwrap();
                                let page_idx = page.as_signed_integer().unwrap() as i32;
                                if let Some(x) = pages.get_mut(tab_idx) {
                                    *x = page_idx
                                }
                            }
                            pages
                        }
                    }

                    _ => {
                        level.unknowns.other.insert(k.clone(), v.clone());
                    }
                }
                continue;
            }
            if k.starts_with("k")
                && let Ok(key_id) = k[2..].parse::<i32>()
            {
                match key_id {
                    1 => level.identity.id = v.as_signed_integer()? as i32,
                    2 => level.identity.name = v.as_string()?.to_string(),
                    3 => {
                        level.identity.description =
                            Some(vec_as_str(&b64_decode(&v.as_string()?[..])[..]))
                    }
                    4 => {
                        // special case: level data
                        let data = v.as_string()?.to_string();
                        level.content.data =
                            Some(LevelState::Encrypted(EncryptedLevelData { data }))
                    }
                    5 => level.identity.creator = v.as_string()?.to_string(),
                    6 => level.identity.user_id = v.as_signed_integer()? as i32,
                    7 => level.ratings.difficulty = v.as_signed_integer()? as i32,
                    8 => level.content.official_song_id = Some(v.as_signed_integer()? as i32),
                    9 => level.ratings.rating = v.as_signed_integer()? as i32,
                    10 => level.ratings.rating_sum = v.as_signed_integer()? as i32,
                    11 => level.ratings.downloads = v.as_signed_integer()? as i32,
                    12 => level.player_stats.completions = v.as_signed_integer()? as i32,
                    13 => level.flags.is_editable = v.as_boolean()?,
                    14 => level.flags.verified = v.as_boolean()?,
                    15 => level.flags.uploaded = v.as_boolean()?,
                    16 => level.identity.level_version = v.as_signed_integer()? as i32,
                    17 => level.identity.game_version = v.as_signed_integer()? as i32,
                    18 => level.player_stats.attempts = v.as_signed_integer()? as i32,
                    19 => level.player_stats.normal_percentage = v.as_signed_integer()? as i32,
                    20 => level.player_stats.practice_percentage = v.as_signed_integer()? as i32,
                    21 => {
                        // special case: GDLevelType enum
                        level.identity.level_type =
                            GDLevelType::try_from(v.as_signed_integer()? as i32).ok()?
                    }
                    22 => level.ratings.like_rating = v.as_signed_integer()? as i32,
                    23 => level.content.length = v.as_signed_integer()? as i32,
                    24 => level.ratings.dislikes = v.as_signed_integer()? as i32,
                    25 => level.ratings.is_demon = v.as_boolean()?,
                    26 => level.ratings.stars = v.as_signed_integer()? as i32,
                    27 => level.ratings.feature_score = Some(v.as_signed_integer()? as i32),
                    /* 28 - 32 are unused */
                    33 => level.ratings.is_auto = v.as_boolean()?,
                    34 => level.integrity.replay_data = Some(v.as_string()?.as_bytes().to_vec()),
                    35 => level.flags.is_playable = v.as_boolean()?,
                    36 => level.player_stats.jumps = v.as_signed_integer()? as i32,
                    37 => level.coins.required_coins = Some(v.as_signed_integer()? as i32),
                    38 => level.flags.is_unlocked = v.as_boolean()?,
                    39 => level.meta.level_size = v.as_signed_integer()? as i32,
                    40 => level.identity.build_version = v.as_signed_integer()? as i32,
                    41 => level.identity.password = Some(v.as_signed_integer()? as i32),
                    42 => level.identity.original_id = Some(v.as_signed_integer()? as i32),
                    43 => level.content.two_player_mode = v.as_boolean()?,
                    /* 44 is unused */
                    45 => level.content.custom_song_id = Some(v.as_signed_integer()? as i32),
                    46 => level.identity.level_revision = Some(v.as_signed_integer()? as i32),
                    47 => level.flags.has_been_modified = v.as_boolean()?,
                    48 => level.content.object_count = v.as_signed_integer()? as i32,
                    /* 49 is unused */
                    50 => level.identity.binary_version = v.as_signed_integer()? as i32,
                    51 => level.meta.capacity_001 = v.as_signed_integer()? as i32,
                    52 => level.meta.capacity_002 = v.as_signed_integer()? as i32,
                    53 => level.meta.capacity_003 = v.as_signed_integer()? as i32,
                    54 => level.meta.capacity_004 = v.as_signed_integer()? as i32,
                    /* 55 - 59 as unused */
                    60 => level.identity.account_id = v.as_signed_integer()? as i32,
                    61 => level.coins.coin1_acquired = v.as_boolean()?,
                    62 => level.coins.coin2_acquired = v.as_boolean()?,
                    63 => level.coins.coin3_acquired = v.as_boolean()?,
                    64 => level.coins.total_coins = v.as_signed_integer()? as i32,
                    65 => level.coins.coins_verified = v.as_boolean()?,
                    66 => level.ratings.requested_stars = Some(v.as_signed_integer()? as i32),
                    67 => level.meta.capacity_string = Some(v.as_string()?.to_string()),
                    68 => level.integrity.triggered_anti_cheat = v.as_boolean()?,
                    69 => level.content.high_object_count = v.as_boolean()?,
                    /* 70 is unused */
                    71 => level.player_stats.mana_orb_percentage = v.as_signed_integer()? as i32,
                    72 => level.flags.has_ldm = v.as_boolean()?,
                    73 => level.flags.ldm_enabled = v.as_boolean()?,
                    74 => level.identity.timely_id = v.as_signed_integer()? as i32,
                    75 => {
                        // special case: EpicRating
                        level.ratings.epic_rating =
                            EpicRating::try_from(v.as_signed_integer()? as i32).ok()?
                    }
                    76 => {
                        // special case: DemonType
                        level.ratings.demon_type =
                            Some(DemonType::try_from(v.as_signed_integer()? as i32).ok()?)
                    }
                    77 => level.flags.is_gauntlet = v.as_boolean()?,
                    78 => level.flags.is_alt_game = v.as_boolean()?,
                    79 => level.flags.unlisted = v.as_boolean()?,
                    80 => level.meta.seconds_editing = v.as_signed_integer()? as i32,
                    81 => level.meta.seconds_editing_copies = v.as_signed_integer()? as i32,
                    82 => level.flags.is_favourited = v.as_boolean()?,
                    83 => level.meta.level_order = v.as_signed_integer()? as i32,
                    84 => level.meta.level_folder = v.as_signed_integer()? as i32,
                    85 => level.player_stats.clicks = v.as_signed_integer()? as i32,
                    86 => level.player_stats.best_attempt_time = v.as_signed_integer()? as i32,
                    87 => level.integrity.level_seed = Some(v.as_signed_integer()? as i32),
                    88 => {
                        // special case: comma-separated list
                        level.player_stats.progress_diffs = v
                            .as_string()?
                            .split(",")
                            .map(|d| d.parse::<i32>().ok())
                            .collect::<Option<Vec<i32>>>()?;
                    }
                    89 => level.integrity.vfd_chk = v.as_boolean()?,
                    90 => level.player_stats.leaderboard_percentage = v.as_signed_integer()? as i32,
                    91 => level.unknowns.k91 = Some(v.as_string()?.to_string()),
                    92 => level.unknowns.k92 = Some(v.as_signed_integer()? as i32),
                    93 => level.flags.unlimited_objects = v.as_boolean()?,
                    94 => level.content.is_platformer = v.as_boolean()?,
                    95 => {
                        level.player_stats.verification_time = Some(v.as_signed_integer()? as i32)
                    }
                    /* 96 - 100 are unused */
                    101 => level.unknowns.k101 = Some(v.as_string()?.to_string()),
                    /* 102 - 103 */
                    104 => {
                        // special case: comma-separated list
                        level.content.song_list = v
                            .as_string()?
                            .split(",")
                            .map(|d| d.parse::<i32>().ok())
                            .collect::<Option<Vec<i32>>>()?;
                    }
                    105 => {
                        // special case: comma-separated list
                        level.content.sfx_list = Some(
                            v.as_string()?
                                .split(",")
                                .map(|d| d.parse::<i32>().ok())
                                .collect::<Option<Vec<i32>>>()?,
                        );
                    }
                    106 => level.unknowns.k106 = Some(v.as_signed_integer()? as i32),
                    107 => level.player_stats.best_time_ms = Some(v.as_signed_integer()? as i32),
                    108 => level.player_stats.best_points = Some(v.as_signed_integer()? as i32),
                    109 => {
                        // special case: comma-separated list
                        level.player_stats.local_best_times = v
                            .as_string()?
                            .split(",")
                            .map(|d| d.parse::<i32>().ok())
                            .collect::<Option<Vec<i32>>>()?;
                    }
                    110 => {
                        // special case: comma-separated list
                        level.player_stats.local_best_points = v
                            .as_string()?
                            .split(",")
                            .map(|d| d.parse::<i32>().ok())
                            .collect::<Option<Vec<i32>>>()?;
                    }
                    111 => level.integrity.platformer_seed = Some(v.as_signed_integer()? as i32),
                    112 => level.flags.no_shake = v.as_boolean()?,
                    _ => {
                        level.unknowns.other.insert(k.clone(), v.clone());
                    }
                }
                continue;
            }
            level.unknowns.other.insert(k.clone(), v.clone());
        }

        Some(level)
    }

    //     /// Returns this object as a `plist::Dictionary`
    //     pub(crate) fn to_dict(&self) -> Dictionary {
    //     }

    /// Returns the Level data as unencrypted.
    /// Level data is left unencrypted when parsing the Level as it is slow.
    pub fn decrypt_level_data(&mut self) -> Result<(), GDError> {
        let raw_data = if let Some(data) = &self.content.data
            && let LevelState::Encrypted(enc) = data
        {
            enc.data.clone()
        } else {
            return Ok(());
        };

        self.content.data = Some(LevelState::Decrypted(LevelData::parse(raw_data).ok_or(
            GDError::CorruptedSavefile("Unable to parse level header".into()),
        )?));
        Ok(())
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data.
    pub fn get_decrypted_data(&self) -> Option<LevelData> {
        let raw_data = match self.content.data.clone() {
            Some(data) => match data {
                LevelState::Encrypted(encrypted) => encrypted.data.clone(),
                LevelState::Decrypted(d) => return Some(d), // already decrypted
            },
            None => return None, // no level data
        };

        LevelData::parse(raw_data)
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data.
    pub fn get_decrypted_data_ref(&mut self) -> Option<&mut LevelData> {
        self.decrypt_level_data().ok()?;
        match &mut self.content.data {
            Some(d) => {
                if let LevelState::Decrypted(data) = d {
                    Some(data)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Adds a GDObject to `self.objects`
    pub fn add_object(&mut self, object: GDObject) {
        if let Some(data) = &mut self.content.data {
            match data {
                LevelState::Decrypted(state) => {
                    state.objects.push(object);
                }
                LevelState::Encrypted(_) => (),
            };
        }
    }

    /// Adds an iterator of GDObjects to `self.objects`
    pub fn add_objects<I: IntoIterator<Item = GDObject>>(&mut self, objects: I) {
        if let Some(data) = &mut self.content.data {
            match data {
                LevelState::Decrypted(state) => {
                    state.objects.extend(objects.into_iter());
                }
                LevelState::Encrypted(_) => (),
            };
        }
    }

    /// Returns a reference to self.content.data.objects if this Level has level data and if the data is decrypted.
    pub fn get_objects_ref(&self) -> Option<&Vec<GDObject>> {
        if let Some(ref data) = self.content.data {
            match data {
                LevelState::Decrypted(state) => Some(&state.objects),
                LevelState::Encrypted(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns a mutable reference to self.content.data.objects if this Level has level data and if the data is decrypted.
    pub fn get_objects_mut(&mut self) -> Option<&mut Vec<GDObject>> {
        if let Some(data) = &mut self.content.data {
            match data {
                LevelState::Decrypted(state) => Some(&mut state.objects),
                LevelState::Encrypted(_) => None,
            }
        } else {
            None
        }
    }

    /// Returns a copy of self.content.data.objects if this Level has level data and if the data is decrypted.
    pub fn get_objects_clone(&self) -> Option<Vec<GDObject>> {
        if let Some(ref data) = self.content.data {
            match data {
                LevelState::Decrypted(state) => Some(state.objects.clone()),
                LevelState::Encrypted(_) => None,
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
                    LevelState::Encrypted(enc) => (enc.data.len().to_string(), " Bytes"),
                    LevelState::Decrypted(dec) => (dec.objects.len().to_string(), " Objects"),
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
            identity: GDLevelIdentity {
                ..Default::default()
            },
            content: GDLevelData {
                ..Default::default()
            },
            ratings: GDLevelRatings {
                ..Default::default()
            },
            coins: GDLevelCoins {
                ..Default::default()
            },
            player_stats: GDLevelPlayerStats {
                ..Default::default()
            },
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
            integrity: GDLevelIntegrity {
                ..Default::default()
            },
            unknowns: GDLevelUnknowns {
                k101: Some("0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0".into()),
                ..Default::default()
            },
        }
    }
}
