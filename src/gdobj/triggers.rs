//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use std::fmt::Display;

use anyhow::anyhow;

use crate::gdobj::{
    Anim, ColourChannel, Event, GDObjConfig, GDObject, GDValue, Item, MoveEasing,
    ids::{
        objects::{
            BG_EFFECT_OFF, BG_EFFECT_ON, BG_SPEED_CONFIG, CAMERA_GUIDE, COLLISION_BLOCK,
            COLLISION_STATE_BLOCK, COUNTER, DISABLE_PLAYER_TRAIL, ENABLE_PLAYER_TRAIL,
            MG_SPEED_CONFIG, START_POS, TOGGLE_BLOCK, TRIGGER_ADVANCED_RANDOM, TRIGGER_ANIMATE,
            TRIGGER_AREA_STOP, TRIGGER_CAMERA_ZOOM, TRIGGER_COLLISION, TRIGGER_COLOUR,
            TRIGGER_COUNT, TRIGGER_END, TRIGGER_EVENT, TRIGGER_FOLLOW, TRIGGER_FOLLOW_PLAYER_Y,
            TRIGGER_GRAVITY, TRIGGER_ITEM_COMPARE, TRIGGER_ITEM_EDIT, TRIGGER_LINK_VISIBLE,
            TRIGGER_MIDDLEGROUND_CHANGE, TRIGGER_MIDDLEGROUND_CONFIG, TRIGGER_MOVE,
            TRIGGER_ON_DEATH, TRIGGER_PERSISTENT_ITEM, TRIGGER_PLAYER_CONTROL, TRIGGER_PULSE,
            TRIGGER_RANDOM, TRIGGER_RESET_GROUP, TRIGGER_REVERSE_GAMEPLAY, TRIGGER_ROTATION,
            TRIGGER_SCALE, TRIGGER_SHAKE, TRIGGER_SPAWN, TRIGGER_SPAWN_PARTICLE, TRIGGER_STOP,
            TRIGGER_TIME, TRIGGER_TIME_CONTROL, TRIGGER_TIME_EVENT, TRIGGER_TIME_WARP,
            TRIGGER_TOGGLE, TRIGGER_TOUCH, UI_CONFIG,
        },
        properties::{
            ACTIVATE_GROUP, ANIMATION_ID, BLENDING_ENABLED, BLUE, CAMERA_GUIDE_PREVIEW_OPACITY,
            CAMERA_ZOOM, CENTER_GROUP_ID, CLAIM_TOUCH, COLOUR_CHANNEL, COMPARE_OPERATOR,
            CONTROLLING_PLAYER_1, CONTROLLING_PLAYER_2, CONTROLLING_TARGET_PLAYER,
            COPY_COLOUR_FROM_CHANNEL, COPY_COLOUR_SPECS, COPY_OPACITY, COUNTER_ALIGNMENT,
            DIRECTIONAL_MODE_DISTANCE, DIRECTIONAL_MOVE_MODE, DISABLE_PREVIEW, DIV_BY_VALUE_X,
            DIV_BY_VALUE_Y, DONT_OVERRIDE, DURATION_GROUP_TRIGGER_CHANCE, DYNAMIC_BLOCK,
            DYNAMIC_MOVE, EASING_RATE, ENTEREXIT_TRANSITION_CONFIG, EVENT_EXTRA_ID,
            EVENT_EXTRA_ID_2, EVENT_LISTENERS, EXCLUSIVE_PULSE_MODE, FIRST_ITEM_TYPE,
            FOLLOW_CAMERAS_X_MOVEMENT, FOLLOW_CAMERAS_Y_MOVEMENT, FOLLOW_DELAY, FOLLOW_OFFSET,
            FOLLOW_PLAYERS_X_MOVEMENT, FOLLOW_PLAYERS_Y_MOVEMENT, FOLLOW_SPEED, GRAVITY, GREEN,
            IGNORE_TIMEWARP, INPUT_ITEM_1, INPUT_ITEM_2, INSTANT_END, IS_DISABLED, IS_INTERACTABLE,
            IS_TIMER, LEFT_OPERATOR, LEFT_ROUND_MODE, LEFT_SIGN_MODE, LOCK_OBJECT_ROTATION,
            MATCH_ROTATION_OF_SPAWNED_PARTICLES, MAX_FOLLOW_SPEED, MAXX_ID, MAXY_ID, MIDDLEGROUND,
            MINX_ID, MINY_ID, MODIFIER, MOVE_EASING, MOVE_UNITS_X, MOVE_UNITS_Y, MULTI_ACTIVATE,
            MULTIACTIVATABLE_TIME_EVENT, NEW_X_SCALE, NEW_Y_SCALE, NO_END_EFFECTS,
            NO_END_SOUND_EFFECTS, NO_LEGACY_HSV, ONLY_MOVE, OPACITY, PAUSE_AT_TARGET_TIME,
            PULSE_DETAIL_COLOUR_ONLY, PULSE_FADE_IN_TIME, PULSE_FADE_OUT_TIME, PULSE_GROUP,
            PULSE_HOLD_TIME, PULSE_MAIN_COLOUR_ONLY, RANDOM_PROBABILITIES_LIST, RED,
            RELATIVE_ROTATION, RELATIVE_SCALE, RESET_CAMERA, RESET_ITEM_TO_0, RESET_REMAP,
            REVERSE_GAMEPLAY, RIGHT_OPERATOR, RIGHT_ROUND_MODE, RIGHT_SIGN_MODE, ROTATE_DEGREES,
            ROTATE_GAMEPLAY, ROTATE_X360, ROTATION_OF_SPAWNED_PARTICLES, ROTATION_OFFSET,
            ROTATION_TARGET_ID, ROTATION_VARIATION_OF_SPAWNED_PARTICLES,
            SCALE_OF_SPAWNED_PARTICLES, SCALE_VARIATION_OF_SPAWNED_PARTICLES, SECOND_ITEM_TYPE,
            SECOND_MODIFIER, SECONDS_ONLY, SET_PERSISTENT_ITEM, SHAKE_INTERVAL, SHAKE_STRENGTH,
            SILENT_MOVE, SMALL_STEP, SPAWN_DELAY, SPAWN_DELAY_VARIATION, SPAWN_ID_REMAPS,
            SPAWN_ONLY, SPAWN_ORDERED, SPECIAL_COUNTER_MODE, START_PAUSED_TIMER, START_TIME,
            STARTING_GAMEMODE, STARTING_IN_DUAL_MODE, STARTING_IN_MINI_MODE,
            STARTING_IN_MIRROR_MODE, STARTING_SPEED, STOP_MODE, STOP_PLAYER_JUMP,
            STOP_PLAYER_MOVEMENT, STOP_PLAYER_ROTATION, STOP_PLAYER_SLIDING, STOP_TIME_COUNTER,
            TARGET_ALL_PERSISTENT_ITEMS, TARGET_CHANNEL, TARGET_COUNT, TARGET_ITEM, TARGET_ITEM_2,
            TARGET_ITEM_TYPE, TARGET_MOVE_MODE, TARGET_MOVE_MODE_AXIS_LOCK, TARGET_ORDER,
            TARGET_TIME, TARGET_TRANSITION_CHANNEL, TIME_VALUE_MULTIPLER, TIMER, TIMEWARP_AMOUNT,
            TOLERANCE, TOUCH_DUAL_MODE, TOUCH_HOLD_MODE, TOUCH_PLAYER_ONLY, TOUCH_TOGGLE_ONOFF,
            TRIGGER_ON_EXIT, USE_CONTROL_ID, USING_PLAYER_COLOUR_1, USING_PLAYER_COLOUR_2,
            X_MOVEMENT_MULTIPLIER, X_OFFSET_OF_SPAWNED_PARTICLES,
            X_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, X_REFERENCE_IS_RELATIVE, X_REFERENCE_POSITION,
            XAXIS_FOLLOW_MOD, Y_MOVEMENT_MULTIPLIER, Y_OFFSET_OF_SPAWNED_PARTICLES,
            Y_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, Y_REFERENCE_IS_RELATIVE, Y_REFERENCE_POSITION,
            YAXIS_FOLLOW_MOD,
        },
    },
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
/// Extra ID 2 parameter in the event trigger
pub enum ExtraID2 {
    #[default]
    All = 0,
    P1 = 1,
    P2 = 2,
}

/// Enum for move targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum MoveTarget {
    // Targets this group's parent object
    Group(i16),
    Player1,
    Player2,
}

/// Enum for the GD gamemodes corresponding to their internal values
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
pub enum Gamemode {
    #[default]
    Cube = 0,
    Ship = 1,
    Ball = 2,
    Ufo = 3,
    Wave = 4,
    Robot = 5,
    Spider = 6,
    Swing = 7,
}

/// Enum for stop trigger modes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum StopMode {
    Stop = 0,
    Pause = 1,
    Resume = 2,
}

/// Enum for item alignments
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ItemAlign {
    Center = 0,
    Left = 1,
    Right = 2,
}

/// Enum for transition object enter/exit config
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum TransitionMode {
    Both = 0,
    Enter = 1,
    Exit = 2,
}

/// Enum for transition object type (from top, from bottom, etc.)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
pub enum TransitionType {
    Fade = 22,
    FromBottom = 23,
    FromTop = 24,
    FromLeft = 25,
    FromRight = 26,
    ScaleIn = 27,
    ScaleOut = 28,
    Random = 55,
    AwayToLeft = 56,
    AwayToRight = 57,
    AwayFromMiddle = 58,
    TowardsMiddle = 59,
    #[default]
    None = 1915,
}

/// Enum for item operators
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Op {
    Set = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
}

/// Enum for item comparison operators
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum CompareOp {
    Equals = 0,
    Greater = 1,
    GreaterOrEquals = 2,
    Less = 3,
    LessOrEquals = 4,
    NotEquals = 5,
}

