//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use crate::gdobj::{
    Anim, ColourChannel, GDObjConfig, GDObject, GDValue, MoveEasing,
    ids::{
        objects::{
            BG_EFFECT_OFF, BG_EFFECT_ON, BG_SPEED_CONFIG, CAMERA_GUIDE, COLLISION_BLOCK,
            COLLISION_STATE_BLOCK, COUNTER, DISABLE_PLAYER_TRAIL, ENABLE_PLAYER_TRAIL,
            MG_SPEED_CONFIG, START_POS, TOGGLE_BLOCK, TRIGGER_ADVANCED_RANDOM, TRIGGER_ANIMATE,
            TRIGGER_CAMERA_ZOOM, TRIGGER_COLOUR, TRIGGER_COUNT, TRIGGER_END, TRIGGER_FOLLOW,
            TRIGGER_GRAVITY, TRIGGER_ITEM_COMPARE, TRIGGER_ITEM_EDIT, TRIGGER_LINK_VISIBLE,
            TRIGGER_MOVE, TRIGGER_ON_DEATH, TRIGGER_PERSISTENT_ITEM, TRIGGER_PLAYER_CONTROL,
            TRIGGER_RANDOM, TRIGGER_RESET_GROUP, TRIGGER_REVERSE_GAMEPLAY, TRIGGER_SHAKE,
            TRIGGER_SPAWN, TRIGGER_SPAWN_PARTICLE, TRIGGER_STOP, TRIGGER_TIME_CONTROL,
            TRIGGER_TIME_EVENT, TRIGGER_TIME_WARP, TRIGGER_TOGGLE,
        },
        properties::{
            ACTIVATE_GROUP, ANIMATION_ID, BLENDING_ENABLED, BLUE, CAMERA_GUIDE_PREVIEW_OPACITY,
            CAMERA_ZOOM, CENTER_GROUP_ID, CLAIM_TOUCH, COLOUR_CHANNEL, COMPARE_OPERATOR,
            CONTROLLING_PLAYER_1, CONTROLLING_PLAYER_2, COPY_COLOUR_FROM_CHANNEL,
            COPY_COLOUR_SPECS, COPY_OPACITY, COUNTER_ALIGNMENT, DIRECTIONAL_MODE_DISTANCE,
            DIRECTIONAL_MOVE_MODE, DISABLE_PREVIEW, DURATION_GROUP_TRIGGER_CHANCE, DYNAMIC_BLOCK,
            DYNAMIC_MOVE, EASING_RATE, ENTEREXIT_TRANSITION_CONFIG, EVENT_TARGET_TIME,
            FIRST_ITEM_TYPE, FOLLOW_CAMERAS_X_MOVEMENT, FOLLOW_CAMERAS_Y_MOVEMENT,
            FOLLOW_PLAYERS_X_MOVEMENT, FOLLOW_PLAYERS_Y_MOVEMENT, GRAVITY, GREEN, INPUT_ITEM_1,
            INPUT_ITEM_2, INSTANT_END, IS_ACTIVE_TRIGGER, IS_DISABLED, IS_TIMER, LEFT_OPERATOR,
            LEFT_ROUND_MODE, LEFT_SIGN_MODE, MATCH_ROTATION_OF_SPAWNED_PARTICLES, MODIFIER,
            MOVE_EASING, MOVE_UNITS_X, MOVE_UNITS_Y, MULTI_ACTIVATE, MULTIACTIVATABLE_TIME_EVENT,
            NO_END_EFFECTS, NO_END_SOUND_EFFECTS, NO_LEGACY_HSV, OPACITY, RANDOM_PROBABLITIES_LIST,
            RED, RESET_CAMERA, RESET_ITEM_TO_0, RESET_REMAP, REVERSE_GAMEPLAY, RIGHT_OPERATOR,
            RIGHT_ROUND_MODE, RIGHT_SIGN_MODE, ROTATE_GAMEPLAY, ROTATION_OF_SPAWNED_PARTICLES,
            ROTATION_VARIATION_OF_SPAWNED_PARTICLES, SCALE_OF_SPAWNED_PARTICLES,
            SCALE_VARIATION_OF_SPAWNED_PARTICLES, SECOND_ITEM_TYPE, SECOND_MODIFIER, SECONDS_ONLY,
            SET_PERSISTENT_ITEM, SHAKE_INTERVAL, SHAKE_STRENGTH, SILENT_MOVE, SMALL_STEP,
            SPAWN_DELAY, SPAWN_DELAY_VARIATION, SPAWN_ONLY, SPAWN_ORDERED, SPECIAL_COUNTER_MODE,
            STARTING_GAMEMODE, STARTING_IN_DUAL_MODE, STARTING_IN_MINI_MODE,
            STARTING_IN_MIRROR_MODE, STARTING_SPEED, STOP_MODE, STOP_PLAYER_JUMP,
            STOP_PLAYER_MOVEMENT, STOP_PLAYER_ROTATION, STOP_PLAYER_SLIDING, STOP_TIME_COUNTER,
            TARGET_ALL_PERSISTENT_ITEMS, TARGET_CHANNEL, TARGET_COUNT, TARGET_ITEM, TARGET_ITEM_2,
            TARGET_ITEM_TYPE, TARGET_MOVE_MODE, TARGET_MOVE_MODE_AXIS_LOCK, TARGET_ORDER,
            TARGET_TRANSITION_CHANNEL, TIMER, TIMEWARP_AMOUNT, TOLERANCE, USE_CONTROL_ID,
            USING_PLAYER_COLOUR_1, USING_PLAYER_COLOUR_2, X_MOVEMENT_MULTIPLIER,
            X_OFFSET_OF_SPAWNED_PARTICLES, X_OFFSET_VARIATION_OF_SPAWNED_PARTICLES,
            XAXIS_FOLLOW_MOD, Y_MOVEMENT_MULTIPLIER, Y_OFFSET_OF_SPAWNED_PARTICLES,
            Y_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, YAXIS_FOLLOW_MOD,
        },
    },
};

