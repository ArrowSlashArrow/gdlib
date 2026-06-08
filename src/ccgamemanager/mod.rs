//! This module covers everything in the CCGameManager.dat file.

use std::{collections::HashMap, io::Cursor};

use plist::{Dictionary, Value};

use crate::{
    cclocallevels::gdlevel::{GDLevel, PLIST_HEADER},
    core::{GDError, get_ccgamemanager_path, io::decrypt_file, proper_plist_tags},
    repr_t,
};

/// Container struct for the CCGameManager.dat file
#[derive(Debug, Default, Clone)]
pub struct CCGameManager {
    /// Info about the player. Namely, selected icons
    pub player_info: GDPlayerInfo,
    /// User statistics stored in the file
    pub stats: GDStatistics,
    /// User's configuration of the game (e.g. volume, resolution, texture quality, etc.)
    pub config: GDConfig,
    /// Account info
    pub account: GDAccount,
    /// Keybindings descriptors. Not much is known about these.
    ///
    /// Internal keys: `KBM_001`, `KBM_002` respectively
    pub keybinds: (Dictionary, Dictionary),
    /// Temporary state variables
    pub temp_state: GDTempState,

    /// Unaccounted-for properties
    pub other_properties: HashMap<String, Value>,
}

impl CCGameManager {
    /// Parses the local CCGameManager.dat file if it exists and is a valid file.
    pub fn from_local() -> Result<Self, GDError> {
        let path = get_ccgamemanager_path().ok_or(GDError::MissingSavefile)?;
        Self::from_raw_string(decrypt_file(path).unwrap())
    }

    /// Parses a raw plist to this object
    pub fn from_raw_string(s: String) -> Result<Self, GDError> {
        if !s.starts_with(PLIST_HEADER) {
            return Err(GDError::CorruptedSavefile("Savefile header does not match the expected header. This may be due to a corrupted savefile or a savefile from a previous version of GD.".into()));
        };

        let xmltree = match Value::from_reader_xml(Cursor::new(proper_plist_tags(s).as_bytes())) {
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
            |v| v.as_string().unwrap().to_string(),
        );

        // opt. string
        parse_values(
            &mut d,
            &mut [
                ("GJA_002", &mut self.account.plaintext_password),
                ("GJA_004", &mut self.account.session_id),
                ("GJA_005", &mut self.account.hashed_password),
            ],
            |v| Some(v.as_string().unwrap().to_string()),
        );

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
            ],
            |v| v.as_signed_integer().unwrap() as i32,
        );

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
            |v| v.as_boolean().unwrap(),
        );

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
            |v| v.as_real().unwrap() as f32,
        );

        for i in 0..5 {
            parse_val(&mut d, &format!("dpad0{}", i + 1), |v| {
                self.config.dpads[i] = GDPlatformerUI::from_str(v.as_string().unwrap());
                Some(())
            });
        }

        self.config.dpad_layout = d
            .get("dpad_layout")
            .map(|v| GDPlatformerUI::from_str(v.as_string().unwrap()));

        parse_val(&mut d, "resolution", |v| {
            self.config.resolution = Resolution::try_from(v.as_signed_integer()? as i32).ok()?;
            Some(())
        });
        parse_val(&mut d, "texQuality", |v| {
            self.config.text_quality =
                TextureQuality::try_from(v.as_signed_integer()? as i32).ok()?;
            Some(())
        });

        parse_val(&mut d, "KBM_001", |v| {
            self.keybinds.0 = v.as_dictionary()?.clone();
            Some(())
        });

        parse_val(&mut d, "KBM_002", |v| {
            self.keybinds.1 = v.as_dictionary()?.clone();
            Some(())
        });

        parse_values(
            &mut d,
            &mut [
                ("GLM_01", &mut self.stats.official_level_progresses),
                ("GLM_03", &mut self.stats.online_levels_played),
            ],
            |v| parse_level_dict(&v).unwrap(),
        );

        // {i32: "1"}
        // robtop seems to use this format for lists of things
        parse_values(
            &mut d,
            &mut [
                ("GLM_06", &mut self.account.following_creators),
                ("GLM_07", &mut self.temp_state.last_played_levels),
                ("GLM_13", &mut self.stats.submitted_ratings),
            ],
            |v| {
                v.as_dictionary()
                    .unwrap()
                    .iter()
                    .map(|(k, _)| k.parse::<i32>().unwrap())
                    .collect()
            },
        );

        /* Values not parsed */
        // GLM_02, GLM_04, GS_8: These keys are unused and modern (2.2) GD savefiles.

        // self.other_properties = d.into_iter().collect();

        Some(())
    }
}