/// Compare operand configuration specifier for the item control trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompareOperand {
    /// Base item whose value is being used for the comparsion
    pub operand_item: Item,
    /// Multiplier
    pub modifier: f64,
    /// Operator between the item's value and modifier. Can only be `Op::Mul` or `Op::Div`
    pub mod_op: Op,
    /// Forces a specific rounding on the resulting value: See [`RoundMode`]
    pub rounding: RoundMode,
    /// Forces a specific sign on the resulting value: See [`SignMode`]
    pub sign: SignMode,
}

impl CompareOperand {
    /// Constructor for an operand that is simply a number literal.  
    /// Useful for comparing an [`Item`] against a number
    pub fn number_literal(num: f64) -> Self {
        Self {
            operand_item: Item::Counter(0),
            modifier: num,
            mod_op: Op::Mul,
            rounding: RoundMode::None,
            sign: SignMode::None,
        }
    }
}

impl From<Item> for CompareOperand {
    fn from(value: Item) -> Self {
        Self {
            operand_item: value,
            modifier: 1.0,
            mod_op: Op::Mul,
            rounding: RoundMode::None,
            sign: SignMode::None,
        }
    }
}

/// Enum for item round modes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum RoundMode {
    /// Leave as-is
    None = 0,
    Nearest = 1,
    Floor = 2,
    Ceiling = 3,
}

/// Enum for item sign modes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum SignMode {
    /// Leave as-is
    None = 0,
    Absolute = 1,
    Negative = 2,
}

/// Enum for target player in gravity trigger
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetPlayer {
    /// Player 1
    Player1 = 138,
    /// Player 2
    Player2 = 200,
    /// Player that touched the gravity trigger
    PlayerTarget = 201,
}

/// Enum for move mode setting. See structs [`DefaultMove`], [`TargetMove`], and [`DirectionalMove`]
#[derive(Debug, Clone, PartialEq)]
pub enum MoveMode {
    /// Normal axis-based move mode
    Default(DefaultMove),
    /// Moves the group to the position of another group
    Targeting(TargetMove),
    /// Moves the group in the direction of another group
    Directional(DirectionalMove),
}

/// Enum for lock config: player or camera
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum MoveLock {
    Player,
    Camera,
}

/// Enum for relative UI reference position
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum UIReferencePos {
    Auto = 1,
    Center = 2,
    Left = 3,
    Right = 4,
}

/// Config struct for default movement
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultMove {
    /// Units to move in x-axis. Used as multiplier of player/camera movement if `x_lock` is used
    pub dx: f64,
    /// Units to move in y-axis. Used as multiplier of player/camera movement if `y_lock` is used
    pub dy: f64,
    /// Optional lock on x movement which allows the object to move relative to either the player or the camera
    pub x_lock: Option<MoveLock>,
    /// Optional lock on y movement which allows the object to move relative to either the player or the camera
    pub y_lock: Option<MoveLock>,
}

/// Config struct for moving to a specific target.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetMove {
    /// Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
    pub target_group_id: MoveTarget,
    /// (Optional) The objects that represent the center of the group that is moving
    pub center_group_id: Option<i16>,
    /// Optional axis restriction. Use constants `MOVE_X_ONLY` and `MOVE_Y_ONLY` to specify axis.
    pub axis_only: Option<AxisOnlyMove>,
}

/// Optional axis lock for move triggers
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisOnlyMove {
    /// Locks to X-axis
    X = 1,
    /// Locks to Y-axis
    Y = 2,
}

/// Config struct for moving to a specific target.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalMove {
    /// Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
    pub target_group_id: MoveTarget,
    /// (Optional) The objects that represent the center of the group that is moving
    pub center_group_id: Option<i16>,
    /// Distance in units to move in the direction of the target objects.
    pub distance: i32,
}

/// Enum for starting speed in a startpos
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(missing_docs)]
pub enum StartingSpeed {
    X0Point5 = 1,
    #[default]
    X1 = 0,
    X2 = 2,
    X3 = 3,
    X4 = 4,
}

/// Config struct for HSV colour settings
#[derive(Debug, Clone, PartialEq)]
pub struct HSVColour {
    /// Hue shift
    pub hue_shift: i32,
    /// Saturation multiplier
    pub saturation_mult: f64,
    /// Brightness multiplier
    pub brightness_mult: f64,
    /// Use static saturation scalar
    pub static_sat_scalar: bool,
    /// Use static brightness scalar
    pub static_bright_scalar: bool,
}

impl Display for HSVColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}a{}a{}a{}a{}",
            self.hue_shift,
            self.saturation_mult,
            self.brightness_mult,
            if self.static_sat_scalar { "1" } else { "0" },
            if self.static_bright_scalar { "1" } else { "0" }
        )
    }
}

/// Enum for target of pulse
#[derive(Debug, Clone, PartialEq)]
pub enum PulseTarget {
    /// Pulse for a group
    Group(PulseGroup),
    /// Pulse for a channel
    Channel(PulseChannel),
}

/// Config struct for channel pulses
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseChannel {
    /// Channel which is pulsed
    pub channel_id: i16,
}

/// Config struct for group pulses
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PulseGroup {
    /// Group that is being pulsed
    pub group_id: i16,
    /// Toggles pulsing the main colour of objects only
    pub main_colour_only: bool,
    /// Toggles pulsing the detail colour of object only
    pub detail_colour_only: bool,
}

/// Enum for pulse mode
pub enum PulseMode {
    /// Pulse with colour
    Colour(Colour),
    /// Pulse with HSV
    HSV(PulseHSV),
}

/// RGB colour tuple
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Colour {
    /// Converts an RGB tuple to a [`Colour`]
    pub fn from_rgb(rgb: (u8, u8, u8)) -> Self {
        Self {
            red: rgb.0,
            green: rgb.1,
            blue: rgb.2,
        }
    }

    /// Parses a hex code (#123456) to a [`Colour`]
    pub fn from_hex<S: AsRef<str>>(hex_str: S) -> Result<Self, anyhow::Error> {
        let str = hex_str.as_ref();
        if str.len() != 7 || !str.starts_with('#') {
            return Err(anyhow!(
                "Hex code must start with a hashtag followed by a 6-digit RGB hex tuple."
            ));
        }

        let mut owned = str.to_string();
        owned.remove(0);

        let hex = i32::from_str_radix(&owned, 16)?;
        Ok(Self {
            red: (hex >> 16 & 0xFF) as u8,
            green: (hex >> 8 & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
        })
    }
}

/// Configuration struct for scaling of objects in the scale trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleConfig {
    /// Scale of target on x-axis
    pub x_scale: f64,
    /// Scale of target on y-axis
    pub y_scale: f64,
    /// Divides the x scale by the existing x-axis scale value of the target
    pub div_by_value_x: bool,
    /// Divides the y scale by the existing y-axis scale value of the target
    pub div_by_value_y: bool,
    /// Makes the objects only move as if they were scaled, but not actually scale them
    pub only_move: bool,
    /// Bases scaling on the center object
    pub relative_scale: bool,
    /// Rotates the x and y axes too
    pub relative_rotation: bool,
}

/// Pulse with HSV mode configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PulseHSV {
    /// HSV pulse specification
    pub hsv_config: HSVColour,
    /// Toggles using static HSV
    pub use_static_hsv: bool,
    /// Target of the pulse
    pub colour_id: ColourChannel,
}

/// Config struct for copy colour options
#[derive(Debug, Clone, PartialEq)]
pub struct CopyColourConfig {
    /// Original colour channel from which to copy colour
    pub original_ch: ColourChannel,
    /// HSV modifier for new colour
    pub hsv_config: HSVColour,
    /// Whether to apply legacy HSV transformation
    pub use_legacy_hsv: bool,
    /// Copy original colour's opacity
    pub copy_opacity: bool,
}

/// Enum for rotation configs
#[derive(Debug, Clone, PartialEq)]
pub enum RotationMode {
    /// Regular rotation based on a fixed degree amount
    Default(RotationNormal),
    /// Rotates the target objects to face a target group
    Aim(RotationAim),
    /// Follows a target object's rotation
    Follow(RotationAim),
}

/// Struct for specifying rotation settings in a rotate trigger
#[derive(Debug, Clone, PartialEq)]
pub struct RotationConfig {
    /// See [`RotationMode`]
    pub mode: RotationMode,
    /// Update location of aim group in real time]
    pub dynamic_mode: bool,
    /// Prevent target object from rotating around its center
    pub lock_object_rotation: bool,
}

/// Configuration struct for the time trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeTriggerConfig {
    /// Starting time of target timer that will be set on activation of the trigger
    pub start_time: f64,
    /// Time at which to call the target group
    pub stop_time: f64,
    /// Whether or not to pause the timer once it reaches the stop time
    pub pause_when_reached: bool,
    /// Time multiplier for this timer
    pub time_mod: f64,
    /// Target timer ID
    pub timer_id: i16,
    /// Toggles ignoring global timewarp
    pub ignore_timewarp: bool,
    /// Starts this timer paused, which allows a time control trigger to un-pause it
    pub start_paused: bool,
    /// Only starts the timer if any of these are met:
    ///     1. Target timer is at 0.00
    ///     2. The `start_paused` option is on
    ///     3. The timer is not currently counting
    pub dont_override: bool,
}

/// Degree amount rotation specifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RotationNormal {
    /// Amount of degrees
    pub degrees: f64,
    /// Amount of full rotations. Specifying these is preferred to specifying an overly large number of degrees.
    pub x360: i32,
}

impl RotationNormal {
    /// Convert from degrees to this object.
    pub fn from_degrees(deg: f64) -> Self {
        Self {
            degrees: deg % 360.0,
            x360: (deg as i32 / 360),
        }
    }
}

/// Optional target of rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum RotationPlayerTarget {
    Player1,
    Player2,
}

