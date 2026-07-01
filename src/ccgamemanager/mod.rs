//! This module covers everything in the CCGameManager.dat file.

use std::{array, collections::HashMap, hash::BuildHasherDefault, io::Cursor};

use nohash_hasher::NoHashHasher;
use plist::{Dictionary, Value};

use crate::{
    ccgamemanager::structs::{
        GDAccount, GDConfig, GDCurrentValues, GDPlatformerUI, GDPlayerInfo, GDSearchFilters,
        GDSongConfig, GDStatistics, Resolution, TextureQuality,
    },
    cclocallevels::gdlevel::{CCLocalLevels, GDLevel, PLIST_HEADER, leveldata::parse_objects},
    core::{GDError, get_ccgamemanager_path, io::decrypt_file, proper_plist_tags},
};

pub mod achievements;
pub mod structs;

type IntMap<V> = HashMap<i32, V, BuildHasherDefault<NoHashHasher<i32>>>;

/// Container struct for the CCGameManager.dat file
#[derive(Debug, Default, Clone)]
pub struct CCGameManager {
    /// Info about the player. Namely, selected icons
    pub player_info: GDPlayerInfo,
    /// Player statistics stored in the file
    pub stats: GDStatistics,
    /// Player's configuration of the game (e.g. volume, resolution, texture quality, etc.)
    pub config: GDConfig,
    /// Account info
    pub account: GDAccount,
    /// Keybindings descriptors. Not much is known about these.
    ///
    /// Internal keys: `KBM_001`, `KBM_002` respectively
    pub keybinds: (Dictionary, Dictionary),
    /// Temporary state variables
    pub temp_state: GDCurrentValues,
    /// MDLM_001 and MDLM_002
    pub song_config: GDSongConfig,

    /// Unaccounted-for properties
    pub other_properties: HashMap<String, Value>,
}

impl CCGameManager {
    /// Parses the local CCGameManager.dat file if it exists and is a valid file.
    pub fn from_local() -> Result<Self, GDError> {
        let path = get_ccgamemanager_path().ok_or(GDError::MissingSavefile)?;
        Self::from_raw_string(decrypt_file(path).unwrap())
    }

    /// Parses a raw savefile plist to this object. The parser accounts for unknown keys, however it will panic in the case of a mistyped value.
    /// For example, an achievement's progress value not being a numeric utf-8 string will cause the parser to panic.  
    /// Please consider this when parsing possibly malformed savefiles.
    pub fn from_raw_string(s: String) -> Result<Self, GDError> {
        if !s.starts_with(PLIST_HEADER) {
            return Err(GDError::CorruptedSavefile("Savefile header does not match the expected header. This may be due to a corrupted savefile or a savefile from a previous version of GD.".into()));
        };

        let xmltree = match Value::from_reader_xml(Cursor::new(proper_plist_tags(s)?.as_bytes())) {
            Ok(v) => v.into_dictionary().unwrap(),
            Err(e) => return Err(GDError::BadPlist(e)),
        };

        let mut this = Self::default();
        if let None = this.parse_dict(xmltree) {
            return Err(GDError::CorruptedSavefile(
                "Unable to parse corrupted savefile.".into(),
            ));
        }

        Ok(this)
    }