/*
template

/// Returns a <TRIGGER> trigger
/// # Arguments
/// * `config`: General object options, such as position and scale

pub fn FN_NAME(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(id, config, vec![()])
}
*/

/// Constant distinct arbitrary value for player 1 position.
pub const POS_PLAYER1: i32 = 99999;
/// Constant distinct arbitrary value for player 2 position.
pub const POS_PLAYER2: i32 = 99998;

/// Constant distinct value for X-axis move lock.
pub const MOVE_X_ONLY: i32 = 1;
/// Constant distinct value for Y-axis move lock.
pub const MOVE_Y_ONLY: i32 = 2;

/// Enum for the GD gamemodes corresponding to their internal values
#[repr(i32)]
pub enum Gamemode {
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
pub enum StopMode {
    Stop = 0,
    Pause = 1,
    Resume = 2,
}

/// Enum for item alignments
#[repr(i32)]
pub enum ItemAlign {
    Center = 0,
    Left = 1,
    Right = 2,
}

/// Enum for counter modes
#[repr(i32)]
pub enum CounterMode {
    Attempts = -3,
    Points = -2,
    MainTime = -1,
}

/// Enum for transition object enter/exit config
#[repr(i32)]
#[derive(PartialEq)]
pub enum TransitionMode {
    Both = 0,
    Enter = 1,
    Exit = 2,
}

/// Enum for transition object type (from top, from bottom, etc.)
#[repr(i32)]
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
    None = 1915,
}

/// Enum for counter types
#[repr(i32)]
pub enum ItemType {
    Counter = 1,
    Timer = 2,
    Points = 3,
    MainTime = 4,
    Attempts = 5,
}

/// Enum for item operators
#[repr(i32)]
pub enum Op {
    Set = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
}

/// Enum for item comparison operators
#[repr(i32)]
pub enum CompareOp {
    Equals = 0,
    Greater = 1,
    GreaterOrEquals = 2,
    Less = 3,
    LessOrEquals = 4,
    NotEquals = 5,
}

/// Enum for item round modes
#[repr(i32)]
pub enum RoundMode {
    None = 0,
    Nearest = 1,
    Floor = 2,
    Ceiling = 3,
}

/// Enum for item sign modes
#[repr(i32)]
pub enum SignMode {
    None = 0,
    Absolute = 1,
    Negative = 2,
}

