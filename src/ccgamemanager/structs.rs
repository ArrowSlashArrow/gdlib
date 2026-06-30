//! All sub-component structs of ccgamemanager::CCGameManager.

use bitflags::bitflags;
use plist::Dictionary;

use crate::{
    ccgamemanager::IntMap,
    cclocallevels::{gdlevel::GDLevel, gdlist::GDList, gdobj::GDObject},
    repr_t,
};

// ---- Substructs ----

/// Player info: username, UDID, user id, all icon info
#[derive(Debug, Default, Clone)]
pub struct GDPlayerInfo {
    /// Internal key: `playerName`
    pub username: String,
    /// Internal key: `playerUDID`
    pub udid: String,
    /// Internal key: `playerUserID`
    pub user_id: i32,
    /// Internal key: `playerFrame`
    pub icon_cube: i32,
    /// Internal key: `playerShip`
    pub icon_ship: i32,
    /// Internal key: `playerBall`
    pub icon_ball: i32,
    /// Internal key: `playerBird`
    pub icon_ufo: i32,
    /// Internal key: `playerDart`
    pub icon_wave: i32,
    /// Internal key: `playerRobot`
    pub icon_robot: i32,
    /// Internal key: `playerSpider`
    pub icon_spider: i32,
    /// Internal key: `playerSwing`
    pub icon_swing: i32,
    /// Internal key: `playerColor`
    pub player_col1: i32,
    /// Internal key: `playerColor2`
    pub player_col2: i32,
    /// Internal key: `playerColor3`
    pub player_col_glow: i32,
    /// Internal key: `playerStreak`
    pub icon_streak: i32,
    /// Internal key: `playerShipStreak`
    pub ship_streak: i32,
    /// Internal key: `playerDeathEffect`
    pub death_effect: i32,
    /// Internal key: `playerJetpack`
    pub icon_jetpack: i32,
    /// Internal key: `playerIconType`
    pub icon_type: i32,
    /// Internal key: `playerGlow`
    pub using_glow: bool,
    /// Internal key: `hasRP`
    pub is_moderator: bool,
}

/// Player-specific statistics.
#[derive(Debug, Default, Clone)]
pub struct GDStatistics {
    /// Number of times this player has launched GD
    ///
    /// Internal key: `bootups`
    pub bootups: i32,
    /// All official levels that the player has progress on.
    ///
    /// Internal key: `GLM_01`
    pub official_level_progresses: Vec<GDLevel>,
    /// Internal key: `GLM_03`
    pub online_levels_played: Vec<GDLevel>,
    /// Internal key: `GS_20`
    pub demon_keys: i32,
    /// All level IDs the player has submitted difficulty ratings for
    ///
    /// Internal key: `GLM_13`
    pub submitted_ratings: Vec<i32>,
    /// All demon levels the player has submitted ratings on
    ///
    /// Internal key: `GLM_15`
    pub submitted_ratings_demons: Vec<i32>,
    /// All gauntlet levels that the player has progress on
    ///
    /// Internal key: `GLM_16`
    pub gauntlet_levels_played: Vec<GDLevel>,
    /// All completed dailies in the form `{timely id: level}`
    ///
    /// Internal keys: `GLM_10`
    pub completed_dailies: IntMap<GDLevel>,
}

