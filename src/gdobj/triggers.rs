//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use crate::{
    core::clamp_to_values,
    gdobj::{ColourChannel, GDObjConfig, GDObject, GDValue, MoveEasing},
};

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
#[repr(i32)]
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
    target_group: i32,
    silent: bool,
    dynamic: bool,
    easing: Option<(MoveEasing, f64)>,
) -> GDObject {
    // aim: target group 2
    let mut properties = vec![
        (51, GDValue::Int(target_group)),
        (10, GDValue::Float(time)),
        (393, GDValue::Int(1)),
        (397, GDValue::Int(dynamic as i32)),
        (544, GDValue::Int(silent as i32)),
    ];

    if let Some((easing, rate)) = easing {
        properties.push((30, GDValue::Int(easing as i32)));
        properties.push((85, GDValue::Float(rate)));
    }

    match move_config {
        MoveMode::Default(config) => {
            if let Some(lock) = config.x_lock {
                properties.push((
                    match lock {
                        MoveLock::Player => 58,
                        MoveLock::Camera => 141,
                    },
                    GDValue::Int(1),
                ));
                properties.push((143, GDValue::Float(config.dx as f64)));
            } else {
                properties.push((28, GDValue::Int(config.dx as i32)));
            }

            if let Some(lock) = config.y_lock {
                properties.push((
                    match lock {
                        MoveLock::Player => 59,
                        MoveLock::Camera => 142,
                    },
                    GDValue::Int(1),
                ));
                properties.push((144, GDValue::Float(config.dy as f64)));
            } else {
                properties.push((29, GDValue::Int(config.dy as i32)));
            }
        }
        MoveMode::Targeting(config) => {
            properties.push((100, GDValue::Int(1)));
            if let Some(id) = config.center_group_id {
                properties.push((395, GDValue::Int(id)));
            }

            if let Some(axis) = config.axis_only {
                properties.push((101, GDValue::Int(axis)));
            }

            match config.target_group_id {
                POS_PLAYER1 => properties.push((138, GDValue::Int(1))),
                POS_PLAYER2 => properties.push((200, GDValue::Int(1))),
                id => properties.push((71, GDValue::Int(id))),
            };
        }
        MoveMode::Directional(config) => {
            if let Some(id) = config.center_group_id {
                properties.push((395, GDValue::Int(id)));
            }

            match config.target_group_id {
                POS_PLAYER1 => properties.push((138, GDValue::Int(1))),
                POS_PLAYER2 => properties.push((200, GDValue::Int(1))),
                id => properties.push((71, GDValue::Int(id))),
            };

            properties.push((394, GDValue::Int(1)));
            properties.push((396, GDValue::Int(config.distance)));
        }
    }

    GDObject::new(901, config, properties)
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
/// * **NOTE**: Defaults are the default values of a startpos, they are NOT filled in for you.
///
/// # ⚠️ Warning
/// This object is VERY WEIRD. There are 25 properties that serve an unknown purpose.
/// This is also the only object with non-integer properties (kA1, kA2, ...)
/// The reverse gameplay option is always on when generated with GDLib for some unknown reason. USE AT YOUR OWN RISK!!
pub fn start_pos(
    config: GDObjConfig,
    start_speed: f64,
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
    let start_speed = clamp_to_values(start_speed, &[0.5, 1.0, 2.0, 3.0, 4.0]);

    let properties = vec![
        (
            10004,
            GDValue::Int(match start_speed {
                0.5 => 1,
                2.0 => 2,
                3.0 => 3,
                4.0 => 4,
                _ => 0,
            }),
        ),
        (10002, GDValue::Int(starting_gamemode as i32)),
        (10003, GDValue::Int(starting_as_mini as i32)),
        (10008, GDValue::Int(starting_as_dual as i32)),
        (10021, GDValue::Int(disabled as i32)),
        (10028, GDValue::Int(starting_mirrored as i32)),
        (10029, GDValue::Int(rotate_gameplay as i32)),
        (10020, GDValue::Int(reverse_gameplay as i32)),
        (10019, GDValue::Int(target_order)),
        (10026, GDValue::Int(target_channel)),
        (10035, GDValue::Int(reset_camera as i32)),
        (10010, GDValue::Int(0)),
        (10011, GDValue::String("".to_string())),
        (10020, GDValue::Int(1)),
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
    ];

    GDObject::new(31, config, properties)
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
        (7, GDValue::Int(colour.0 as i32)),
        (8, GDValue::Int(colour.1 as i32)),
        (9, GDValue::Int(colour.2 as i32)),
        (10, GDValue::Float(fade_time)),
        (15, GDValue::Int(use_player_col_1 as i32)),
        (23, GDValue::Int(channel.as_i32())),
        (16, GDValue::Int(use_player_col_2 as i32)),
        (35, GDValue::Float(opacity)),
    ];

    if blending {
        properties.push((17, GDValue::Bool(true)));
    }

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
            properties.push((210, GDValue::Bool(true)));
        }
        if copy_opacity {
            properties.push((60, GDValue::Bool(true)));
        }
        properties.push((49, GDValue::String(cfg_string)));
        properties.push((50, GDValue::Int(channel.as_i32())));
    }

    GDObject::new(899, config, properties)
}