/// Config struct for aim mode rotation
#[derive(Debug, Clone, PartialEq)]
pub struct RotationAim {
    /// Group around which to rotate
    pub aim_target: i16,
    /// Rotation offset of the rotating group
    pub rot_offset: f64,
    ///  Overrides aim_target if not None, uses either P1 or P2 as the target instead.
    pub player_target: Option<RotationPlayerTarget>,
}

/// Configuration descriptor for spawning particles in particle spawn trigger
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ParticleSpawnConfig {
    /// (x, y) tuple for offsets from their original spawn location.
    ///   Note: all particle objects spawn in the same position, regardless of their offsets within their group.
    pub position_offsets: Option<(i32, i32)>,
    /// (x, y) tuple for range of possible random positional variation.
    pub position_variation: Option<(i32, i32)>,
    /// (rotation, variation) tuple that describes the rotation of the particles + random offset range
    pub rotation_config: Option<(i32, i32)>,
    /// (scale, variation) tuple that describes the scale of the particles + random offset range
    pub scale_config: Option<(f64, f64)>,
    /// Makes all of the particles in the group be rotated in the same direction.
    pub match_rotation: bool,
}

/// Gameplay starting settings specification struct for the startpos trigger
#[derive(Debug, Default, Clone, Copy)]
pub struct StartposConfig {
    /// Starting speed of player
    pub start_speed: StartingSpeed,
    /// Starting gamemode; Default: Cube
    pub starting_gamemode: Gamemode,
    /// Starting as mini? Default: false
    pub starting_as_mini: bool,
    /// Start as dual? Default: false
    pub starting_as_dual: bool,
    /// Start as mirrored? Default: false
    pub starting_mirrored: bool,
    /// Reset camera? Default: false
    pub reset_camera: bool,
    /// Rotate gameplay? Default: false
    pub rotate_gameplay: bool,
    /// Reverse gameplay? Default: false
    pub reverse_gameplay: bool,
}

/// Configuration struct for collide triggers which specifies the colliders
/// that are checked for collisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColliderConfig {
    /// ID of first collision block
    pub collider1: i16,
    /// ID of second collision block
    pub collider2: i16,
    /// Whether to check for collision with player 1 instead of collider 1
    pub collide_player1: bool,
    /// Whether to check for collision with player 2 instead of collider 1.
    ///   Does not override collision checking with player 1 if `collide_player1` is also true.
    pub collide_player2: bool,
    /// Whether to check for collision between the two players instead of two collision blocks
    pub collide_both_players: bool,
}

impl ColliderConfig {
    /// Creates a new instance of this object from two collision block IDs
    pub fn two_colliders(collider1_id: i16, collider2_id: i16) -> Self {
        Self {
            collider1: collider1_id,
            collider2: collider2_id,
            collide_player1: false,
            collide_player2: false,
            collide_both_players: false,
        }
    }
}

/// Configuration struct for the primary colour operation in a colour trigger
#[derive(Debug, Clone, Copy)]
pub struct ColourTriggerConfig {
    /// (R, G, B) tuple of `u8`s
    pub colour: Colour,
    /// Channel whose colour will be changed
    pub channel: ColourChannel,
    /// Opacity of colour
    pub opacity: f64,
    /// Use blending?
    pub blending: bool,
    /// Use player colour 1 instead of the specified colour.
    pub use_player_col_1: bool,
    /// Use player colour 2 instead of the specified colour.
    pub use_player_col_2: bool,
}

#[repr(i32)]
#[allow(missing_docs)]
/// Enum for middle grounds
pub enum MiddleGround {
    None = 0,
    SeasweptMountains = 1,
    RockyMountains = 2,
    Clouds = 3,
}

#[repr(i32)]
/// Enum for an optional player target. Used in the touch trigger
pub enum OptionalPlayerTarget {
    /// Registers input from both players
    None = 0,
    /// Only registers input from player 1.
    Player1 = 1,
    /// Only registers input from player 2.
    Player2 = 2,
}

#[repr(i32)]
/// Enum for modes of activation in a touch trigger
pub enum TouchToggle {
    /// Alternates between activating and deactivating the target group
    None = 0,
    /// Activates target group only
    ToggleOn = 1,
    /// De-activates target group only
    ToggleOff = 2,
}

/// Returns a move trigger object
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `move_config`: Details for the movement of the target group. See [`MoveMode`] struct
/// * `time`: Move time of group.
/// * `target_group`: Group that is moving.
/// * `silent`: Skips collision checking with the player(s) in the path of its motion. Useful for reducing lag. Collision blocks are unaffected.
/// * `dynamic`: Updates location of the target group in real time for target/directional move modes.
/// * `easing`: Optional easing and easing rate (default: 2) tuple.
pub fn move_trigger(
    config: &GDObjConfig,
    move_config: MoveMode,
    time: f64,
    target_group: i16,
    silent: bool,
    dynamic: bool,
    easing: Option<(MoveEasing, f64)>,
) -> GDObject {
    // aim: target group 2
    let mut properties = vec![
        (TARGET_ITEM, GDValue::Group(target_group)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(time)),
        (SMALL_STEP, GDValue::Bool(true)),
        (DYNAMIC_MOVE, GDValue::Bool(dynamic)),
        (SILENT_MOVE, GDValue::Bool(silent)),
    ];

    add_easing(&mut properties, easing);

    match move_config {
        MoveMode::Default(config) => {
            if let Some(lock) = config.x_lock {
                properties.push((
                    match lock {
                        MoveLock::Player => FOLLOW_PLAYERS_X_MOVEMENT,
                        MoveLock::Camera => FOLLOW_CAMERAS_X_MOVEMENT,
                    },
                    GDValue::Int(1),
                ));
                properties.push((X_MOVEMENT_MULTIPLIER, GDValue::Float(config.dx)));
            } else {
                properties.push((MOVE_UNITS_X, GDValue::Int(config.dx as i32)));
            }

            if let Some(lock) = config.y_lock {
                properties.push((
                    match lock {
                        MoveLock::Player => FOLLOW_PLAYERS_Y_MOVEMENT,
                        MoveLock::Camera => FOLLOW_CAMERAS_Y_MOVEMENT,
                    },
                    GDValue::Int(1),
                ));
                properties.push((Y_MOVEMENT_MULTIPLIER, GDValue::Float(config.dy)));
            } else {
                properties.push((MOVE_UNITS_Y, GDValue::Int(config.dy as i32)));
            }
        }
        MoveMode::Targeting(config) => {
            properties.push((TARGET_MOVE_MODE, GDValue::Int(1)));
            if let Some(id) = config.center_group_id {
                properties.push((CENTER_GROUP_ID, GDValue::Group(id)));
            }

            if let Some(axis) = config.axis_only {
                properties.push((TARGET_MOVE_MODE_AXIS_LOCK, GDValue::Int(axis as i32)));
            }

            match config.target_group_id {
                MoveTarget::Player1 => properties.push((CONTROLLING_PLAYER_1, GDValue::Int(1))),
                MoveTarget::Player2 => properties.push((CONTROLLING_PLAYER_2, GDValue::Int(1))),
                MoveTarget::Group(id) => properties.push((TARGET_ITEM_2, GDValue::Group(id))),
            };
        }
        MoveMode::Directional(config) => {
            if let Some(id) = config.center_group_id {
                properties.push((CENTER_GROUP_ID, GDValue::Group(id)));
            }

            match config.target_group_id {
                MoveTarget::Player1 => properties.push((CONTROLLING_PLAYER_1, GDValue::Int(1))),
                MoveTarget::Player2 => properties.push((CONTROLLING_PLAYER_2, GDValue::Int(1))),
                MoveTarget::Group(id) => properties.push((TARGET_ITEM_2, GDValue::Group(id))),
            };

            properties.push((DIRECTIONAL_MOVE_MODE, GDValue::Int(1)));
            properties.push((DIRECTIONAL_MODE_DISTANCE, GDValue::Int(config.distance)));
        }
    }

    GDObject::new(TRIGGER_MOVE, config, properties)
}

/// Returns a start pos object
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `gameplay_settings`: Gameplay options for startpos
/// * `target_order`: Target order (of what, I don't know); Default: 0
/// * `target_channel`: Target channel (once again, I don't know); Default: 0
/// * `disabled`: Disabled startpos? Default: false
pub fn start_pos(
    config: &GDObjConfig,
    gameplay_settings: StartposConfig,
    target_order: i32,
    target_channel: i32,
    disabled: bool,
) -> GDObject {
    GDObject::new(
        START_POS,
        config,
        vec![
            (
                STARTING_SPEED,
                GDValue::Int(gameplay_settings.start_speed as i32),
            ),
            (
                STARTING_GAMEMODE,
                GDValue::Int(gameplay_settings.starting_gamemode as i32),
            ),
            (
                STARTING_IN_MINI_MODE,
                GDValue::Bool(gameplay_settings.starting_as_mini),
            ),
            (
                STARTING_IN_DUAL_MODE,
                GDValue::Bool(gameplay_settings.starting_as_dual),
            ),
            (IS_DISABLED, GDValue::Bool(disabled)),
            (
                STARTING_IN_MIRROR_MODE,
                GDValue::Bool(gameplay_settings.starting_mirrored),
            ),
            (
                ROTATE_GAMEPLAY,
                GDValue::Bool(gameplay_settings.rotate_gameplay),
            ),
            (
                REVERSE_GAMEPLAY,
                GDValue::Bool(gameplay_settings.reverse_gameplay),
            ),
            (TARGET_ORDER, GDValue::Int(target_order)),
            (TARGET_CHANNEL, GDValue::Int(target_channel)),
            (RESET_CAMERA, GDValue::Bool(gameplay_settings.reset_camera)),
            (10010, GDValue::Int(0)),
            (10011, GDValue::String(String::new())),
            (10022, GDValue::Int(0)),
            (10023, GDValue::Int(0)),
            (10024, GDValue::Int(0)),
            (10027, GDValue::Int(1)),
            (10031, GDValue::Int(1)),
            (10032, GDValue::Int(1)),
            (10033, GDValue::Int(1)),
            (10034, GDValue::Int(1)),
            (10036, GDValue::Int(0)),
            (10037, GDValue::Int(1)),
            (10038, GDValue::Int(1)),
            (10039, GDValue::Int(1)),
            (10040, GDValue::Int(1)),
            (10041, GDValue::Int(1)),
            (10042, GDValue::Int(1)),
            (10043, GDValue::Int(0)),
            (10044, GDValue::Int(0)),
            (10045, GDValue::Int(1)),
            (10046, GDValue::Int(0)),
            (10009, GDValue::Int(1)),
        ],
    )
}