/// Enum for target player in gravity trigger
#[repr(u16)]
pub enum TargetPlayer {
    Player1 = 138,
    Player2 = 200,
    PlayerTarget = 201,
}

/// Enum for move mode setting. See structs [`DefaultMove`], [`TargetMove`], and [`DirectionalMove`]
pub enum MoveMode {
    Default(DefaultMove),
    Targeting(TargetMove),
    Directional(DirectionalMove),
}

/// Enum for lock config: player or camera
pub enum MoveLock {
    Player,
    Camera,
}

/// Config struct for default movement
/// # Fields
/// * `dx`: Units to move in x-axis. Used as multiplier of player/camera movement if `x_lock` is used
/// * `dy`: Units to move in y-axis. Used as multiplier of player/camera movement if `y_lock` is used
/// * `x_lock`: Optional lock on x movement which allows the object to move relative to either the player or the camera
/// * `y_lock`: Optional lock on y movement which allows the object to move relative to either the player or the camera
pub struct DefaultMove {
    pub dx: f64,
    pub dy: f64,
    pub x_lock: Option<MoveLock>,
    pub y_lock: Option<MoveLock>,
}

/// Config struct for moving to a specific target.
/// # Fields
/// * `target_group_id`: Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
/// * `center_group_id`: (Optional) The objects that represent the center of the group that is moving
/// * `axis_only`: Optional axis restriction. Use constants `MOVE_X_ONLY` and `MOVE_Y_ONLY` to specify axis.
pub struct TargetMove {
    pub target_group_id: i32,
    pub center_group_id: Option<i32>,
    pub axis_only: Option<i32>,
}

/// Config struct for moving to a specific target.
/// # Fields
/// * `target_group_id`: Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
/// * `center_group_id`: (Optional) The objects that represent the center of the group that is moving
/// * `distance`: Distance in units to move in the direction of the target objects.
pub struct DirectionalMove {
    pub target_group_id: i32,
    pub center_group_id: Option<i32>,
    pub distance: i32,
}