/// Returns a stop trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `stop_mode`: Stop mode (see [`StopMode`] struct)
/// * `use_control_id`: Only stops certain triggers within a group if enabled.
pub fn stop_trigger(
    config: GDObjConfig,
    target_group: i32,
    stop_mode: StopMode,
    use_control_id: bool,
) -> GDObject {
    let properties = vec![
        (51, GDValue::Int(target_group)),
        (535, GDValue::Int(use_control_id as i32)),
        (580, GDValue::Int(stop_mode as i32)),
    ];

    GDObject::new(1616, config, properties)
}

/// Returns an alpha trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `opacity`: Opacity to set group at
/// * `fade_time`: Time to fade to the opacity
pub fn alpha_trigger(
    config: GDObjConfig,
    target_group: i32,
    opacity: f64,
    fade_time: f64,
) -> GDObject {
    GDObject::new(
        1007,
        config,
        vec![
            (10, GDValue::Float(fade_time)),
            (35, GDValue::Float(opacity)),
            (51, GDValue::Int(target_group)),
        ],
    )
}

/// Returns a toggle trigger
///
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `activate_group`: Active group instead of deactivating?
pub fn toggle_trigger(config: GDObjConfig, target_group: i32, activate_group: bool) -> GDObject {
    let mut properties = vec![
        (51, GDValue::Int(target_group)),
        (64, GDValue::Bool(true)),
        (67, GDValue::Bool(true)),
    ];

    if activate_group {
        properties.push((56, GDValue::Bool(true)));
    }
    GDObject::new(1049, config, properties)
}

/// dont call this
pub fn pulse_trigger(config: GDObjConfig) {}

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
        properties.push((217, GDValue::Int(mode as i32)));
    }
    if let Some(channel) = target_channel {
        properties.push((344, GDValue::Int(channel)));
    }

    GDObject::new(transition as i32, config, properties)
}

// misc stuff

/// Returns a reverse gameplay trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn reverse_gameplay(config: GDObjConfig) -> GDObject {
    GDObject::new(1917, config, vec![])
}

/// Returns a link visible trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is linked visibly
pub fn link_visible(config: GDObjConfig, target_group: i32) -> GDObject {
    GDObject::new(3662, config, vec![(51, GDValue::Int(target_group))])
}

/// Returns a timewarp trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `time_scale`: How much to speed up/slow down time by. 1.0 is the default
pub fn timewarp(config: GDObjConfig, time_scale: f64) -> GDObject {
    GDObject::new(1935, config, vec![(120, GDValue::Float(time_scale))])
}