/// Returns a colour trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `fade_time`: Time to fade into the colour
/// * `copy_colour`: Optional [`CopyColourConfig`]
pub fn colour_trigger(
    config: &GDObjConfig,
    colour_cfg: ColourTriggerConfig,
    fade_time: f64,
    copy_colour: Option<CopyColourConfig>,
) -> GDObject {
    let mut properties = vec![
        (RED, GDValue::Int(colour_cfg.colour.red as i32)),
        (GREEN, GDValue::Int(colour_cfg.colour.green as i32)),
        (BLUE, GDValue::Int(colour_cfg.colour.blue as i32)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(fade_time)),
        (
            USING_PLAYER_COLOUR_1,
            GDValue::Bool(colour_cfg.use_player_col_1),
        ),
        (
            USING_PLAYER_COLOUR_2,
            GDValue::Bool(colour_cfg.use_player_col_2),
        ),
        (COLOUR_CHANNEL, GDValue::Short(colour_cfg.channel.into())),
        (OPACITY, GDValue::Float(colour_cfg.opacity)),
        (BLENDING_ENABLED, GDValue::Bool(colour_cfg.blending)),
    ];

    if let Some(config) = copy_colour {
        let cfg_string = config.hsv_config.to_string();
        if !config.use_legacy_hsv {
            properties.push((NO_LEGACY_HSV, GDValue::Bool(true)));
        }

        properties.push((COPY_OPACITY, GDValue::Bool(config.copy_opacity)));
        properties.push((COPY_COLOUR_SPECS, GDValue::String(cfg_string)));
        properties.push((
            COPY_COLOUR_FROM_CHANNEL,
            GDValue::ColourChannel(config.original_ch),
        ));
    }

    GDObject::new(TRIGGER_COLOUR, config, properties)
}

/// Returns a pulse trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `pulse_fade_in_time`: fade-in time of the pulse in seconds  
/// * `pulse_hold_time`: gold time of the pulse in seconds  
/// * `pulse_fade_out_time`: fade-out time of the pulse in seconds  
/// * `exclusive_pulse`: disable all other pulses of the same ID when this trigger is activated
/// * `pulse_target`: Target group/channel of pulse. See [`PulseTarget`]
/// * `pulse_mode`: Colour settings of this pulse. See [`PulseMode`]
pub fn pulse_trigger(
    config: &GDObjConfig,
    pulse_fade_in_time: f64,
    pulse_hold_time: f64,
    pulse_fade_out_time: f64,
    exclusive_pulse: bool,
    pulse_target: PulseTarget,
    pulse_mode: PulseMode,
) -> GDObject {
    let mut properties = vec![
        (PULSE_FADE_IN_TIME, GDValue::Float(pulse_fade_in_time)),
        (PULSE_HOLD_TIME, GDValue::Float(pulse_hold_time)),
        (PULSE_FADE_OUT_TIME, GDValue::Float(pulse_fade_out_time)),
        (EXCLUSIVE_PULSE_MODE, GDValue::Bool(exclusive_pulse)),
    ];
    match pulse_target {
        PulseTarget::Channel(c) => properties.push((TARGET_ITEM, GDValue::Group(c.channel_id))),
        PulseTarget::Group(g) => {
            properties.extend_from_slice(&[
                (
                    PULSE_DETAIL_COLOUR_ONLY,
                    GDValue::Bool(g.detail_colour_only),
                ),
                (PULSE_MAIN_COLOUR_ONLY, GDValue::Bool(g.main_colour_only)),
                (PULSE_GROUP, GDValue::Group(g.group_id)),
            ]);
        }
    }

    match pulse_mode {
        PulseMode::Colour(c) => {
            properties.extend_from_slice(&[
                (RED, GDValue::Int(c.red as i32)),
                (GREEN, GDValue::Int(c.green as i32)),
                (BLUE, GDValue::Int(c.blue as i32)),
            ]);
        }
        PulseMode::HSV(h) => {
            properties.extend_from_slice(&[
                (NO_LEGACY_HSV, GDValue::Bool(h.use_static_hsv)),
                (COPY_COLOUR_SPECS, GDValue::String(h.hsv_config.to_string())),
                (
                    COPY_COLOUR_FROM_CHANNEL,
                    GDValue::ColourChannel(h.colour_id),
                ),
            ]);
        }
    }
    GDObject::new(TRIGGER_PULSE, config, properties)
}

/// Returns a stop trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `stop_mode`: Stop mode (see [`StopMode`] struct)
/// * `use_control_id`: Only stops certain triggers within a group if enabled.
#[inline(always)]
pub fn stop_trigger(
    config: &GDObjConfig,
    target_group: i16,
    stop_mode: StopMode,
    use_control_id: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_STOP,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (USE_CONTROL_ID, GDValue::Bool(use_control_id)),
            (STOP_MODE, GDValue::Int(stop_mode as i32)),
        ],
    )
}

/// Returns an alpha trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `opacity`: Opacity to set group at
/// * `fade_time`: Time to fade to the opacity
#[inline(always)]
pub fn alpha_trigger(
    config: &GDObjConfig,
    target_group: i16,
    opacity: f64,
    fade_time: f64,
) -> GDObject {
    GDObject::new(
        1007,
        config,
        vec![
            (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(fade_time)),
            (OPACITY, GDValue::Float(opacity)),
            (TARGET_ITEM, GDValue::Group(target_group)),
        ],
    )
}

/// Returns a toggle trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `activate_group`: Active group instead of deactivating?
#[inline(always)]
pub fn toggle_trigger(config: &GDObjConfig, target_group: i16, activate_group: bool) -> GDObject {
    GDObject::new(
        TRIGGER_TOGGLE,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (ACTIVATE_GROUP, GDValue::Bool(activate_group)),
        ],
    )
}

/// Returns a transition object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `transition`: Type of transition. See [`TransitionType`] struct
/// * `mode`: Mode for transition (enter/exit only). See [`TransitionMode`] struct
/// * `target_channel`: Optional target channel argument which specifies a channel for this transition.
pub fn transition_object(
    config: &GDObjConfig,
    transition: TransitionType,
    mode: TransitionMode,
    target_channel: Option<i32>,
) -> GDObject {
    let mut properties = vec![];

    if mode != TransitionMode::Both {
        properties.push((ENTEREXIT_TRANSITION_CONFIG, GDValue::Int(mode as i32)));
    }
    if let Some(channel) = target_channel {
        properties.push((TARGET_TRANSITION_CHANNEL, GDValue::Int(channel)));
    }

    GDObject::new(transition as i32, config, properties)
}

// misc stuff

/// Returns a reverse gameplay trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn reverse_gameplay(config: &GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_REVERSE_GAMEPLAY, config, vec![])
}

/// Returns a link visible trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is linked visibly
#[inline(always)]
pub fn link_visible(config: &GDObjConfig, target_group: i16) -> GDObject {
    GDObject::new(
        TRIGGER_LINK_VISIBLE,
        config,
        vec![(TARGET_ITEM, GDValue::Group(target_group))],
    )
}

/// Returns a timewarp trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `time_scale`: How much to speed up/slow down time by. 1.0 is the default
#[inline(always)]
pub fn timewarp(config: &GDObjConfig, time_scale: f64) -> GDObject {
    GDObject::new(
        TRIGGER_TIME_WARP,
        config,
        vec![(TIMEWARP_AMOUNT, GDValue::Float(time_scale))],
    )
}

/// Returns a trigger that shows the player
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn show_player(config: &GDObjConfig) -> GDObject {
    GDObject::new(1613, config, vec![])
}

/// Returns a trigger that hides the player
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn hide_player(config: &GDObjConfig) -> GDObject {
    GDObject::new(1612, config, vec![])
}

/// Returns a trigger that shows the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn show_player_trail(config: &GDObjConfig) -> GDObject {
    GDObject::new(ENABLE_PLAYER_TRAIL, config, vec![])
}

/// Returns a trigger that hides the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale\
#[inline(always)]
pub fn hide_player_trail(config: &GDObjConfig) -> GDObject {
    GDObject::new(DISABLE_PLAYER_TRAIL, config, vec![])
}

/// Returns a trigger that enables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn bg_effect_on(config: &GDObjConfig) -> GDObject {
    GDObject::new(BG_EFFECT_ON, config, vec![])
}

/// Returns a trigger that disables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn bg_effect_off(config: &GDObjConfig) -> GDObject {
    GDObject::new(BG_EFFECT_OFF, config, vec![])
}

/// Returns a group reset trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is to be reset
#[inline(always)]
pub fn group_reset(config: &GDObjConfig, target_group: i16) -> GDObject {
    GDObject::new(
        TRIGGER_RESET_GROUP,
        config,
        vec![(TARGET_ITEM, GDValue::Group(target_group))],
    )
}

/// Returns a shake trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `strength`: Strength of shake
/// * `interval`: Interval in seconds between each shake
/// * `duration`: Total duration of shaking
#[inline(always)]
pub fn shake_trigger(
    config: &GDObjConfig,
    strength: i32,
    interval: f64,
    duration: f64,
) -> GDObject {
    GDObject::new(
        TRIGGER_SHAKE,
        config,
        vec![
            (SHAKE_STRENGTH, GDValue::Int(strength)),
            (SHAKE_INTERVAL, GDValue::Float(interval)),
            (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(duration)),
        ],
    )
}