/// User's configuration of the game.
#[derive(Debug, Default, Clone)]
pub struct GDConfig {
    /// Internal key: `bgVolume`
    pub bgm_volume: f32,
    /// Internal key: `sfxVolume`
    pub sfx_volume: f32,
    /// Internal key: `texQuality`
    pub texture_quality: TextureQuality,
    /// Internal key: `resolution`
    pub resolution: Resolution,
    /// Internal key: `showSongMarkers`
    pub show_song_markers: bool,
    /// Internal key: `showProgressBar`
    pub show_progress_bar: bool,
    /// Internal key: `clickedGarage`
    pub has_clicked_garage: bool,
    /// Internal key: `clickedEditor`
    pub has_clicked_editor: bool,
    /// Internal key: `clickedPractice`
    pub has_clicked_practice: bool,
    /// Internal key: `showedEditorGuide`
    pub seen_editor_guide: bool,
    /// Internal key: `showedLowDetailDialog`
    pub seen_ldm_dialog: bool,
    /// Internal key: `showedRateStarDialog`
    pub seen_rate_star_dialog: bool,
    /// Internal key: `hasRatedGame`
    pub has_rated_game: bool,
    /// Internal key: `binaryVersion`
    pub binary_version: i32,
    /// Respective internal keys: `practicePosX`, `praticePosY`
    pub practice_ui_pos: (f32, f32),
    /// Internal key: `practiceOpacity`
    pub practice_ui_opacity: f32,
    /// Internal key: `customFPSTarget`
    pub fps_target: f32,
    /// Music offset in milliseconds
    ///
    /// Internal key: `timeOffset`
    pub music_offset: i32,
    /// Internal keys: `dpad01`, `dpad02`, `dpad03`, `dpad04`, `dpad05`
    pub dpads: [GDPlatformerUI; 5],
    /// Internal key: `dpad_layout`
    pub dpad_layout: Option<GDPlatformerUI>,
    /// List of folder names for saved online levels. Folder names are stored in order, starting from folder 1. If an unnamed folder is found at index >= 1, it is stored as a `None`.
    ///
    /// Internal key: `GLM_18`
    pub saved_levels_foldernames: Vec<(i32, String)>,
    /// List of folder names for locally created levels (found in the editor tab). Folder names are stored in order, starting from folder 1. If an unnamed folder is found at index >= 1, it is stored as a `None`.
    ///
    /// Internal key: `GLM_19`
    pub local_levels_foldernames: Vec<(i32, String)>,
    /// Raw GLM_12 key encoding optimized for size. This key has a purpose that is assumed to be related to likes, though it is unknown.
    ///
    /// Internal key: `GLM_12`
    pub glm12_unknown: Vec<[i32; 4]>,
    /// Internal key: `GLM_23`
    pub glm23_unknown: i32,
    /// Lists that this player has favourited.
    ///
    /// Internal key: `GLM_22`
    pub favourite_lists: Vec<GDList>,
    /// Saved custom objects from the editor. Custom objects are indexed by some negative integer in the raw dictionary which serves an unclear purpose. Each element is stored as (index, objects).
    ///
    /// Internal key: `customObjectDict`
    pub custom_objects: Vec<(i32, Vec<GDObject>)>,
}

/// Configuration to do with the player's account and social settings
#[derive(Debug, Default, Clone)]
pub struct GDAccount {
    /// Internal key: `GJA_001`
    pub username: String,
    /// Password in plaintext (used in 2.1 and below)
    ///
    /// Internal key: `GJA_002`
    pub plaintext_password: Option<String>,
    /// Internal key: `GJA_003`
    pub account_id: i32,
    /// Appears to be unused
    ///
    /// Internal key: `GJA_004`
    pub session_id: Option<String>,
    /// Password encrypted with GJP2 encryption. This can be generated with [`crate::core::crypto::generate_gjp2_hexdigest`]
    ///
    /// Internal key: `GJA_005`
    pub hashed_password: Option<String>,
    /// List of creators' account IDs that this player follows
    ///
    /// Internal key: `GLM_06`
    pub following_creators: Vec<i32>,
    /// List of levels that the player has reported.
    ///
    /// Internal key: `GLM_14`
    pub reported_levels: Vec<i32>,
}

/// Temporary variables stored in the savefile that are expected to be overwritten in the future
#[derive(Debug, Default, Clone)]
pub struct GDCurrentValues {
    /// Levels that were played in the last session
    ///
    /// Internal key: `GLM_07`
    pub last_played_levels: Vec<i32>,
    /// The current daily level's TimelyID
    ///
    /// Internal key: `GLM_11`
    pub current_daily_level: i32,
    /// The current weekly level's TimelyID
    ///
    /// Internal key: `GLM_17`
    pub current_weekly_level: i32,
}

/// Song config
#[derive(Debug, Default, Clone)]
pub struct GDSongConfig {
    /// Presumably the songs that the user has stored or downloaded, but is unknown.
    ///
    /// Internal key: `MDLM_001`
    pub stored_songs: (), // Vec<SongInfoObject>
    /// Has something to do with song priority, but is unknown.
    ///
    /// Internal key: `MDLM_002`    
    pub song_priority: i32,
}

// ---- Supplementary structs ----