// removes all valid values specified in `fields`
// this function ensures that `d` is left with only the unaccounted keys
fn parse_values<F: Fn(Value) -> R, R>(
    d: &mut Dictionary,
    fields: &mut [(&str, &mut R)],
    parser: F,
) {
    for (k, f) in fields {
        if let Some(v) = d.remove(k) {
            **f = parser(v);
        }
    }
}

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

/// Player info: username, UDID, user id, all icon info
#[derive(Debug, Default, Clone)]
#[allow(missing_docs)]
pub struct GDPlayerInfo {
    pub username: String,
    pub udid: String,
    pub user_id: i32,
    /// Internally, `playerFrame`
    pub icon_cube: i32,
    pub icon_ship: i32,
    pub icon_ball: i32,
    /// Internally, `playerBird`
    pub icon_ufo: i32,
    /// Internally, `playerDart`
    pub icon_wave: i32,
    pub icon_robot: i32,
    pub icon_spider: i32,
    pub icon_swing: i32,
    pub player_col1: i32,
    pub player_col2: i32,
    pub player_col_glow: i32,
    pub icon_streak: i32,
    pub ship_streak: i32,
    pub death_effect: i32,
    pub icon_jetpack: i32,
    pub icon_type: i32,
    pub using_glow: bool,
    pub is_moderator: bool,
}

/// Player-specific statistics.
#[derive(Debug, Default, Clone)]
#[allow(missing_docs)]
pub struct GDStatistics {
    /// Number of times this player has launched GD
    pub bootups: i32,
    // todo: achievements, GLM_XX, GS_XX
    /// All official levels that the player has progress on.
    ///
    /// Internal key: `GLM_01`
    pub official_level_progresses: Vec<GDLevel>,
    pub online_levels_played: Vec<GDLevel>,
    pub demon_keys: i32,
    /// All levels the player has submitted ratings on
    pub submitted_ratings: Vec<i32>,
}

/// User's configuration of the game.
#[derive(Debug, Default, Clone)]
#[allow(missing_docs)]
pub struct GDConfig {
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub text_quality: TextureQuality,
    pub resolution: Resolution,
    pub show_song_markers: bool,
    pub show_progress_bar: bool,
    pub has_clicked_garage: bool,
    pub has_clicked_editor: bool,
    pub has_clicked_practice: bool,
    pub seen_editor_guide: bool,
    pub seen_ldm_dialog: bool,
    pub seen_rate_star_dialog: bool,
    pub has_rated_game: bool,
    pub binary_version: i32,
    pub practice_ui_pos: (f32, f32),
    pub practice_ui_opacity: f32,
    pub fps_target: f32,
    /// Music offset in milliseconds
    pub music_offset: i32,
    /// `dpadn` for n in 1..=5
    pub dpads: [GDPlatformerUI; 5],
    pub dpad_layout: Option<GDPlatformerUI>,
}

repr_t!(
    strict TextureQuality: i32 {
        Auto = 0,
        Low = 1,
        Medium = 2,
        High = 3,
    } default Auto
);

repr_t!(
    strict Resolution: i32 {
        R640x480 = 1,     // 4:3
        R720x480 = 2,     // 3:2
        R720x576 = 3,     // 5:4
        R800x600 = 4,     // 4:3
        R1024x768 = 5,    // 4:3
        R1152x864 = 6,    // 4:3
        R1176x664 = 7,    // 147:83
        R1280x720 = 8,    // 16:9
        R1280x768 = 9,    // 5:3
        R1280x800 = 10,   // 16:10
        R1280x960 = 11,   // 4:3
        R1280x1024 = 12,  // 5:4
        R1360x768 = 13,   // 85:48
        R1366x768 = 14,   // 683:384
        R1440x900 = 15,   // 16:10
        R1600x900 = 16,   // 16:9
        R1600x1024 = 17,  // 25:16
        R1600x1200 = 18,  // 4:3
        R1680x1050 = 19,  // 16:10
        R1768x992 = 20,   // 221:124
        R1920x1080 = 21,  // 16:9
        R1920x1200 = 22,  // 16:10
        R1920x1440 = 23,  // 4:3
        R2048x1536 = 24,  // 4:3
        R2560x1440 = 25,  // 16:9
        R2560x1600 = 26,  // 16:10
        R3840x2160 = 27,  // 16:9
    } default R1920x1080
);