/// Returns a background speed config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `mod_x`: X-axis speed of BG in terms of player speed. Default is 0.3
/// * `mod_y`: Y-axis speed of BG in terms of player speed. Default is 0.5
#[inline(always)]
pub fn bg_speed(config: &GDObjConfig, mod_x: f64, mod_y: f64) -> GDObject {
    GDObject::new(
        BG_SPEED_CONFIG,
        config,
        vec![
            (X_MOVEMENT_MULTIPLIER, GDValue::Float(mod_x)),
            (Y_MOVEMENT_MULTIPLIER, GDValue::Float(mod_y)),
        ],
    )
}

/// Returns a middleground speed config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `mod_x`: X-axis speed of MG in terms of player speed. Default is 0.3
/// * `mod_y`: Y-axis speed of MG in terms of player speed. Default is 0.5
#[inline(always)]
pub fn mg_speed(config: &GDObjConfig, mod_x: f64, mod_y: f64) -> GDObject {
    GDObject::new(
        MG_SPEED_CONFIG,
        config,
        vec![
            (X_MOVEMENT_MULTIPLIER, GDValue::Float(mod_x)),
            (Y_MOVEMENT_MULTIPLIER, GDValue::Float(mod_y)),
        ],
    )
}

/// Returns a player control trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `p1`: Enables these controls for player 1
/// * `p2`: Enables these controls for player 2
/// * `stop_jump`: Cancel's the player's current jump
/// * `stop_move`: Stops the player from moving
/// * `stop_rotation`: Stops the player's rotation
/// * `stop_slide`: Stops the player from sliding after a force
#[inline(always)]
pub fn player_control(
    config: &GDObjConfig,
    p1: bool,
    p2: bool,
    stop_jump: bool,
    stop_move: bool,
    stop_rotation: bool,
    stop_slide: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_PLAYER_CONTROL,
        config,
        vec![
            (CONTROLLING_PLAYER_1, GDValue::Bool(p1)),
            (CONTROLLING_PLAYER_2, GDValue::Bool(p2)),
            (STOP_PLAYER_JUMP, GDValue::Bool(stop_jump)),
            (STOP_PLAYER_MOVEMENT, GDValue::Bool(stop_move)),
            (STOP_PLAYER_ROTATION, GDValue::Bool(stop_rotation)),
            (STOP_PLAYER_SLIDING, GDValue::Bool(stop_slide)),
        ],
    )
}

/// Returns a gravity trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `gravity`: how much gravity.
/// * `target_player`: (Optional) Player target for this gravity trigger
pub fn gravity_trigger(
    config: &GDObjConfig,
    gravity: f64,
    target_player: Option<TargetPlayer>,
) -> GDObject {
    let mut properties = vec![(GRAVITY, GDValue::Float(gravity))];

    if let Some(player) = target_player {
        properties.push((player as u16, GDValue::Bool(true)));
    }
    GDObject::new(TRIGGER_GRAVITY, config, properties)
}

/// Returns an end trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `spawn_id`: Optional group to spawn once this trigger is activated
/// * `target_pos`: Optional target end position group
/// * `no_effects`: Disables visual end effects
/// * `instant`: Teleoprts the player instead of doing the default end pull animation
/// * `no_sfx`: Disables end sound effects
pub fn end_trigger(
    config: &GDObjConfig,
    spawn_id: Option<i16>,
    target_pos: Option<i16>,
    no_effects: bool,
    instant: bool,
    no_sfx: bool,
) -> GDObject {
    let mut properties = vec![
        (NO_END_EFFECTS, GDValue::Bool(no_effects)),
        (INSTANT_END, GDValue::Bool(instant)),
        (NO_END_SOUND_EFFECTS, GDValue::Bool(no_sfx)),
    ];

    if let Some(id) = spawn_id {
        properties.push((TARGET_ITEM, GDValue::Group(id)));
    }

    if let Some(pos) = target_pos {
        properties.push((TARGET_ITEM_2, GDValue::Group(pos)));
    }

    GDObject::new(TRIGGER_END, config, properties)
}

// items and counters

/// Returns a counter object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `item_id`: ID of the counter
/// * `timer`: Is a timer?
/// * `align`: Visual alignment of counter object. See [`ItemAlign`] struct.
/// * `seconds_only`: Show only seconds if timer?
/// * `special_mode`: Other special mode of timer. See CounterMode struct.
pub fn counter_object(
    config: &GDObjConfig,
    item: Item,
    align: ItemAlign,
    seconds_only: bool,
) -> GDObject {
    let mut properties = vec![
        (SECONDS_ONLY, GDValue::Bool(seconds_only)),
        (COUNTER_ALIGNMENT, GDValue::Int(align as i32)),
    ];

    match item {
        Item::Attempts | Item::MainTime | Item::Points => {
            properties.push((
                SPECIAL_COUNTER_MODE,
                GDValue::Int(item.as_special_mode_i32()),
            ));
        }
        Item::Counter(c) => {
            properties.push((INPUT_ITEM_1, GDValue::Item(c)));
        }
        Item::Timer(t) => {
            properties.extend_from_slice(&[
                (INPUT_ITEM_1, GDValue::Item(t)),
                (IS_TIMER, GDValue::Bool(true)),
            ]);
        }
    }

    GDObject::new(COUNTER, config, properties)
}

/// Returns an item edit trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `operand1`: Optional first operand, tuple of (ID, item type)
/// * `operand2`: Optional second operand, tuple of (ID, item type)
/// * `target_id`: Target item id
/// * `target_type`: Target item type
/// * `modifier`: f64 modifier; default is 1.0
/// * `assign_op`: operator for modifying target; see [`Op`] enum. An `Op::Add` is equivalent to `+=`.
/// * `multiply_mod`: whether the result between operands should be multiplied or divided by the mod.
/// * `id_op`: operator between operands 1 and 2; see [`Op`] enum.
/// * `id_rounding`: rounding mode of the result after both operands are evaluated; see [`RoundMode`] enum.
/// * `result_rounding`: rounding mode of the final result; see [`RoundMode`] enum.
/// * `id_sign`: sign mode of the result after both operands are evaluated; see [`SignMode`] enum.
/// * `result_sign`: sign mode of the final result; see [`SignMode`] enum.
#[allow(clippy::too_many_arguments)]
pub fn item_edit(
    config: &GDObjConfig,
    operand1: Option<Item>,
    operand2: Option<Item>,
    target: Item,
    modifier: f64,
    assign_op: Op,
    multiply_mod: bool,
    id_op: Option<Op>,
    id_rounding: RoundMode,
    result_rounding: RoundMode,
    id_sign: SignMode,
    result_sign: SignMode,
) -> GDObject {
    // set default values
    let mod_op = match multiply_mod {
        true => Op::Mul,
        false => Op::Div,
    };
    let id_op = match id_op {
        Some(op) => op,
        None => Op::Add,
    };

    let mut properties = vec![
        (TARGET_ITEM, GDValue::Item(target.id())),
        (TARGET_ITEM_TYPE, GDValue::Int(target.get_type_as_i32())),
        (MODIFIER, GDValue::Float(modifier)),
        (LEFT_OPERATOR, GDValue::Int(assign_op as i32)),
        (RIGHT_OPERATOR, GDValue::Int(id_op as i32)),
        (COMPARE_OPERATOR, GDValue::Int(mod_op as i32)),
        (LEFT_ROUND_MODE, GDValue::Int(id_rounding as i32)),
        (RIGHT_ROUND_MODE, GDValue::Int(result_rounding as i32)),
        (LEFT_SIGN_MODE, GDValue::Int(id_sign as i32)),
        (RIGHT_SIGN_MODE, GDValue::Int(result_sign as i32)),
    ];

    if let Some(item) = operand1 {
        properties.extend_from_slice(&[
            (INPUT_ITEM_1, GDValue::Item(item.id())),
            (FIRST_ITEM_TYPE, GDValue::Int(item.get_type_as_i32())),
        ]);
    }

    if let Some(item) = operand2 {
        properties.extend_from_slice(&[
            (INPUT_ITEM_2, GDValue::Item(item.id())),
            (SECOND_ITEM_TYPE, GDValue::Int(item.get_type_as_i32())),
        ]);
    }

    GDObject::new(TRIGGER_ITEM_EDIT, config, properties)
}

/// Returns an item compare trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `true_id`: Group that is activated when the comparison is true
/// * `false_id`: Group that is activated when the comparison is false
/// * \*`lhs`: [`CompareOperand`] config struct for left-hand side operand.
/// * \*`rhs`: [`CompareOperand`] config struct for right-hand side operand.
/// * `compare_op`: Operator used to compare the two sides. See [`CompareOp`] enum.
/// * `tolerance`: Tolerant range of comparsion. Comparsion will be true if the absolute resulting value is less than or equal to the tolerance.
///
/// \* The modifier operators describe how the modifier interacts with the item, except for setting the item
///
/// The modifier is applied to each respective operand according to the specified modifier operator.
/// The round and sign modes are applied at the end of evaluation to each operand.
/// The right-hand side will be just the modifier if the item id is left as 0 (not specified).
/// This is useful when it is necessary to compare an item value and an integer or float literal.
pub fn item_compare(
    config: &GDObjConfig,
    true_id: i16,
    false_id: i16,
    lhs: CompareOperand,
    rhs: CompareOperand,
    compare_op: CompareOp,
    tolerance: f64,
) -> GDObject {
    let properties = vec![
        (TARGET_ITEM, GDValue::Item(true_id)),
        (TARGET_ITEM_2, GDValue::Item(false_id)),
        // ids
        (INPUT_ITEM_1, GDValue::Item(lhs.operand_item.id())),
        (INPUT_ITEM_2, GDValue::Item(rhs.operand_item.id())),
        // types
        (
            FIRST_ITEM_TYPE,
            GDValue::Int(lhs.operand_item.get_type_as_i32()),
        ),
        (
            SECOND_ITEM_TYPE,
            GDValue::Int(rhs.operand_item.get_type_as_i32()),
        ),
        // modifiers
        (MODIFIER, GDValue::Float(lhs.modifier)),
        (SECOND_MODIFIER, GDValue::Float(rhs.modifier)),
        // modifiers ops
        (LEFT_OPERATOR, GDValue::Int(lhs.mod_op as i32)),
        (RIGHT_OPERATOR, GDValue::Int(rhs.mod_op as i32)),
        (COMPARE_OPERATOR, GDValue::Int(compare_op as i32)),
        (TOLERANCE, GDValue::Float(tolerance)),
        // round modes
        (LEFT_ROUND_MODE, GDValue::Int(lhs.rounding as i32)),
        (RIGHT_ROUND_MODE, GDValue::Int(rhs.rounding as i32)),
        // sign modes
        (LEFT_SIGN_MODE, GDValue::Int(lhs.sign as i32)),
        (RIGHT_SIGN_MODE, GDValue::Int(rhs.sign as i32)),
    ];

    GDObject::new(TRIGGER_ITEM_COMPARE, config, properties)
}