    fn parse_dict(&mut self, dict: Dictionary) -> Option<()> {
        let mut d = dict;
        // string
        parse_values(
            &mut d,
            &mut [
                ("playerUDID", &mut self.player_info.udid),
                ("playerName", &mut self.player_info.username),
                ("GJA_001", &mut self.account.username),
            ],
            |v| v.as_string().map(|v| v.to_string()),
        )?;

        // opt. string
        parse_values(
            &mut d,
            &mut [
                ("GJA_002", &mut self.account.plaintext_password),
                ("GJA_004", &mut self.account.session_id),
                ("GJA_005", &mut self.account.hashed_password),
            ],
            |v| v.as_string().map(|v| Some(v.to_string())),
        )?;

        // i32
        parse_values(
            &mut d,
            &mut [
                ("playerUserID", &mut self.player_info.user_id),
                ("playerFrame", &mut self.player_info.icon_cube),
                ("playerShip", &mut self.player_info.icon_ship),
                ("playerBall", &mut self.player_info.icon_ball),
                ("playerBird", &mut self.player_info.icon_ufo),
                ("playerDart", &mut self.player_info.icon_wave),
                ("playerRobot", &mut self.player_info.icon_robot),
                ("playerSpider", &mut self.player_info.icon_spider),
                ("playerSwing", &mut self.player_info.icon_swing),
                ("playerColor", &mut self.player_info.player_col1),
                ("playerColor2", &mut self.player_info.player_col2),
                ("playerColor3", &mut self.player_info.player_col_glow),
                ("playerStreak", &mut self.player_info.icon_streak),
                ("playerShipStreak", &mut self.player_info.ship_streak),
                ("playerDeathEffect", &mut self.player_info.death_effect),
                ("playerJetpack", &mut self.player_info.icon_jetpack),
                ("playerIconType", &mut self.player_info.icon_type),
                ("bootups", &mut self.stats.bootups),
                ("binaryVersion", &mut self.config.binary_version),
                ("timeOffset", &mut self.config.music_offset),
                ("GJA_003", &mut self.account.account_id),
                ("GS_20", &mut self.stats.demon_keys),
                ("GLM_11", &mut self.temp_state.current_daily_level),
                ("GLM_17", &mut self.temp_state.current_weekly_level),
                ("GLM_23", &mut self.config.glm23_unknown),
                ("MDLM_002", &mut self.song_config.song_priority),
            ],
            |v| v.as_signed_integer().map(|v| v as i32),
        )?;

        // bool
        parse_values(
            &mut d,
            &mut [
                ("playerGlow", &mut self.player_info.using_glow),
                ("hasRP", &mut self.player_info.is_moderator),
                ("showSongMarkers", &mut self.config.show_song_markers),
                ("showProgressBar", &mut self.config.show_progress_bar),
                ("clickedGarage", &mut self.config.has_clicked_garage),
                ("clickedEditor", &mut self.config.has_clicked_editor),
                ("clickedPractice", &mut self.config.has_clicked_practice),
                ("showedEditorGuide", &mut self.config.seen_editor_guide),
                ("showedLowDetailDialog", &mut self.config.seen_ldm_dialog),
                (
                    "showedRateStarDialog",
                    &mut self.config.seen_rate_star_dialog,
                ),
                ("hasRatedGame", &mut self.config.has_rated_game),
            ],
            |v| v.as_boolean(),
        )?;

        // f32
        parse_values(
            &mut d,
            &mut [
                ("bgVolume", &mut self.config.bgm_volume),
                ("sfxVolume", &mut self.config.sfx_volume),
                ("practicePosX", &mut self.config.practice_ui_pos.0),
                ("practicePosY", &mut self.config.practice_ui_pos.1),
                ("practiceOpacity", &mut self.config.practice_ui_opacity),
                ("customFPSTarget", &mut self.config.fps_target),
            ],
            |v| v.as_real().map(|v| v as f32),
        )?;

        for i in 0..5 {
            parse_val(&mut d, &format!("dpad0{}", i + 1), |v| {
                self.config.dpads[i] = GDPlatformerUI::from_str(v.as_string().unwrap());
                Some(())
            })?;
        }

        self.config.dpad_layout = d
            .get("dpad_layout")
            .map(|v| GDPlatformerUI::from_str(v.as_string().unwrap()));

        parse_val(&mut d, "resolution", |v| {
            self.config.resolution = Resolution::try_from(v.as_signed_integer()? as i32).ok()?;
            Some(())
        })?;
        parse_val(&mut d, "texQuality", |v| {
            self.config.texture_quality =
                TextureQuality::try_from(v.as_signed_integer()? as i32).ok()?;
            Some(())
        })?;

        parse_val(&mut d, "KBM_001", |v| {
            self.keybinds.0 = v.as_dictionary()?.clone();
            Some(())
        })?;

        parse_val(&mut d, "KBM_002", |v| {
            self.keybinds.1 = v.as_dictionary()?.clone();
            Some(())
        })?;

        // {level_id: GDlevel}
        // common format for storing lists of levels
        // though, some level lists follow a slightly different schema for keys
        parse_values(
            &mut d,
            &mut [
                ("GLM_01", &mut self.stats.official_level_progresses),
                ("GLM_03", &mut self.stats.online_levels_played),
                ("GLM_16", &mut self.stats.gauntlet_levels_played),
            ],
            |v| parse_level_dict(&v),
        )?;

        // {i32: "1"}
        // robtop seems to use this format for lists of things
        parse_values(
            &mut d,
            &mut [
                ("GLM_06", &mut self.account.following_creators),
                ("GLM_07", &mut self.temp_state.last_played_levels),
                ("GLM_13", &mut self.stats.submitted_ratings),
                ("GLM_14", &mut self.account.reported_levels),
                ("GLM_15", &mut self.stats.submitted_ratings_demons),
            ],
            |v| {
                Some(
                    v.as_dictionary()?
                        .iter()
                        .map(|(k, _)| k.parse::<i32>().unwrap())
                        .collect::<Vec<_>>(),
                )
            },
        )?;

        parse_values(
            &mut d,
            &mut [
                ("GLM_18", &mut self.config.saved_levels_foldernames),
                ("GLM_19", &mut self.config.local_levels_foldernames),
            ],
            parse_foldernames,
        )?;

        parse_val(
            &mut d,
            "GLM_12",
            // keys always of the form `likes_a_b_c_d` where a, b, c, d are i32
            |v| {
                self.config.glm12_unknown = v
                    .as_dictionary()?
                    .iter()
                    .map(|(k, _)| {
                        let mut split = k.split("_").into_iter();
                        let _ = split.next(); // skip `like`
                        let keys =
                            array::from_fn(|_| split.next().unwrap().parse::<i32>().unwrap());
                        keys
                    })
                    .collect::<Vec<_>>();
                Some(())
            },
        )?;

        parse_val(&mut d, "GLM_10", |v| {
            self.stats.completed_dailies = v
                .as_dictionary()?
                .iter()
                .map(|(k, v)| {
                    (
                        k.parse::<i32>().unwrap(),
                        GDLevel::from_dict(v.as_dictionary().unwrap()).unwrap(),
                    )
                })
                .collect();

            Some(())
        })?;

        parse_val(&mut d, "GLM_22", |v| {
            self.config.favourite_lists = CCLocalLevels::parse_lists_from_value(&v).ok()?;
            Some(())
        })?;

        parse_val(&mut d, "customObjectDict", |v| {
            self.config.custom_objects = v
                .as_dictionary()?
                .iter()
                .map(|(k, v)| {
                    (
                        k.parse::<i32>().unwrap(),
                        parse_objects(v.as_string().unwrap()),
                    )
                })
                .collect();
            Some(())
        })?;

        parse_val(&mut d, "reportedAchievements", |v| {
            // input dict: {ident: progress}
            v.as_dictionary()?.iter().for_each(|(ident, progress)| {
                self.stats.set_achievement_by_ident(
                    ident,
                    progress.as_string().unwrap().parse::<u16>().unwrap() as u16,
                );
            });
            Some(())
        })?;

        parse_val(&mut d, "GLM_08", |v| {
            self.config.search_filters = GDSearchFilters::from_dict(v.as_dictionary()?)?;
            Some(())
        })?;

        // todo: parse GLM_20
        // currently too lazy to do that because it's a complex key

        /* Values not parsed */
        // GLM_02, GLM_04, GS_8: These keys are unused in modern (2.2+) GD savefiles.
        /* values unknown */
        // MDLM_003: Unknown dictionary
        // GLM_09: Has something to do with filters for online levels, but it appears unused

        self.other_properties = d.into_iter().collect();

        Some(())
    }
}