/* todo
struct descriptor for SongInfoObject:

1	ID	Integer	The ID of the song on Newgrounds
2	name	String	The name of the song
3	artistID	Integer	Newgrounds ArtistID
4	artistName	String	The name of the artist who made the song
5	size	Integer	Size of the song in MB, rounded to two decimal places
6	videoID	String	the Video ID for the songs YouTube Video
7	youtubeURL	String	The URL of the newgrounds user's youtube channel
8	isVerified	Bool	if the song artist is scouted on newgrounds
9	songPriority	Integer	priority over the song list
10	link	String	Link to the song's mp3
11	nongEnum	Integer	Type of NONG. 0 for none, 1 for NCS.
12	extraArtistIDs	Array[Integer]	IDs of extra artists, separated by .
13	new	Boolean	Whether the NEW icon shows up or not
14	newType	Integer	Type of NEW icon. 0 for Yellow, 1 for Blue
15	extraArtistNames	Array	Artist names in this format: {id},{name},{id},{name}

*/

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
/// Platformer controls UI config
pub struct GDPlatformerUI {
    /// The width of the button hitbox
    pub width: i32,
    /// The height of the button hitbox
    pub height: i32,
    /// The scale of the buttons
    pub scale: f32,
    /// The button opacity (from 0 to 255)
    pub opacity: i32,
    /// The position of the buttons
    pub pos: (f32, f32),
    /// The ModeB checkbox
    pub mode_b: bool,
    /// The deadzone between the buttons
    pub deadzone: f32,
    /// The distance between the buttons
    pub radius: f32,
    /// The Snap checkbox
    pub snap: bool,
    /// The Split checkbox
    pub split: bool,
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

/* TODO: for GLM_08, make a GDSearchFilter struct. all fields are boolean, so use bitflags */
/* TODO 2: add a "Internal key: `...`" footer for every (sub)struct field docstring in CCGameManager */
#[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
/// Search filter state for online levels
pub struct GDSearchFilters {
    /// The majority of search filters. Contains all search boolean flags by which online levels may be searched.
    pub boolean_flags: GDSearchFilterFlags,
    /// Optionally enabled song ID filter. Filters out all levels that do not have this song ID if given.
    pub song: Option<i32>,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
    #[must_use]
    /// Filters for searching online levels
    pub struct GDSearchFilterFlags: u32 {
        /// Internal key: `Diff0`
        const DifficultyNA       = 1;
        /// Internal key: `Diff1`
        const DifficultyAuto     = 1 << 1;
        /// Internal key: `Diff2`
        const DifficultyEasy     = 1 << 2;
        /// Internal key: `Diff3`
        const DifficultyNormal   = 1 << 3;
        /// Internal key: `Diff4`
        const DifficultyHard     = 1 << 4;
        /// Internal key: `Diff5`
        const DifficultyHarder   = 1 << 5;
        /// Internal key: `Diff6`
        const DifficultyInsane   = 1 << 6;
        /// Internal key: `Diff7`
        const DifficultyDemon    = 1 << 7;
        /// Tiny levels
        ///
        /// Internal key: `Len0`
        const LengthTiny         = 1 << 8;
        /// Small levels
        ///
        /// Internal key: `Len1`
        const LengthSmall        = 1 << 9;
        /// Medium levels
        ///
        /// Internal key: `Len2`
        const LengthMedium       = 1 << 10;
        /// Long levels
        ///
        /// Internal key: `Len3`
        const LengthLong         = 1 << 11;
        /// XL levels
        ///
        /// Internal key: `Len4`
        const LengthXL           = 1 << 12;
        /// Internal key: `demon_filter`
        const DemonFilter       = 1 << 13;
        /// Platformer levels
        ///
        /// Internal key: `Len5`
        const Platformer         = 1 << 14;
        /// Star-rated levels
        ///
        /// Internal key: `star_filter`
        const StarFilter        = 1 << 15;
        /// Internal key: `mythic_filter`
        const mythic_filter      = 1 << 18;
        /// Internal key: `enable_songFilter`
        const enable_songFilter  = 1 << 19;
        /// Internal key: `uncompleted_filter`
        const uncompleted_filter = 1 << 20;
        /// Internal key: `completed_filter`
        const completed_filter   = 1 << 21;
        /// Internal key: `featured_filter`
        const featured_filter    = 1 << 22;
        /// Internal key: `original_filter`
        const original_filter    = 1 << 23;
        /// Internal key: `twoP_filter`
        const TwoPlayer        = 1 << 24;
        /// Internal key: `nostar_filter`
        const NoStars      = 1 << 25;
        /// Internal key: `coin_filter`
        const Coins        = 1 << 26;
        /// Levels from creators that the player follows.
        ///
        /// Internal key: `follow_filter`
        const FollowedCreator      = 1 << 27;
        /// Levels from friends of this player.
        ///
        /// Internal key: `friend_filter`
        const Friends      = 1 << 28;
        /// Levels with a rating of epic.
        ///
        /// Internal key: `epic_filter`
        const RatingEpic        = 1 << 29;
        /// Internal key: `legendary_filter`
        const RatingLegendary   = 1 << 30;
    }
}

impl GDSearchFilters {
    /// Parses a dictionary from CCGameManager to search filters
    pub fn from_dict(d: &Dictionary) -> Option<Self> {
        None
    }
}