/// Returns a persistent item trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `item_id`: Target item ID
/// * `timer`: Targets a timer with the corresponding ID if enabled
/// * `persistent`: make this item persistent?
/// * `target_all`: Target all persistent items?
/// * `reset`: Reset item(s) to 0?
pub fn persistent_item(
    config: &GDObjConfig,
    item_id: i16,
    timer: bool,
    persistent: bool,
    target_all: bool,
    reset: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_PERSISTENT_ITEM,
        config,
        vec![
            (TARGET_ITEM, GDValue::Item(item_id)),
            (SET_PERSISTENT_ITEM, GDValue::Bool(persistent)),
            (TARGET_ALL_PERSISTENT_ITEMS, GDValue::Bool(target_all)),
            (RESET_ITEM_TO_0, GDValue::Bool(reset)),
            (TIMER, GDValue::Bool(timer)),
        ],
    )
}

// spawners

/// Returns a random trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `chance`: chance to trigger group 1
/// * `target_group1`: target group 1
/// * `target_group1`: target group 2
pub fn random_trigger(
    config: &GDObjConfig,
    chance: f64,
    target_group1: i16,
    target_group2: i16,
) -> GDObject {
    GDObject::new(
        TRIGGER_RANDOM,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group1)),
            (TARGET_ITEM_2, GDValue::Group(target_group2)),
            (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(chance)),
        ],
    )
}

/// Returns a spawn trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `spawn_id`: Spawns this group
/// * `delay`: Delay between beign triggered and spawning the group
/// * `delay_variation`: Random variation on delay
/// * `reset_remap`: Resets the remapping of group IDs
/// * `spawn_ordered`: Spawns constituents of group in the order of x-position
/// * `preview_disable`: prevents the trigger's resulting spawns from being rendered in editor preview
pub fn spawn_trigger(
    config: &GDObjConfig,
    spawn_id: i16,
    delay: f64,
    delay_variation: f64,
    reset_remap: bool,
    spawn_ordered: bool,
    preview_disable: bool,
    spawn_remap: Vec<(i16, i16)>,
) -> GDObject {
    GDObject::new(
        TRIGGER_SPAWN,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(spawn_id)),
            (SPAWN_DELAY, GDValue::Float(delay)),
            (DISABLE_PREVIEW, GDValue::Bool(preview_disable)),
            (SPAWN_ORDERED, GDValue::Bool(spawn_ordered)),
            (SPAWN_DELAY_VARIATION, GDValue::Float(delay_variation)),
            (RESET_REMAP, GDValue::Bool(reset_remap)),
            (SPAWN_ID_REMAPS, GDValue::from_spawn_remaps(spawn_remap)),
        ],
    )
}

/// Returns an on-death trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Spawns this group
/// * `activate_group`: Activate this group (instead of toggling off)?
#[inline(always)]
pub fn on_death(config: &GDObjConfig, target_group: i16, activate_group: bool) -> GDObject {
    GDObject::new(
        TRIGGER_ON_DEATH,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (ACTIVATE_GROUP, GDValue::Bool(activate_group)),
        ],
    )
}

/// Returns a particle spawner trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `particle_group`: Group that contains the particle objects
/// * `position_group`: Group at which the particles will be spawned
/// * `spawn_cfg`: Spawning configure for the particles themselves. See [`ParticleSpawnConfig`]
pub fn spawn_particle(
    config: &GDObjConfig,
    particle_group: i16,
    position_group: i16,
    spawn_cfg: ParticleSpawnConfig,
) -> GDObject {
    let mut properties = vec![
        (TARGET_ITEM, GDValue::Group(particle_group)),
        (TARGET_ITEM_2, GDValue::Group(position_group)),
        (
            MATCH_ROTATION_OF_SPAWNED_PARTICLES,
            GDValue::Bool(spawn_cfg.match_rotation),
        ),
    ];

    if let Some((x, y)) = spawn_cfg.position_offsets {
        properties.push((X_OFFSET_OF_SPAWNED_PARTICLES, GDValue::Int(x)));
        properties.push((Y_OFFSET_OF_SPAWNED_PARTICLES, GDValue::Int(y)));
    }

    if let Some((x, y)) = spawn_cfg.position_variation {
        properties.push((X_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(x)));
        properties.push((Y_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(y)));
    }

    if let Some((rot, var)) = spawn_cfg.rotation_config {
        properties.push((ROTATION_OF_SPAWNED_PARTICLES, GDValue::Int(rot)));
        properties.push((ROTATION_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(var)));
    }

    if let Some((scale, var)) = spawn_cfg.scale_config {
        properties.push((SCALE_OF_SPAWNED_PARTICLES, GDValue::Float(scale)));
        properties.push((SCALE_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Float(var)));
    }

    GDObject::new(TRIGGER_SPAWN_PARTICLE, config, properties)
}

// collision blocks

/// Returns a collision block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Collision block ID
/// * `dynamic`: Does this block register collisions with other collision blocks?
#[inline(always)]
pub fn collision_block(config: &GDObjConfig, id: i16, dynamic: bool) -> GDObject {
    GDObject::new(
        COLLISION_BLOCK,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Item(id)),
            (DYNAMIC_BLOCK, GDValue::Bool(dynamic)),
        ],
    )
}

/// Returns a toggle block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Spawns this group
/// * `activate_group`: Activate/spawn group instead of deactivating?
/// * `claim_touch`: Disable buffer clicking?
/// * `multi_activate`: Allow multiple activations?
/// * `spawn_only`: Spawn only without toggling?
#[inline(always)]
pub fn toggle_block(
    config: &GDObjConfig,
    target_group: i16,
    activate_group: bool,
    claim_touch: bool,
    multi_activate: bool,
    spawn_only: bool,
) -> GDObject {
    GDObject::new(
        TOGGLE_BLOCK,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (ACTIVATE_GROUP, GDValue::Bool(activate_group)),
            (MULTI_ACTIVATE, GDValue::Bool(multi_activate)),
            (CLAIM_TOUCH, GDValue::Bool(claim_touch)),
            (SPAWN_ONLY, GDValue::Bool(spawn_only)),
        ],
    )
}

/// Returns a state block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `state_on`: Group that is activated when the player enters this block's hitbox
/// * `state_off`: Group that is activated when the player exits this block's hitbox
#[inline(always)]
pub fn state_block(config: &GDObjConfig, state_on: i16, state_off: i16) -> GDObject {
    GDObject::new(
        COLLISION_STATE_BLOCK,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(state_on)),
            (TARGET_ITEM_2, GDValue::Group(state_off)),
        ],
    )
}

/// Returns a collision trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `collider_cfg`: Settings for colliders for this collision detection. See [`ColliderConfig`]
/// * `target_id`: ID of group that is activated when the two colliders collide
/// * `activate_group`: whether this trigger will activate or deactivate the target group
/// * `trigger_on_exit`: activates group when the two colliders' hitboxes stop overlapping after collision
///   instead of when they start colliding.
///
/// **Note**: At least one of the collider blocks must be dynamic for this collision to register.
#[inline(always)]
pub fn collision_trigger(
    config: &GDObjConfig,
    collider_cfg: ColliderConfig,
    target_id: i16,
    activate_group: bool,
    trigger_on_exit: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_COLLISION,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Item(collider_cfg.collider1)),
            (INPUT_ITEM_2, GDValue::Item(collider_cfg.collider2)),
            (TARGET_ITEM, GDValue::Item(target_id)),
            (
                CONTROLLING_PLAYER_1,
                GDValue::Bool(collider_cfg.collide_player1),
            ),
            (
                CONTROLLING_PLAYER_2,
                GDValue::Bool(collider_cfg.collide_player2),
            ),
            (
                CONTROLLING_TARGET_PLAYER,
                GDValue::Bool(collider_cfg.collide_both_players),
            ),
            (ACTIVATE_GROUP, GDValue::Bool(activate_group)),
            (TRIGGER_ON_EXIT, GDValue::Bool(trigger_on_exit)),
        ],
    )
}