/// Enum for
#[repr(i32)]
pub enum StartingSpeed {
    X0Point5 = 1,
    X1 = 0,
    X2 = 2,
    X3 = 3,
    X4 = 4,
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
    config: GDObjConfig,
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
        (SMALL_STEP, GDValue::Int(1)),
        (DYNAMIC_MOVE, GDValue::Int(dynamic as i32)),
        (SILENT_MOVE, GDValue::Int(silent as i32)),
    ];

    if let Some((easing, rate)) = easing {
        properties.push((MOVE_EASING, GDValue::Int(easing as i32)));
        properties.push((EASING_RATE, GDValue::Float(rate)));
    }

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
                properties.push((CENTER_GROUP_ID, GDValue::Int(id)));
            }

            if let Some(axis) = config.axis_only {
                properties.push((TARGET_MOVE_MODE_AXIS_LOCK, GDValue::Int(axis)));
            }

            match config.target_group_id {
                POS_PLAYER1 => properties.push((CONTROLLING_PLAYER_1, GDValue::Int(1))),
                POS_PLAYER2 => properties.push((CONTROLLING_PLAYER_2, GDValue::Int(1))),
                id => properties.push((TARGET_ITEM_2, GDValue::Int(id))),
            };
        }
        MoveMode::Directional(config) => {
            if let Some(id) = config.center_group_id {
                properties.push((CENTER_GROUP_ID, GDValue::Int(id)));
            }

            match config.target_group_id {
                POS_PLAYER1 => properties.push((CONTROLLING_PLAYER_1, GDValue::Int(1))),
                POS_PLAYER2 => properties.push((CONTROLLING_PLAYER_2, GDValue::Int(1))),
                id => properties.push((TARGET_ITEM_2, GDValue::Int(id))),
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
/// * `start_speed`: Starting speed of player: will be clamped to closest value in `[0.5, 1.0, 2.0, 3.0, 4.0]`
/// * `starting_gamemode`: Starting gamemode; Default: Cube
/// * `starting_as_mini`: Starting as mini? Default: false
/// * `starting_as_dual`: Start as dual? Default: false
/// * `starting_mirrored`: Start as mirrored? Default: false
/// * `reset_camera`: Reset camera? Default: false
/// * `rotate_gameplay`: Rotate gameplay? Default: false
/// * `reverse_gameplay`: Reverse gameplay? Default: false
/// * `target_order`: Target order (of what, I don't know); Default: 0
/// * `target_channel`: Target channel (once again, I don't know); Default: 0
/// * `disabled`: Disabled startpos? Default: false
pub fn start_pos(
    config: GDObjConfig,
    start_speed: StartingSpeed,
    starting_gamemode: Gamemode,
    starting_as_mini: bool,
    starting_as_dual: bool,
    starting_mirrored: bool,
    reset_camera: bool,
    rotate_gameplay: bool,
    reverse_gameplay: bool,
    target_order: i32,
    target_channel: i32,
    disabled: bool,
) -> GDObject {
    GDObject::new(
        START_POS,
        config,
        vec![
            (STARTING_SPEED, GDValue::Int(start_speed as i32)),
            (STARTING_GAMEMODE, GDValue::Int(starting_gamemode as i32)),
            (STARTING_IN_MINI_MODE, GDValue::Bool(starting_as_mini)),
            (STARTING_IN_DUAL_MODE, GDValue::Bool(starting_as_dual)),
            (IS_DISABLED, GDValue::Bool(disabled)),
            (STARTING_IN_MIRROR_MODE, GDValue::Bool(starting_mirrored)),
            (ROTATE_GAMEPLAY, GDValue::Bool(rotate_gameplay)),
            (REVERSE_GAMEPLAY, GDValue::Bool(reverse_gameplay)),
            (TARGET_ORDER, GDValue::Int(target_order)),
            (TARGET_CHANNEL, GDValue::Int(target_channel)),
            (RESET_CAMERA, GDValue::Bool(reset_camera)),
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
/// * `colour`: (R, G, B) tuple of `u8`s
/// * `fade_time`: Time to fade into the colour
/// * `opacity`: Opacity of colour
/// * `blending`: Use blending?
/// * `use_player_col_1`: Use player colour 1 instead of the specified colour.
/// * `use_player_col_2`: Use player colour 2 instead of the specified colour.
/// * `copy_colour`: None: Don't copy colour; Some: Copy colour with this configuation:
/// (original channel, hue shift, saturation multiplier, brightness multiplier, static saturation scalar?,
/// static brightness scalar?, use legacy hsv?, copy opacity?)
pub fn colour_trigger(
    config: GDObjConfig,
    colour: (u8, u8, u8),
    channel: ColourChannel,
    fade_time: f64,
    opacity: f64,
    blending: bool,
    use_player_col_1: bool,
    use_player_col_2: bool,
    copy_colour: Option<(ColourChannel, i32, f64, f64, bool, bool, bool, bool)>,
) -> GDObject {
    let mut properties = vec![
        (RED, GDValue::Int(colour.0 as i32)),
        (GREEN, GDValue::Int(colour.1 as i32)),
        (BLUE, GDValue::Int(colour.2 as i32)),
        (DURATION_GROUP_TRIGGER_CHANCE, GDValue::Float(fade_time)),
        (USING_PLAYER_COLOUR_1, GDValue::Int(use_player_col_1 as i32)),
        (USING_PLAYER_COLOUR_2, GDValue::Int(use_player_col_2 as i32)),
        (COLOUR_CHANNEL, GDValue::Int(channel.as_i32())),
        (OPACITY, GDValue::Float(opacity)),
        (BLENDING_ENABLED, GDValue::Bool(blending)),
    ];

    if let Some((
        channel,
        hue,
        saturation,
        lightness,
        static_sat_scalar,
        static_brightness_scalar,
        legacy_hsv,
        copy_opacity,
    )) = copy_colour
    {
        let mut cfg_string = format!(
            "{hue}a{saturation}a{lightness}a{}a",
            static_sat_scalar as i32
        );
        if !legacy_hsv {
            cfg_string += &format!("{}", static_brightness_scalar as i32);
            properties.push((NO_LEGACY_HSV, GDValue::Bool(true)));
        }

        properties.push((COPY_OPACITY, GDValue::Bool(copy_opacity)));
        properties.push((COPY_COLOUR_SPECS, GDValue::String(cfg_string)));
        properties.push((COPY_COLOUR_FROM_CHANNEL, GDValue::Int(channel.as_i32())));
    }

    GDObject::new(TRIGGER_COLOUR, config, properties)
}

/// Returns a stop trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `stop_mode`: Stop mode (see [`StopMode`] struct)
/// * `use_control_id`: Only stops certain triggers within a group if enabled.
#[inline(always)]
pub fn stop_trigger(
    config: GDObjConfig,
    target_group: i16,
    stop_mode: StopMode,
    use_control_id: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_STOP,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (USE_CONTROL_ID, GDValue::Int(use_control_id as i32)),
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
    config: GDObjConfig,
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
pub fn toggle_trigger(config: GDObjConfig, target_group: i16, activate_group: bool) -> GDObject {
    GDObject::new(
        TRIGGER_TOGGLE,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (ACTIVATE_GROUP, GDValue::Bool(activate_group)),
        ],
    )
}

// todo
fn pulse_trigger(config: GDObjConfig) {}

/// Returns a transition object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `transition`: Type of transition. See [`TransitionType`] struct
/// * `mode`: Mode for transition (enter/exit only). See [`TransitionMode`] struct
/// * `target_channel`: Optional target channel argument which specifies a channel for this transition.
pub fn transition_object(
    config: GDObjConfig,
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
pub fn reverse_gameplay(config: GDObjConfig) -> GDObject {
    GDObject::new(TRIGGER_REVERSE_GAMEPLAY, config, vec![])
}

/// Returns a link visible trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is linked visibly
#[inline(always)]
pub fn link_visible(config: GDObjConfig, target_group: i16) -> GDObject {
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
pub fn timewarp(config: GDObjConfig, time_scale: f64) -> GDObject {
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
pub fn show_player(config: GDObjConfig) -> GDObject {
    GDObject::new(1613, config, vec![]) // todo
}

/// Returns a trigger that hides the player
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn hide_player(config: GDObjConfig) -> GDObject {
    GDObject::new(1612, config, vec![]) // todo
}

/// Returns a trigger that shows the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn show_player_trail(config: GDObjConfig) -> GDObject {
    GDObject::new(ENABLE_PLAYER_TRAIL, config, vec![])
}

/// Returns a trigger that hides the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale\
#[inline(always)]
pub fn hide_player_trail(config: GDObjConfig) -> GDObject {
    GDObject::new(DISABLE_PLAYER_TRAIL, config, vec![])
}

/// Returns a trigger that enables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn bg_effect_on(config: GDObjConfig) -> GDObject {
    GDObject::new(BG_EFFECT_ON, config, vec![])
}

/// Returns a trigger that disables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
#[inline(always)]
pub fn bg_effect_off(config: GDObjConfig) -> GDObject {
    GDObject::new(BG_EFFECT_OFF, config, vec![])
}

/// Returns a group reset trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is to be reset
#[inline(always)]
pub fn group_reset(config: GDObjConfig, target_group: i16) -> GDObject {
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
pub fn shake_trigger(config: GDObjConfig, strength: i32, interval: f64, duration: f64) -> GDObject {
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
pub fn bg_speed(config: GDObjConfig, mod_x: f64, mod_y: f64) -> GDObject {
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
pub fn mg_speed(config: GDObjConfig, mod_x: f64, mod_y: f64) -> GDObject {
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
    config: GDObjConfig,
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
    config: GDObjConfig,
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
    config: GDObjConfig,
    spawn_id: Option<i32>,
    target_pos: Option<i32>,
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
        properties.push((TARGET_ITEM, GDValue::Int(id)));
    }

    if let Some(pos) = target_pos {
        properties.push((TARGET_ITEM_2, GDValue::Int(pos)));
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
/// * `special_mode`: Other special mode of timer. See [`CounterMode`] struct.
pub fn counter_object(
    config: GDObjConfig,
    item_id: i16,
    timer: bool,
    align: ItemAlign,
    seconds_only: bool,
    special_mode: Option<CounterMode>,
) -> GDObject {
    let mut properties = vec![
        (INPUT_ITEM_1, GDValue::Item(item_id)),
        (SECONDS_ONLY, GDValue::Bool(seconds_only)),
        (COUNTER_ALIGNMENT, GDValue::Int(align as i32)),
        (IS_TIMER, GDValue::Bool(timer)),
    ];

    if let Some(mode) = special_mode {
        properties.push((SPECIAL_COUNTER_MODE, GDValue::Int(mode as i32)));
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
/// * `assign_op`: operator for assigning to result; see [`Op`] enum.
/// * `mod_op`: operator for applying mod to result; see [`Op`] enum.
/// * `id_op`: operator between operands 1 and 2; see [`Op`] enum.
/// * `id_rounding`: operand rounding function; see [`RoundMode`] enum.
/// * `result_rounding`: final rounding function; see [`RoundMode`] enum.
/// * `id_sign`: operand signing function; see [`SignMode`] enum.
/// * `result_sign`: final signing function; see [`SignMode`] enum.
pub fn item_edit(
    config: GDObjConfig,
    operand1: Option<(i32, ItemType)>,
    operand2: Option<(i32, ItemType)>,
    target_id: i16,
    target_type: ItemType,
    modifier: f64,
    assign_op: Op,
    mod_op: Option<Op>,
    id_op: Option<Op>,
    id_rounding: RoundMode,
    result_rounding: RoundMode,
    id_sign: SignMode,
    result_sign: SignMode,
) -> GDObject {
    let mod_op = match mod_op {
        Some(op) => op,
        None => Op::Mul,
    };
    let id_op = match id_op {
        Some(op) => op,
        None => Op::Add,
    };

    let op_1 = match operand1 {
        Some(cfg) => cfg,
        None => (0, ItemType::Counter),
    };
    let op_2 = match operand2 {
        Some(cfg) => cfg,
        None => (0, ItemType::Counter),
    };

    GDObject::new(
        TRIGGER_ITEM_EDIT,
        config,
        vec![
            (IS_ACTIVE_TRIGGER, GDValue::Int(1)),
            (TARGET_ITEM, GDValue::Item(target_id)),
            (INPUT_ITEM_1, GDValue::Int(op_1.0)),
            (INPUT_ITEM_2, GDValue::Int(op_2.0)),
            (FIRST_ITEM_TYPE, GDValue::Int(op_1.1 as i32)),
            (SECOND_ITEM_TYPE, GDValue::Int(op_2.1 as i32)),
            (TARGET_ITEM_TYPE, GDValue::Int(target_type as i32)),
            (MODIFIER, GDValue::Float(modifier)),
            (LEFT_OPERATOR, GDValue::Int(assign_op as i32)),
            (RIGHT_OPERATOR, GDValue::Int(id_op as i32)),
            (COMPARE_OPERATOR, GDValue::Int(mod_op as i32)),
            (LEFT_ROUND_MODE, GDValue::Int(id_rounding as i32)),
            (RIGHT_ROUND_MODE, GDValue::Int(result_rounding as i32)),
            (LEFT_SIGN_MODE, GDValue::Int(id_sign as i32)),
            (RIGHT_SIGN_MODE, GDValue::Int(result_sign as i32)),
        ],
    )
}

/// Returns an item compare trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `true_id`: Group that is activated when the comparison is true
/// * `false_id`: Group that is activated when the comparison is false
/// * \*`lhs`: (id, item type, modifier, modifier operator: [`Op`], [`RoundMode`], [`SignMode`]) config tuple for left-hand side operator.
/// * \*`rhs`: (id, item type, modifier, modifier operator: [`Op`], [`RoundMode`], [`SignMode`]) config tuple for right-hand side operator.
/// The right-hand side will be just the modifier is the item id is left as 0 (not specified)
/// * `compare_op`: Operator used to compare the two sides. See [`CompareOp`] enum.
/// * `tolerance`: Tolerant range of comparsion. Comparsion will be true if the absolute resulting value is less than or equal to the tolerance.
///
/// \*The modifier operators describe how the modifier interacts with the item, except for setting the item
pub fn item_compare(
    config: GDObjConfig,
    true_id: i32,
    false_id: i32,
    lhs: (i32, ItemType, f64, Op, RoundMode, SignMode),
    rhs: (i32, ItemType, f64, Op, RoundMode, SignMode),
    compare_op: CompareOp,
    tolerance: f64,
) -> GDObject {
    let properties = vec![
        (TARGET_ITEM, GDValue::Int(true_id)),
        (TARGET_ITEM_2, GDValue::Int(false_id)),
        // ids
        (INPUT_ITEM_1, GDValue::Int(lhs.0)),
        (INPUT_ITEM_2, GDValue::Int(rhs.0)),
        // types
        (FIRST_ITEM_TYPE, GDValue::Int(lhs.1 as i32)),
        (SECOND_ITEM_TYPE, GDValue::Int(rhs.1 as i32)),
        // modifiers
        (MODIFIER, GDValue::Float(lhs.2)),
        (SECOND_MODIFIER, GDValue::Float(rhs.2)),
        // modifiers ops
        (LEFT_OPERATOR, GDValue::Int(lhs.3 as i32)),
        (RIGHT_OPERATOR, GDValue::Int(rhs.3 as i32)),
        (COMPARE_OPERATOR, GDValue::Int(compare_op as i32)),
        (TOLERANCE, GDValue::Float(tolerance)),
        // round modes
        (LEFT_ROUND_MODE, GDValue::Int(lhs.4 as i32)),
        (RIGHT_ROUND_MODE, GDValue::Int(rhs.4 as i32)),
        // sign modes
        (LEFT_SIGN_MODE, GDValue::Int(lhs.5 as i32)),
        (RIGHT_SIGN_MODE, GDValue::Int(rhs.5 as i32)),
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
    config: GDObjConfig,
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
    config: GDObjConfig,
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
/// * `reset_remap`: does something
/// * `spawn_ordered`: Spawns constituents of group in the order of x-position
/// * `preview_disable`: also does something
pub fn spawn_trigger(
    config: GDObjConfig,
    spawn_id: i32,
    delay: f64,
    delay_variation: f64,
    reset_remap: bool,
    spawn_ordered: bool,
    preview_disable: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_SPAWN,
        config,
        vec![
            (TARGET_ITEM, GDValue::Int(spawn_id)),
            (SPAWN_DELAY, GDValue::Float(delay)),
            (DISABLE_PREVIEW, GDValue::Bool(preview_disable)),
            (SPAWN_ORDERED, GDValue::Bool(spawn_ordered)),
            (SPAWN_DELAY_VARIATION, GDValue::Float(delay_variation)),
            (RESET_REMAP, GDValue::Bool(reset_remap)),
            // todo: the list of remaps
        ],
    )
}

/// Returns an on-death trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Spawns this group
/// * `activate_group`: Activate this group (instead of toggling off)?
#[inline(always)]
pub fn on_death(config: GDObjConfig, target_group: i16, activate_group: bool) -> GDObject {
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
/// * `position_offsets`: (x, y) tuple for offsets from their original spawn location.
/// Note: all particle objects spawn in the same position, regardless of their offsets within their group.
/// * `position_variation`: (x, y) tuple for range of possible random positional variation.
/// * `rotation_config`: (rotation, variation) tuple that describes the rotation of the particles + random offset range
/// * `scale_config`: (scale, variation) tuple that describes the scale of the particles + random offset range
/// * `match_rotation`: makes all of the particles in the group be rotated in the same direction.
pub fn spawn_particle(
    config: GDObjConfig,
    particle_group: i32,
    position_group: i32,
    position_offsets: Option<(i32, i32)>,
    position_variation: Option<(i32, i32)>,
    rotation_config: Option<(i32, i32)>,
    scale_config: Option<(f64, f64)>,
    match_rotation: bool,
) -> GDObject {
    let mut properties = vec![
        (TARGET_ITEM, GDValue::Int(particle_group)),
        (TARGET_ITEM_2, GDValue::Int(position_group)),
        (
            MATCH_ROTATION_OF_SPAWNED_PARTICLES,
            GDValue::Bool(match_rotation),
        ),
    ];

    if let Some((x, y)) = position_offsets {
        properties.push((X_OFFSET_OF_SPAWNED_PARTICLES, GDValue::Int(x)));
        properties.push((Y_OFFSET_OF_SPAWNED_PARTICLES, GDValue::Int(y)));
    }

    if let Some((x, y)) = position_variation {
        properties.push((X_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(x)));
        properties.push((Y_OFFSET_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(y)));
    }

    if let Some((rot, var)) = rotation_config {
        properties.push((ROTATION_OF_SPAWNED_PARTICLES, GDValue::Int(rot)));
        properties.push((ROTATION_VARIATION_OF_SPAWNED_PARTICLES, GDValue::Int(var)));
    }

    if let Some((scale, var)) = scale_config {
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
pub fn collision_block(config: GDObjConfig, id: i32, dynamic: bool) -> GDObject {
    GDObject::new(
        COLLISION_BLOCK,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Int(id)),
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
    config: GDObjConfig,
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
pub fn state_block(config: GDObjConfig, state_on: i32, state_off: i32) -> GDObject {
    GDObject::new(
        COLLISION_STATE_BLOCK,
        config,
        vec![
            (TARGET_ITEM, GDValue::Int(state_on)),
            (TARGET_ITEM_2, GDValue::Int(state_off)),
        ],
    )
}

// /// TODO
// pub fn collision_trigger(
//     config: GDObjConfig
// ) -> GDObject {

// }

// /// TODO
// pub fn instant_coll_trigger(
//     config: GDObjConfig
// ) -> GDObject {

// }

// time triggers

/// Returns a time control trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Timer ID
/// * `stop`: If enabled, stops the timer; otherwise, starts the timer.
#[inline(always)]
pub fn time_control(config: GDObjConfig, id: i32, stop: bool) -> GDObject {
    GDObject::new(
        TRIGGER_TIME_CONTROL,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Int(id)),
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
    config: GDObjConfig,
    id: i32,
    target_group: i16,
    target_time: f64,
    multi_activate: bool,
) -> GDObject {
    GDObject::new(
        TRIGGER_TIME_EVENT,
        config,
        vec![
            (INPUT_ITEM_1, GDValue::Int(id)),
            (TARGET_ITEM, GDValue::Group(target_group)),
            (EVENT_TARGET_TIME, GDValue::Float(target_time)),
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
    config: GDObjConfig,
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
    config: GDObjConfig,
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
pub fn follow_trigger(
    config: GDObjConfig,
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

/// Returns a follow trigger object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Objects to animate
/// * `animation`: Animation ID, provided in [`Anim`] enum
pub fn animate_trigger(config: GDObjConfig, target_group: i16, animation: Anim) -> GDObject {
    GDObject::new(
        TRIGGER_ANIMATE,
        config,
        vec![
            (TARGET_ITEM, GDValue::Group(target_group)),
            (ANIMATION_ID, GDValue::Int(animation.as_i32())),
        ],
    )
}

pub fn count_trigger(
    config: GDObjConfig,
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
/// Chances are considered relative to each other, meaning that they are not
/// precentage-based. Two groups with the same relative chance will have the same
/// (50-50) chance to be triggered
pub fn advanced_random_trigger(config: GDObjConfig, probabilities: Vec<(i16, i32)>) -> GDObject {
    GDObject::new(
        TRIGGER_ADVANCED_RANDOM,
        config,
        vec![(
            RANDOM_PROBABLITIES_LIST,
            GDValue::from_prob_list(probabilities),
        )],
    )
}

/* TODO: trigger constructors
 * 2nd part of basics
 * pulse trigger
 *
 * Animation triggers
 * rotate trigger
 * scale trigger
 * animate trigger
 * frame trigger
 * follor player y
 * advanced follow
 * edit advanced follow
 * re-target advanced follow
 * keyframe setup trigger
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
 * switch mg
 * middleground config
 *
 * Item triggers
 * touch trigger
 * instant count trigger
 * pickup trigger
 *
 * Spawner triggers
 * advanced random
 * sequence
 * event trigger
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
 * Time triggers
 * time trigger
 *
 * Misc.
 * ui config
 * bpm marker
 * gradient
 *
 * Collision blocks
 * collision trigger
 * instant collision
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