#[derive(Debug, Default, Clone)]
#[allow(missing_docs)]
/// Platformer controls UI config
pub struct GDPlatformerUI {
    pub width: i32,      // The width of the button hitboxl
    pub height: i32,     // The height of the button hitbox
    pub scale: f32,      // The scale of the buttons
    pub opacity: i32,    // The button opacity (from 0 to 255)
    pub pos: (f32, f32), // The position of the buttons
    pub mode_b: bool,    // The ModeB checkbox
    pub deadzone: f32,   // The deadzone between the buttons
    pub radius: f32,     // The distance between the buttons
    pub snap: bool,      // The Snap checkbox
    pub split: bool,     // The Split checkbox
}

impl GDPlatformerUI {
    /// Parses a comma-separated list of values to this object
    pub fn from_str(s: &str) -> Self {
        let mut this = Self::default();
        let fns = &[
            Self::parse_width,
            Self::parse_height,
            Self::parse_scale,
            Self::parse_opacity,
            Self::parse_pos_x,
            Self::parse_pos_y,
            Self::parse_mode_b,
            Self::parse_deadzone,
            Self::parse_radius,
            Self::parse_snap,
            Self::parse_split,
        ];
        s.split(",")
            .into_iter()
            .enumerate()
            .for_each(|(idx, s)| (fns[idx])(&mut this, s));

        this
    }

    fn parse_width(&mut self, s: &str) {
        self.width = s.parse::<i32>().unwrap();
    }
    fn parse_height(&mut self, s: &str) {
        self.height = s.parse::<i32>().unwrap();
    }
    fn parse_scale(&mut self, s: &str) {
        self.scale = s.parse::<f32>().unwrap();
    }
    fn parse_opacity(&mut self, s: &str) {
        self.opacity = s.parse::<i32>().unwrap();
    }
    fn parse_pos_x(&mut self, s: &str) {
        self.pos.0 = s.parse::<f32>().unwrap();
    }
    fn parse_pos_y(&mut self, s: &str) {
        self.pos.1 = s.parse::<f32>().unwrap();
    }
    fn parse_mode_b(&mut self, s: &str) {
        self.mode_b = s.parse::<i32>().unwrap() == 1;
    }
    fn parse_deadzone(&mut self, s: &str) {
        self.deadzone = s.parse::<f32>().unwrap();
    }
    fn parse_radius(&mut self, s: &str) {
        self.radius = s.parse::<f32>().unwrap();
    }
    fn parse_snap(&mut self, s: &str) {
        self.snap = s.parse::<i32>().unwrap() == 1;
    }
    fn parse_split(&mut self, s: &str) {
        self.split = s.parse::<i32>().unwrap() == 1;
    }
}

/// Configuration to do with the player's account and social settings
#[derive(Debug, Default, Clone)]
#[allow(missing_docs)]
pub struct GDAccount {
    pub username: String,
    /// Password in plaintext (used in 2.1 and below)
    pub plaintext_password: Option<String>,
    pub account_id: i32,
    /// Appears to be unused
    pub session_id: Option<String>,
    /// Password encrypted with GJP2 encryption. This can be generated with [`crate::core::crypto::generate_gjp2_hexdigest`]
    pub hashed_password: Option<String>,
    /// List of creators' account IDs that this player follows
    pub following_creators: Vec<i32>,
}

/// Temporary variables stored in the savefile that are expected to be overwritten in the future
#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct GDTempState {
    /// Levels that were played in the last session
    pub last_played_levels: Vec<i32>,
    /// The current daily level's TimelyID
    pub current_daily_level: i32,
}

/* TODO: for GLM_08, make a GDSearchFilter struct. all fields are boolean, so use bitflags */
/* TODO 2: add a "Internal key: `...`" footer for every (sub)struct field docstring in CCGameManager */