/// Returns a collision trigger
///
/// Activates a group when the two colliders collide or do not collide.
/// This condition is only checked once and never again
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `collider_cfg`: Settings for colliders for this collision detection. See [`ColliderConfig`]
/// * `true_id`: ID of group that is activated if the two colliders collide
/// * `false_id`: ID of group that is activated if the two colliders do not collide
#[inline(always)]
pub fn instant_coll_trigger(
    config: &GDObjConfig,
    collider_cfg: ColliderConfig,
    true_id: i16,
    false_id: i16,
) -> GDObject {
    GDObject::new(
        TRIGGER_COLLISION,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Item(collider_cfg.collider1)),
            (INPUT_ITEM_2, GDValue::Item(collider_cfg.collider2)),
            (TARGET_ITEM, GDValue::Item(true_id)),
            (TARGET_ITEM_2, GDValue::Item(false_id)),
            (
                CONTROLLING_PLAYER_1,
                GDValue::Bool(collider_cfg.collide_player1),
            ),
            (
                CONTROLLING_PLAYER_2,
                GDValue::Bool(collider_cfg.collide_player2),
            ),
            (
                CONTROLLING_TARGET_PLAYER,
                GDValue::Bool(collider_cfg.collide_both_players),
            ),
        ],
    )
}

// time triggers

/// Returns a time trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `time_cfg`: Main trigger configuration. See [`TimeTriggerConfig`].
/// * `target_group`: Group that is activated when the timer reaches the target value
pub fn time_trigger(
    config: &GDObjConfig,
    time_cfg: TimeTriggerConfig,
    target_group: i16,
) -> GDObject {
    GDObject::new(
        TRIGGER_TIME,
        config,
        vec![
            (START_TIME, GDValue::Float(time_cfg.start_time)),
            (TARGET_TIME, GDValue::Float(time_cfg.stop_time)),
            (
                PAUSE_AT_TARGET_TIME,
                GDValue::Bool(time_cfg.pause_when_reached),
            ),
            (TIME_VALUE_MULTIPLER, GDValue::Float(time_cfg.time_mod)),
            (INPUT_ITEM_1, GDValue::Item(time_cfg.timer_id)),
            (TARGET_ITEM, GDValue::Group(target_group)),
            (IGNORE_TIMEWARP, GDValue::Bool(time_cfg.ignore_timewarp)),
            (START_PAUSED_TIMER, GDValue::Bool(time_cfg.start_paused)),
            (DONT_OVERRIDE, GDValue::Bool(time_cfg.dont_override)),
        ],
    )
}

/// Returns a time control trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Timer ID
/// * `stop`: If enabled, stops the timer; otherwise, starts the timer.
#[inline(always)]
pub fn time_control(config: &GDObjConfig, id: i16, stop: bool) -> GDObject {
    GDObject::new(
        TRIGGER_TIME_CONTROL,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Item(id)),
            (STOP_TIME_COUNTER, GDValue::Bool(stop)),
        ],
    )
}

/// Returns a time event trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Timer ID
/// * `target_group`: If enabled, stops the timer; otherwise, starts the timer.
/// * `target_time`: At what time the timer should be to activate objects in `target_group`.
/// * `multi_activate`: Should this event be triggerable multiple times?
#[inline(always)]
pub fn time_event(
    config: &GDObjConfig,
    id: i16,
    target_group: i16,
    target_time: f64,
    multi_activate: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_TIME_EVENT,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Group(id)),
            (TARGET_ITEM, GDValue::Group(target_group)),
            (TARGET_TIME, GDValue::Float(target_time)),
            (MULTIACTIVATABLE_TIME_EVENT, GDValue::Bool(multi_activate)),
        ],
    )
}

// camera triggers

/// Returns a camera zoom trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `zoom`: Resulting camera zoom. Default is 1.0
/// * `time`: Time to zoom
/// * `easing`: Zoom easing config. See [`MoveEasing`] struct.
pub fn camera_zoom(
    config: &GDObjConfig,
    zoom: f64,
    time: f64,
    easing: Option<(MoveEasing, f64)>,
) -> GDObject {
    let mut properties = vec![
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(time)),
        (CAMERA_ZOOM, GDValue::Float(zoom)),
    ];

    if let Some((easing, rate)) = easing {
        properties.push((MOVE_EASING, GDValue::Int(easing as i32)));
        properties.push((EASING_RATE, GDValue::Float(rate)));
    }
    GDObject::new(TRIGGER_CAMERA_ZOOM, config, properties)
}

/// Returns a camera guide object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `zoom`: Zoom of camera guide
/// * `offset_x`: Center offset from this object in x axis
/// * `offset_y`: Center offset from this object in y axis
/// * `opacity`: Opacity of guidelines
#[inline(always)]
pub fn camera_guide(
    config: &GDObjConfig,
    zoom: f64,
    offset_x: i32,
    offset_y: i32,
    opacity: f64,
) -> GDObject {
    GDObject::new(
        CAMERA_GUIDE,
        config,
        vec![
            (MOVE_UNITS_X, GDValue::Int(offset_x)),
            (MOVE_UNITS_Y, GDValue::Int(offset_y)),
            (CAMERA_ZOOM, GDValue::Float(zoom)),
            (CAMERA_GUIDE_PREVIEW_OPACITY, GDValue::Float(opacity)),
        ],
    )
}

// im too lazy to organise ts

/// Returns a follow trigger object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `x_mod`: Multiplier for x-axis movement of follow group
/// * `y_mod`: Multiplier for y-axis movement of follow group
/// * `follow_time`: Time that the follow group is followed for. -1.0 = infinite.
/// * `target_group`: Group that is following
/// * `follow_group`: Group that is being followed
#[inline(always)]
pub fn follow_trigger(
    config: &GDObjConfig,
    x_mod: f64,
    y_mod: f64,
    follow_time: f64,
    target_group: i16,
    follow_group: i16,
) -> GDObject {
    GDObject::new(
        TRIGGER_FOLLOW,
        config,
        vec![
            (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(follow_time)),
            (XAXIS_FOLLOW_MOD, GDValue::Float(x_mod)),
            (YAXIS_FOLLOW_MOD, GDValue::Float(y_mod)),
            (TARGET_ITEM, GDValue::Group(target_group)),
            (TARGET_ITEM_2, GDValue::Group(follow_group)),
        ],
    )
}

/// Returns an animate trigger object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Objects to animate
/// * `animation`: Animation ID, provided in [`Anim`] enum
#[inline(always)]
pub fn animate_trigger(config: &GDObjConfig, target_group: i16, animation: Anim) -> GDObject {
    GDObject::new(
        TRIGGER_ANIMATE,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (ANIMATION_ID, GDValue::Int(animation.into())),
        ],
    )
}

/// Returns a count trigger object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `item_id`: Checks this item
/// * `target_id`: Target group to activate
/// * `target_count`: Target count of item at `item_id`
/// * `activate_group`: Whether or not to activate the target group
/// * `multi_activate`: Whether or not this trigger is multi-activatable
#[inline(always)]
pub fn count_trigger(
    config: &GDObjConfig,
    item_id: i16,
    target_id: i16,
    target_count: i32,
    activate_group: bool,
    multi_activate: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_COUNT,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Item(item_id)),
            (TARGET_ITEM, GDValue::Group(target_id)),
            (TARGET_COUNT, GDValue::Int(target_count)),
            (ACTIVATE_GROUP, GDValue::Bool(activate_group)),
            (MULTI_ACTIVATE, GDValue::Bool(multi_activate)),
        ],
    )
}

/// Returns an advanced random trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `probabilities`: List of tuples: (target group, chance to trigger this group).
///
/// Chances are considered relative to each other, meaning that they are not
/// precentage-based. Two groups with the same relative chance will have the same
/// (50-50) chance to be triggered
#[inline(always)]
pub fn advanced_random_trigger(config: &GDObjConfig, probabilities: Vec<(i16, i32)>) -> GDObject {
    GDObject::new(
        TRIGGER_ADVANCED_RANDOM,
        config,
        vec![(
            RANDOM_PROBABILITIES_LIST,
            GDValue::from_prob_list(probabilities),
        )],
    )
}

/// Returns a UI config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: the UI objects
/// * `ui_reference_obj`: Group with a single object that is a reference for the center of the camera.
/// * `x_reference`: Reference position for the element on the X-axis
/// * `y_reference`: Reference position for the element on the Y-axis
/// * `x_ref_relative`: Whether or not the x-axis position scales with aspect ratio
/// * `y_ref_relative`: Whether or not the y-axis position scales with aspect ratio
#[inline(always)]
pub fn ui_config_trigger(
    config: &GDObjConfig,
    target_group: i16,
    ui_reference_obj: i16,
    x_reference: UIReferencePos,
    y_reference: UIReferencePos,
    x_ref_relative: bool,
    y_ref_relative: bool,
) -> GDObject {
    GDObject::new(
        UI_CONFIG,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (TARGET_ITEM_2, GDValue::Group(ui_reference_obj)),
            (X_REFERENCE_POSITION, GDValue::Int(x_reference as i32)),
            (Y_REFERENCE_POSITION, GDValue::Int(y_reference as i32 + 4)),
            (X_REFERENCE_IS_RELATIVE, GDValue::Bool(x_ref_relative)),
            (Y_REFERENCE_IS_RELATIVE, GDValue::Bool(y_ref_relative)),
        ],
    )
}