/// Returns a trigger that shows the player
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn show_player(config: GDObjConfig) -> GDObject {
    GDObject::new(1613, config, vec![])
}

/// Returns a trigger that hides the player
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn hide_player(config: GDObjConfig) -> GDObject {
    GDObject::new(1612, config, vec![])
}

/// Returns a trigger that shows the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn show_player_trail(config: GDObjConfig) -> GDObject {
    GDObject::new(32, config, vec![])
}

/// Returns a trigger that hides the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn hide_player_trail(config: GDObjConfig) -> GDObject {
    GDObject::new(33, config, vec![])
}

/// Returns a trigger that enables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn bg_effect_on(config: GDObjConfig) -> GDObject {
    GDObject::new(1818, config, vec![])
}

/// Returns a trigger that disables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn bg_effect_off(config: GDObjConfig) -> GDObject {
    GDObject::new(1819, config, vec![])
}

/// Returns a group reset trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is to be reset
pub fn group_reset(config: GDObjConfig, target_group: i32) -> GDObject {
    GDObject::new(3618, config, vec![(51, GDValue::Int(target_group))])
}

/// Returns a shake trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `strength`: Strength of shake
/// * `interval`: Interval in seconds between each shake
/// * `duration`: Total duration of shaking
pub fn shake_trigger(config: GDObjConfig, strength: i32, interval: f64, duration: f64) -> GDObject {
    GDObject::new(
        1520,
        config,
        vec![
            (75, GDValue::Int(strength)),
            (84, GDValue::Float(interval)),
            (10, GDValue::Float(duration)),
        ],
    )
}

/// Returns a background speed config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `mod_x`: X-axis speed of BG in terms of player speed. Default is 0.3
/// * `mod_y`: Y-axis speed of BG in terms of player speed. Default is 0.5
pub fn bg_speed(config: GDObjConfig, mod_x: f64, mod_y: f64) -> GDObject {
    GDObject::new(
        3606,
        config,
        vec![(143, GDValue::Float(mod_x)), (144, GDValue::Float(mod_y))],
    )
}