// removes all valid values specified in `fields`
// this function ensures that `d` is left with only the unaccounted keys
#[must_use]
fn parse_values<F: Fn(Value) -> Option<R>, R>(
    d: &mut Dictionary,
    fields: &mut [(&str, &mut R)],
    parser: F,
) -> Option<()> {
    for (k, f) in fields {
        if let Some(v) = d.remove(k) {
            **f = parser(v)?;
        }
    }
    Some(())
}

#[must_use]
#[inline]
fn parse_val<F: FnMut(Value) -> Option<()>>(
    d: &mut Dictionary,
    key: &str,
    mut parser: F,
) -> Option<()> {
    if let Some(v) = d.remove(key) {
        parser(v)
    } else {
        Some(())
    }
}

fn parse_level_dict(v: &Value) -> Option<Vec<GDLevel>> {
    v.as_dictionary()?
        .iter()
        .map(|(_id, level_dict)| GDLevel::from_dict(level_dict.as_dictionary().unwrap()))
        .collect::<Option<Vec<GDLevel>>>()
}

fn parse_foldernames(v: Value) -> Option<Vec<(i32, String)>> {
    if v.as_dictionary()?.is_empty() {
        return Some(vec![]);
    }

    let mut raw_folders = v
        .as_dictionary()?
        .iter()
        .map(|(idx, name)| {
            (
                idx.parse::<i32>().unwrap(),
                name.as_string().unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();

    // this ensures that the folders are ordered by index
    raw_folders.sort_by(|(a, _), (b, _)| a.cmp(b));

    Some(raw_folders)
}