/// Returns a rotate trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `move_time`: Time to rotate the target
/// * `rotation_cfg`: Rotation specifics. See [`RotationConfig`]
/// * `easing`: optional move easing and rate. See [`MoveEasing`]
/// * `target_group`: Group that will rotate
/// * `center_group_id`: Group that is being rotated around
/// * `bounding_box`: Optional vertices of a bounding box that limit the position of the rotation group.
///
/// The tuple corresponds to the `MinX`, `MinY`, `MaxX`, `MaxY` group ids respectively in the rotate trigger.
pub fn rotate_trigger(
    config: &GDObjConfig,
    move_time: f64,
    rotation_cfg: RotationConfig,
    easing: Option<(MoveEasing, f64)>,
    target_group: i16,
    center_group_id: i16,
    bounding_box: Option<(i16, i16, i16, i16)>,
) -> GDObject {
    let mut properties = vec![
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(move_time)),
        (DYNAMIC_MOVE, GDValue::Bool(rotation_cfg.dynamic_mode)),
        (
            LOCK_OBJECT_ROTATION,
            GDValue::Bool(rotation_cfg.lock_object_rotation),
        ),
        (TARGET_ITEM, GDValue::Group(target_group)),
        (TARGET_ITEM_2, GDValue::Group(center_group_id)),
    ];

    match rotation_cfg.mode {
        RotationMode::Aim(cfg) => {
            properties.extend_from_slice(&[
                (TARGET_MOVE_MODE, GDValue::Bool(true)),
                (ROTATION_TARGET_ID, GDValue::Group(cfg.aim_target)),
                (ROTATION_OFFSET, GDValue::Float(cfg.rot_offset)),
            ]);

            if let Some(player) = cfg.player_target {
                properties.push(match player {
                    RotationPlayerTarget::Player1 => (CONTROLLING_PLAYER_1, GDValue::Bool(true)),
                    RotationPlayerTarget::Player2 => (CONTROLLING_PLAYER_2, GDValue::Bool(true)),
                });
            }
        }
        RotationMode::Follow(cfg) => {
            properties.extend_from_slice(&[
                (DIRECTIONAL_MOVE_MODE, GDValue::Bool(true)),
                (ROTATION_TARGET_ID, GDValue::Group(cfg.aim_target)),
                (ROTATION_OFFSET, GDValue::Float(cfg.rot_offset)),
            ]);

            if let Some(player) = cfg.player_target {
                properties.push(match player {
                    RotationPlayerTarget::Player1 => (CONTROLLING_PLAYER_1, GDValue::Bool(true)),
                    RotationPlayerTarget::Player2 => (CONTROLLING_PLAYER_2, GDValue::Bool(true)),
                });
            }
        }
        RotationMode::Default(cfg) => {
            properties.extend_from_slice(&[
                (ROTATE_DEGREES, GDValue::Float(cfg.degrees)),
                (ROTATE_X360, GDValue::Int(cfg.x360)),
            ]);
        }
    }

    add_easing(&mut properties, easing);
    if let Some((min_x, min_y, max_x, max_y)) = bounding_box {
        properties.extend_from_slice(&[
            (MINX_ID, GDValue::Group(min_x)),
            (MINY_ID, GDValue::Group(min_y)),
            (MAXX_ID, GDValue::Group(max_x)),
            (MAXY_ID, GDValue::Group(max_y)),
        ]);
    }

    GDObject::new(TRIGGER_ROTATION, config, properties)
}

/// Returns a scale trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `scale_config`: Scaling config. See [`ScaleConfig`]
/// * `easing`: Optional move easing and rate. See [`MoveEasing`]
/// * `center_group_id`: Center of group that is being scaled. Leave as 0 to use the default center
/// * `target_group`: Group that is being scaled.
/// * `duration`: How long the scaling will be
pub fn scale_trigger(
    config: &GDObjConfig,
    scale_config: ScaleConfig,
    easing: Option<(MoveEasing, f64)>,
    center_group_id: i16,
    target_group: i16,
    duration: f64,
) -> GDObject {
    let mut properties = vec![
        (NEW_X_SCALE, GDValue::Float(scale_config.x_scale)),
        (NEW_Y_SCALE, GDValue::Float(scale_config.y_scale)),
        (DIV_BY_VALUE_X, GDValue::Bool(scale_config.div_by_value_x)),
        (DIV_BY_VALUE_Y, GDValue::Bool(scale_config.div_by_value_y)),
        (TARGET_ITEM, GDValue::Group(target_group)),
        (TARGET_ITEM_2, GDValue::Group(center_group_id)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(duration)),
        (ONLY_MOVE, GDValue::Bool(scale_config.only_move)),
        (RELATIVE_SCALE, GDValue::Bool(scale_config.relative_scale)),
        (
            RELATIVE_ROTATION,
            GDValue::Bool(scale_config.relative_rotation),
        ),
    ];

    add_easing(&mut properties, easing);
    GDObject::new(TRIGGER_SCALE, config, properties)
}

/// Returns a scale trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `speed`: Follow speed in the range \[0.0, 1.0]; 1.0 = instantaneously snaps to player y-pos
/// * `delay`: Delay of the following group
/// * `offset`: Y offset of the following group
/// * `max_speed`: Speed limit of the following group
/// * `move_time`: How long the group will follow the player
/// * `target_group`: The group that is following the player's y-pos
#[inline(always)]
pub fn follow_player_y(
    config: &GDObjConfig,
    speed: f64,
    delay: f64,
    offset: i32,
    max_speed: f64,
    move_time: f64,
    target_group: i16,
) -> GDObject {
    GDObject::new(
        TRIGGER_FOLLOW_PLAYER_Y,
        config,
        vec![
            (FOLLOW_SPEED, GDValue::Float(speed)),
            (FOLLOW_DELAY, GDValue::Float(delay)),
            (FOLLOW_OFFSET, GDValue::Int(offset)),
            (MAX_FOLLOW_SPEED, GDValue::Float(max_speed)),
            (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(move_time)),
            (TARGET_ITEM, GDValue::Group(target_group)),
        ],
    )
}

/// Returns a middleground config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn mg_config(
    config: &GDObjConfig,
    offset_y: i32,
    easing: Option<(MoveEasing, f64)>,
) -> GDObject {
    let mut properties = vec![(MOVE_UNITS_Y, GDValue::Int(offset_y))];
    add_easing(&mut properties, easing);
    GDObject::new(TRIGGER_MIDDLEGROUND_CONFIG, config, properties)
}

// util fn to add easing to properties if it is specified
fn add_easing(properties: &mut Vec<(u16, GDValue)>, easing: Option<(MoveEasing, f64)>) {
    if let Some((easing, rate)) = easing {
        properties.extend_from_slice(&[
            (MOVE_EASING, GDValue::Easing(easing)),
            (EASING_RATE, GDValue::Float(rate)),
        ])
    }
}

/// Returns an event config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn event_trigger(
    config: &GDObjConfig,
    target_group: i16,
    events: Vec<Event>,
    extra_id: i16,
    extra_id2: ExtraID2,
) -> GDObject {
    GDObject::new(
        TRIGGER_EVENT,
        config,
        vec![
            (IS_INTERACTABLE, GDValue::Bool(true)),
            (TARGET_ITEM, GDValue::Group(target_group)),
            (EVENT_LISTENERS, GDValue::Events(events)),
            (EVENT_EXTRA_ID, GDValue::Group(extra_id)),
            (EVENT_EXTRA_ID_2, GDValue::Int(extra_id2 as i32)),
        ],
    )
}

/// Returns a middle ground change trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `middleground`: Middleground to change to
pub fn middle_ground_trigger(config: &GDObjConfig, middleground: MiddleGround) -> GDObject {
    GDObject::new(
        TRIGGER_MIDDLEGROUND_CHANGE,
        config,
        vec![(MIDDLEGROUND, GDValue::Int(middleground as i32))],
    )
}

/// Returns a middle ground change trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Group that is activated when the trigger registers a click
/// * `hold_mode`: Toggles target group on holding and releasing instead of clicking
/// * `dual_mode`: Blocks 2nd player's clicks. Deprecated in favour of [`OptionalPlayerTarget::Player1`]
/// * `toggle`: Toggles a specific activation mode. See [`TouchToggle`]
/// * `target_player`: Only registers clicks from one player. See [`OptionalPlayerTarget`]
pub fn touch_trigger(
    config: &GDObjConfig,
    target_group: i16,
    hold_mode: bool,
    dual_mode: bool,
    toggle: TouchToggle,
    target_player: OptionalPlayerTarget,
) -> GDObject {
    GDObject::new(
        TRIGGER_TOUCH,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (TOUCH_HOLD_MODE, GDValue::Bool(hold_mode)),
            (TOUCH_DUAL_MODE, GDValue::Bool(dual_mode)),
            (TOUCH_TOGGLE_ONOFF, GDValue::Int(toggle as i32)),
            (TOUCH_PLAYER_ONLY, GDValue::Int(target_player as i32)),
        ],
    )
}

/// Returns an area stop trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `effect_id`: Area effect that is stopped
pub fn area_stop(config: &GDObjConfig, effect_id: i16) -> GDObject {
    GDObject::new(
        TRIGGER_AREA_STOP,
        config,
        vec![(TARGET_ITEM, GDValue::Short(effect_id))],
    )
}

/* TODO: trigger constructors
 * Animation triggers
 * advanced follow
 * edit advanced follow
 * re-target advanced follow
 * keyframe setup trigger
 * keyframe setup object
 *
 * Area triggers
 * area move
 * area rotate
 * area scale
 * area fade
 * area tint
 * edit area move
 * edit area rotate
 * edit area scale
 * edit area fade
 * edit area tint
 * enter area move
 * enter area rotate
 * enter area scale
 * enter area fade
 * enter area tint
 * enter area stop
 * area stop
 *
 * Background triggers
 * switch bg
 * sdwitch ground
 *
 * Item triggers
 * instant count trigger
 * pickup trigger
 *
 * Spawner triggers
 * sequence
 *
 * Camera
 * static camera
 * offset camera
 * gameplay offset camera
 * rotate camera
 * edge camera
 * camera mode
 *
 * Gameplay triggers
 * rotate gameplay
 *
 * Sound triggers
 * song trigger
 * edit song trigger
 * sfx trigger
 * edit sfx trigger
 *
 * Misc.
 * bpm marker
 * gradient
 *
 * Player triggers
 * options
 * teleport trigger
 *
 * Shaders
 * shader setup
 * shock wave shader
 * shock line shader
 * glitch shader
 * chromatic shader
 * chromatic glitch shader
 * pixelate shader
 * lens circle shader
 * radial bulb shader
 * motion blur shader
 * bulge shader
 * pinch shader
 * gray scale shader
 * sepia shader
 * invert colour shader
 * hue shader
 * edit colour shader
 * split screen shader
 */