/// Returns a middleground speed config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `mod_x`: X-axis speed of MG in terms of player speed. Default is 0.3
/// * `mod_y`: Y-axis speed of MG in terms of player speed. Default is 0.5
pub fn mg_speed(config: GDObjConfig, mod_x: f64, mod_y: f64) -> GDObject {
    GDObject::new(
        3612,
        config,
        vec![(143, GDValue::Float(mod_x)), (144, GDValue::Float(mod_y))],
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
        1932,
        config,
        vec![
            (138, GDValue::Int(p1 as i32)),
            (200, GDValue::Int(p2 as i32)),
            (540, GDValue::Int(stop_jump as i32)),
            (541, GDValue::Int(stop_move as i32)),
            (542, GDValue::Int(stop_rotation as i32)),
            (543, GDValue::Int(stop_slide as i32)),
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
    let mut properties = vec![(148u16, GDValue::Float(gravity))];

    if let Some(player) = target_player {
        properties.push((player as i32 as u16, GDValue::Bool(true)));
    }
    GDObject::new(2066, config, properties)
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
        (460, GDValue::Int(no_effects as i32)),
        (461, GDValue::Int(instant as i32)),
        (467, GDValue::Int(no_sfx as i32)),
    ];

    if let Some(id) = spawn_id {
        properties.push((51, GDValue::Int(id)));
    }

    if let Some(pos) = target_pos {
        properties.push((71, GDValue::Int(pos)));
    }

    GDObject::new(3600, config, properties)
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
    item_id: i32,
    timer: bool,
    align: ItemAlign,
    seconds_only: bool,
    special_mode: Option<CounterMode>,
) -> GDObject {
    let mut properties = vec![
        (80, GDValue::Int(item_id)),
        (389, GDValue::Int(seconds_only as i32)),
        (391, GDValue::Int(align as i32)),
        (466, GDValue::Int(timer as i32)),
    ];

    if let Some(mode) = special_mode {
        properties.push((390, GDValue::Int(mode as i32)));
    }

    GDObject::new(1615, config, properties)
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
    target_id: i32,
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
        3619,
        config,
        vec![
            (36, GDValue::Int(1)),
            (51, GDValue::Int(target_id)),
            (80, GDValue::Int(op_1.0)),
            (95, GDValue::Int(op_2.0)),
            (476, GDValue::Int(op_1.1 as i32)),
            (477, GDValue::Int(op_2.1 as i32)),
            (478, GDValue::Int(target_type as i32)),
            (479, GDValue::Float(modifier)),
            (480, GDValue::Int(assign_op as i32)),
            (481, GDValue::Int(id_op as i32)),
            (482, GDValue::Int(mod_op as i32)),
            (485, GDValue::Int(id_rounding as i32)),
            (486, GDValue::Int(result_rounding as i32)),
            (578, GDValue::Int(id_sign as i32)),
            (579, GDValue::Int(result_sign as i32)),
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
        (51, GDValue::Int(true_id)),
        (71, GDValue::Int(false_id)),
        // ids
        (80, GDValue::Int(lhs.0)),
        (95, GDValue::Int(rhs.0)),
        // types
        (476, GDValue::Int(lhs.1 as i32)),
        (477, GDValue::Int(rhs.1 as i32)),
        // modifiers
        (479, GDValue::Float(lhs.2)),
        (483, GDValue::Float(rhs.2)),
        // modifiers ops
        (480, GDValue::Int(lhs.3 as i32)),
        (481, GDValue::Int(rhs.3 as i32)),
        (482, GDValue::Int(compare_op as i32)),
        (484, GDValue::Float(tolerance)),
        // round modes
        (485, GDValue::Int(lhs.4 as i32)),
        (486, GDValue::Int(rhs.4 as i32)),
        // sign modes
        (578, GDValue::Int(lhs.5 as i32)),
        (579, GDValue::Int(rhs.5 as i32)),
    ];

    GDObject::new(3620, config, properties)
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
    item_id: i32,
    timer: bool,
    persistent: bool,
    target_all: bool,
    reset: bool,
) -> GDObject {
    GDObject::new(
        3641,
        config,
        vec![
            (51, GDValue::Int(item_id)),
            (491, GDValue::Int(persistent as i32)),
            (492, GDValue::Int(target_all as i32)),
            (493, GDValue::Int(reset as i32)),
            (494, GDValue::Int(timer as i32)),
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
    target_group1: i32,
    target_group2: i32,
) -> GDObject {
    GDObject::new(
        1912,
        config,
        vec![
            (51, GDValue::Int(target_group1)),
            (71, GDValue::Int(target_group2)),
            (10, GDValue::Float(chance)),
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
        1268,
        config,
        vec![
            (51, GDValue::Int(spawn_id)),
            (63, GDValue::Float(delay)),
            (102, GDValue::Int(preview_disable as i32)),
            (441, GDValue::Int(spawn_ordered as i32)),
            (556, GDValue::Float(delay_variation)),
            (581, GDValue::Int(reset_remap as i32)),
        ],
    )
}

/// Returns an on-death trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Spawns this group
/// * `activate_group`: Activate this group (instead of toggling off)?
pub fn on_death(config: GDObjConfig, target_group: i32, activate_group: bool) -> GDObject {
    GDObject::new(
        1812,
        config,
        vec![
            (51, GDValue::Int(target_group)),
            (56, GDValue::Int(activate_group as i32)),
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
        (51, GDValue::Int(particle_group)),
        (71, GDValue::Int(position_group)),
        (551, GDValue::Bool(match_rotation)),
    ];

    if let Some((x, y)) = position_offsets {
        properties.push((547, GDValue::Int(x)));
        properties.push((548, GDValue::Int(y)));
    }

    if let Some((x, y)) = position_variation {
        properties.push((549, GDValue::Int(x)));
        properties.push((550, GDValue::Int(y)));
    }

    if let Some((rot, var)) = rotation_config {
        properties.push((552, GDValue::Int(rot)));
        properties.push((553, GDValue::Int(var)));
    }

    if let Some((scale, var)) = scale_config {
        properties.push((554, GDValue::Float(scale)));
        properties.push((555, GDValue::Float(var)));
    }

    GDObject::new(3608, config, properties)
}

// collision blocks

/// Returns a collision block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Collision block ID
/// * `dynamic`: Does this block register collisions with other collision blocks?
pub fn collision_block(config: GDObjConfig, id: i32, dynamic: bool) -> GDObject {
    GDObject::new(
        1816,
        config,
        vec![(80, GDValue::Int(id)), (94, GDValue::Int(dynamic as i32))],
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
pub fn toggle_block(
    config: GDObjConfig,
    target_group: i32,
    activate_group: bool,
    claim_touch: bool,
    multi_activate: bool,
    spawn_only: bool,
) -> GDObject {
    GDObject::new(
        3643,
        config,
        vec![
            (51, GDValue::Int(target_group)),
            (56, GDValue::Int(activate_group as i32)),
            (99, GDValue::Int(multi_activate as i32)),
            (445, GDValue::Int(claim_touch as i32)),
            (504, GDValue::Int(spawn_only as i32)),
        ],
    )
}

/// Returns a state block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `state_on`: Group that is activated when the player enters this block's hitbox
/// * `state_off`: Group that is activated when the player exits this block's hitbox
pub fn state_block(config: GDObjConfig, state_on: i32, state_off: i32) -> GDObject {
    GDObject::new(
        3640,
        config,
        vec![(51, GDValue::Int(state_on)), (71, GDValue::Int(state_off))],
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
pub fn time_control(config: GDObjConfig, id: i32, stop: bool) -> GDObject {
    GDObject::new(
        3617,
        config,
        vec![(80, GDValue::Int(id)), (472, GDValue::Int(stop as i32))],
    )
}

/// Returns a time event trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Timer ID
/// * `target_group`: If enabled, stops the timer; otherwise, starts the timer.
/// * `target_time`: At what time the timer should be to activate objects in `target_group`.
/// * `multi_activate`: Should this event be triggerable multiple times?
pub fn time_event(
    config: GDObjConfig,
    id: i32,
    target_group: i32,
    target_time: f64,
    multi_activate: bool,
) -> GDObject {
    GDObject::new(
        3615,
        config,
        vec![
            (80, GDValue::Int(id)),
            (51, GDValue::Int(target_group)),
            (473, GDValue::Float(target_time)),
            (475, GDValue::Int(multi_activate as i32)),
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
    let mut properties = vec![(10, GDValue::Float(time)), (371, GDValue::Float(zoom))];

    if let Some((easing, rate)) = easing {
        properties.push((30, GDValue::Int(easing as i32)));
        properties.push((85, GDValue::Float(rate)));
    }
    GDObject::new(1913, config, properties)
}

/// Returns a camera guide object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `zoom`: Zoom of camera guide
/// * `offset_x`: Center offset from this object in x axis
/// * `offset_y`: Center offset from this object in y axis
/// * `opacity`: Opacity of guidelines
pub fn camera_guide(
    config: GDObjConfig,
    zoom: f64,
    offset_x: i32,
    offset_y: i32,
    opacity: f64,
) -> GDObject {
    GDObject::new(
        1913,
        config,
        vec![
            (28, GDValue::Int(offset_x)),
            (29, GDValue::Int(offset_y)),
            (371, GDValue::Float(zoom)),
            (506, GDValue::Float(opacity)),
        ],
    )
}

/* TODO: trigger constructors
 * 2nd part of basics
 * pulse trigger
 *
 * Animation triggers
 * move trigger (all options)
 * rotate trigger
 * scale trigger
 * follow trigger
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
 * count trigger
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
