//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use serde_json::{json, Value};
use crate::{gdobj::{GDObjConfig, GDObjProperties, GDObject}, utils::clamp_to_values};

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
    Swing = 7
}

/// Enum for stop trigger modes
#[repr(i32)]
pub enum StopMode {
    Stop = 0,
    Pause = 1,
    Resume = 2
}

/// Enum for item alignments
#[repr(i32)]
pub enum ItemAlign {
    Center = 0,
    Left = 1,
    Right = 2
}

/// Enum for counter modes
#[repr(i32)]
pub enum CounterMode {
    Attempts = -3,
    Points = -2,
    MainTime = -1
}

/// Enum for transition object enter/exit config
#[repr(i32)]
#[derive(PartialEq)]
pub enum TransitionMode {
    Both = 0,
    Enter = 1,
    Exit = 2
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
    None = 1915
}

/// Enum for counter types
#[repr(i32)]
pub enum ItemType {
    Counter = 1,
    Timer = 2,
    Points = 3,
    MainTime = 4,
    Attempts = 5
}

/// Enum for item operators
#[repr(i32)]
pub enum Op {
    Set = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4
}

/// Enum for item round modes
#[repr(i32)]
pub enum RoundMode {
    None = 0,
    Nearest = 1,
    Floor = 2,
    Ceiling = 3
}

/// Enum for item sign modes
#[repr(i32)]
pub enum SignMode {
    None = 0,
    Absolute = 1,
    Negative = 2
}

/// Enum for colour channels and their IDs
#[repr(i32)]
pub enum ColourChannel {
    Background = 1000,
    Ground1 = 1001,
    Ground2 = 1009,
    Line = 1002,
    Object = 1004,
    ThreeDLine = 1003,
    MiddleGround = 1013,
    MiddleGround2 = 1014,
    P1 = 1005,
    P2 = 1006
}

/// Enum for target player in gravity trigger
#[repr(i32)]
pub enum TargetPlayer {
    Player1 = 138,
    Player2 = 200,
    PlayerTarget = 201
}

/// Enum for move mode setting. See structs [`DefaultMove`], [`TargetMove`], and [`DirectionalMove`]
pub enum MoveMode {
    Default(DefaultMove),
    Targeting(TargetMove),
    Directional(DirectionalMove)
}

/// Enum for lock config: player or camera
pub enum MoveLock {
    Player,
    Camera
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
    pub y_lock: Option<MoveLock>
}

/// Config struct for moving to a specific target.
/// # Fields
/// * `target_group_id`: Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
/// * `center_group_id`: (Optional) The objects that represent the center of the group that is moving
/// * `axis_only`: Optional axis restriction. Use constants `MOVE_X_ONLY` and `MOVE_Y_ONLY` to specify axis.
pub struct TargetMove {
    pub target_group_id: i32,
    pub center_group_id: Option<i32>,
    pub axis_only: Option<i32>
}

/// Config struct for moving to a specific target.
/// # Fields
/// * `target_group_id`: Group that will be moved to. Use `POS_PLAYER1` and `POS_PLAYER2` consts to specify moving to one of the players.
/// * `center_group_id`: (Optional) The objects that represent the center of the group that is moving
/// * `distance`: Distance in units to move in the direction of the target objects.
pub struct DirectionalMove {
    pub target_group_id: i32,
    pub center_group_id: Option<i32>,
    pub distance: i32
}

/// Enum for all the move easings 
#[repr(i32)]
pub enum MoveEasing {
    EaseInOut = 1,
    EaseIn = 2,
    EaseOut = 3,
    ElasticInOut = 4,
    ElasticIn = 5,
    ElasticOut = 6,
    BounceInOut = 7,
    BounceIn = 8,
    BounceOut = 9,
    ExponentialInOut = 10,
    ExponentialIn = 11,
    ExponentialOut = 12,
    SineInOut = 13,
    SineIn = 14,
    SineOut = 15,
    BackInOut = 16,
    BackIn = 17,
    BackOut = 18
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
    time: f32,
    target_group: i32,
    silent: bool,
    dynamic: bool,
    easing: Option<(MoveEasing, f32)>
) -> GDObject { 
    // aim: target group 2
    let mut properties = json!({
        "51": target_group,
        "10": time,
        "393": 1, // small step option
        "397": dynamic as i32,
        "544": silent as i32
    });
    let map = properties.as_object_mut().unwrap();
    if let Some((easing, rate)) = easing {
        map.insert("30".to_string(), Value::from(easing as i32));
        map.insert("85".to_string(), Value::from(rate));
    }

    match move_config {
        MoveMode::Default(config) => {
            if let Some(lock) = config.x_lock {
                map.insert(
                    match lock {
                        MoveLock::Player => "58",
                        MoveLock::Camera => "141"
                    }.to_string(), 
                    Value::from(1)
                );
                map.insert("143".to_string(), Value::from(config.dx as f32));
            } else {
                map.insert("28".to_string(), Value::from(config.dx as i32));
            }

            if let Some(lock) = config.y_lock {
                map.insert(
                    match lock {
                        MoveLock::Player => "59",
                        MoveLock::Camera => "142"
                    }.to_string(), 
                    Value::from(1)
                );
                map.insert("144".to_string(), Value::from(config.dy as f32));
            } else {
                map.insert("29".to_string(), Value::from(config.dy as i32));
            }
        },
        MoveMode::Targeting(config) => {
            map.insert("100D".to_string(), Value::from(1));
            if let Some(id) = config.center_group_id {
                map.insert("395".to_string(), Value::from(id));
            }
            
            if let Some(axis) = config.axis_only {
                map.insert("101".to_string(), Value::from(axis));
            }

            match config.target_group_id {
                POS_PLAYER1 => map.insert("138".to_string(), Value::from(1)),
                POS_PLAYER2 => map.insert("200".to_string(), Value::from(1)),
                id => map.insert("71".to_string(), Value::from(id)),
            };
        },
        MoveMode::Directional(config) => {
        if let Some(id) = config.center_group_id {  
                map.insert("395".to_string(), Value::from(id));
            }

            match config.target_group_id {
                POS_PLAYER1 => map.insert("138".to_string(), Value::from(1)),
                POS_PLAYER2 => map.insert("200".to_string(), Value::from(1)),
                id => map.insert("71".to_string(), Value::from(id)),
            };

            map.insert("394".to_string(), Value::from(1));
            map.insert("396".to_string(), Value::from(config.distance));
        },
    }

    GDObject::new(901, config, GDObjProperties::from_json(properties))
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
    disabled: bool
) -> GDObject {
    let start_speed = clamp_to_values(start_speed, &[0.5, 1.0, 2.0, 3.0, 4.0]);

    let properties = json!({
        "kA4": match start_speed {
            0.5 => "1",
            2.0 => "2",
            3.0 => "3",
            4.0 => "4",
            _ => "0"
        },
        "kA2": starting_gamemode as i32,
        "kA3": starting_as_mini as i32,
        "kA8": starting_as_dual as i32,
        "kA21": disabled as i32,
        "kA28": starting_mirrored as i32,
        "kA29": rotate_gameplay as i32,
        "kA20": reverse_gameplay as i32,
        "kA19": target_order,
        "kA26": target_channel,
        "kA35": reset_camera as i32,
        // and then whatever the fuck these are
        "kA10": "0",
        "kA11": "",
        "kA20": "1",
        "kA22": "0",
        "kA23": "0",
        "kA24": "0",
        "kA27": "1",
        "kA31": "1",
        "kA32": "1",
        "kA33": "1",
        "kA34": "1",
        "kA36": "0",
        "kA37": "1",
        "kA38": "1",
        "kA39": "1",
        "kA40": "1",
        "kA41": "1",
        "kA42": "1",
        "kA43": "0",
        "kA44": "0",
        "kA45": "1",
        "kA46": "0",
        "kA9": "1"
    });

    GDObject::new(31, config, GDObjProperties::from_json(properties))
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
pub fn colour_trigger<T: Into<i32>>(
    config: GDObjConfig,
    colour: (u8, u8, u8),
    channel: T,
    fade_time: f32,
    opacity: f32,
    blending: bool,
    use_player_col_1: bool,
    use_player_col_2: bool,
    copy_colour: Option<(T, i32, f32, f32, bool, bool, bool, bool)>
) -> GDObject {
    let mut properties = json!({
        "7": colour.0,
        "8": colour.1,
        "9": colour.2,
        "10": fade_time,
        "15": use_player_col_1 as i32,
        "23": channel.into(),
        "16": use_player_col_2 as i32,
        "35": opacity
    });

    let map = properties.as_object_mut().unwrap();

    if blending {
        map.insert("17".to_string(), Value::from(""));
    }

    if let Some((channel, hue, saturation, lightness, static_sat_scalar, static_brightness_scalar, legacy_hsv, copy_opacity)) = copy_colour {
        let mut cfg_string = format!("{hue}a{saturation}a{lightness}a{}a", static_sat_scalar as i32);
        if !legacy_hsv {
            cfg_string += &format!("{}", static_brightness_scalar as i32);
            map.insert("210".to_string(), Value::from(""));
        }
        if copy_opacity {
            map.insert("60".to_string(), Value::from("1"));
        }
        map.insert("49".to_string(), Value::from(cfg_string));
        map.insert("50".to_string(), Value::from(channel.into()));
    }

    GDObject::new(899, config, GDObjProperties::from_json(properties))
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
    use_control_id: bool
) -> GDObject {
    let properties = GDObjProperties::from_json(json!({
        "51": target_group,
        "535": use_control_id as i32,
        "580": stop_mode as i32
    }));

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
    opacity: f32,
    fade_time: f32
) -> GDObject {
    GDObject::new(1007, config, GDObjProperties::from_json(json!({
        "10": fade_time,
        "35": opacity,
        "51": target_group
    })))
}

/// Returns a toggle trigger
/// 
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Target group to stop/pause/resume
/// * `activate_group`: Active group instead of deactivating?
pub fn toggle_trigger(
    config: GDObjConfig,
    target_group: i32,
    activate_group: bool
) -> GDObject {
    let mut properties = json!({
        "51": target_group,
        "64": "1",
        "67": ""
    });
    let map = properties.as_object_mut().unwrap();
    if activate_group {
        map.insert("56".to_string(), Value::from("1"));
    }
    GDObject::new(1049, config, GDObjProperties::from_json(properties))
}

/// dont call this
pub fn pulse_trigger(
    config: GDObjConfig
) {}

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
    target_channel: Option<i32>
) -> GDObject {
    let mut properties = json!({});
    let map = properties.as_object_mut().unwrap();

    if mode != TransitionMode::Both {
        map.insert("217".to_string(), Value::from(mode as i32));
    }
    if let Some(channel) = target_channel {
        map.insert("344".to_string(), Value::from(channel));
    }
    
    GDObject::new(transition as i32, config, GDObjProperties::from_json(properties))
}

// misc stuff

/// Returns a reverse gameplay trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn reverse_gameplay(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(1917, config, GDObjProperties::new())
}

/// Returns a link visible trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is linked visibly
pub fn link_visible(
    config: GDObjConfig,
    target_group: i32
) -> GDObject {
    GDObject::new(3662, config, GDObjProperties::from_json(json!({
        "51": target_group
    })))
}

/// Returns a timewarp trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `time_scale`: How much to speed up/slow down time by. 1.0 is the default
pub fn timewarp(
    config: GDObjConfig,
    time_scale: f32
) -> GDObject {
    GDObject::new(1935, config, GDObjProperties::from_json(json!({
        "120": time_scale
    })))
}

/// Returns a trigger that shows the player
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn show_player(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(1613, config, GDObjProperties::new())
}

/// Returns a trigger that hides the player
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn hide_player(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(1612, config, GDObjProperties::new())
}

/// Returns a trigger that shows the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn show_player_trail(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(32, config, GDObjProperties::new())
}

/// Returns a trigger that hides the player trail
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn hide_player_trail(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(33, config, GDObjProperties::new())
}

/// Returns a trigger that enables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn bg_effect_on(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(1818, config, GDObjProperties::new())
}

/// Returns a trigger that disables the background effect
/// # Arguments
/// * `config`: General object options, such as position and scale
pub fn bg_effect_off(
    config: GDObjConfig
) -> GDObject {
    GDObject::new(1819, config, GDObjProperties::new())
}

/// Returns a group reset trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: group that is to be reset
pub fn group_reset(
    config: GDObjConfig,
    target_group: i32
) -> GDObject {
    GDObject::new(3618, config, GDObjProperties::from_json(json!({
        "51": target_group
    })))
}

/// Returns a shake trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `strength`: Strength of shake
/// * `interval`: Interval in seconds between each shake
/// * `duration`: Total duration of shaking
pub fn shake_trigger(
    config: GDObjConfig,
    strength: i32,
    interval: f32,
    duration: f32
) -> GDObject {
    GDObject::new(1520, config, GDObjProperties::from_json(json!({
        "75": strength,
        "84": interval,
        "10": duration
    })))
}

/// Returns a background speed config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `mod_x`: X-axis speed of BG in terms of player speed. Default is 0.3 
/// * `mod_y`: Y-axis speed of BG in terms of player speed. Default is 0.5
pub fn bg_speed(
    config: GDObjConfig,
    mod_x: f32,
    mod_y: f32
) -> GDObject {
    GDObject::new(3606, config, GDObjProperties::from_json(json!({
        "143": mod_x,
        "144": mod_y
    })))
}

/// Returns a middleground speed config trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `mod_x`: X-axis speed of MG in terms of player speed. Default is 0.3 
/// * `mod_y`: Y-axis speed of MG in terms of player speed. Default is 0.5
pub fn mg_speed(
    config: GDObjConfig,
    mod_x: f32,
    mod_y: f32
) -> GDObject {
    GDObject::new(3612, config, GDObjProperties::from_json(json!({
        "143": mod_x,
        "144": mod_y
    })))
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
    stop_slide: bool
) -> GDObject {
    GDObject::new(1932, config, GDObjProperties::from_json(json!({
        "138": p1 as i32,
        "200": p2 as i32,
        "540": stop_jump as i32,
        "541": stop_move as i32,
        "542": stop_rotation as i32,
        "543": stop_slide as i32,
    })))
}

/// Returns a gravity trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `gravity`: how much gravity.
/// * `target_player`: (Optional) Player target for this gravity trigger
pub fn gravity_trigger(
    config: GDObjConfig,
    gravity: f32,
    target_player: Option<TargetPlayer>
) -> GDObject {
    let mut properties= json!({
        "148": gravity
    });
    let props = properties.as_object_mut().unwrap();
    if let Some(player) = target_player {
        props.insert(format!("{}", player as i32), Value::from("1"));
    }
    GDObject::new(2066, config, GDObjProperties::from_json(properties))
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
    no_sfx: bool
) -> GDObject {
    let mut properties= json!({
        "460": no_effects as i32,
        "461": instant as i32,
        "467": no_sfx as i32
    });
    let props = properties.as_object_mut().unwrap();

    if let Some(id) = spawn_id {
        props.insert("51".to_string(), Value::from(id));
    }

    if let Some(pos) = target_pos {
        props.insert("71".to_string(), Value::from(pos));
    }

    GDObject::new(3600, config, GDObjProperties::from_json(properties))
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
    special_mode: Option<CounterMode>
) -> GDObject {
    let mut properties = json!({
        "80": item_id,
        "389": seconds_only as i32,
        "391": align as i32,
        "466": timer as i32
    });

    let map = properties.as_object_mut().unwrap();
    if let Some(mode) = special_mode {
        map.insert("390".to_string(), Value::from(mode as i32));
    }

    GDObject::new(1615, config, GDObjProperties::from_json(properties))
}

/// Returns an item edit trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `operand1`: Optional first operand, tuple of (ID, item type)
/// * `operand2`: Optional second operand, tuple of (ID, item type)
/// * `target_id`: Target item id
/// * `target_type`: Target item type
/// * `modifier`: f32 modifier; default is 1.0
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
    modifier: f32,
    assign_op: Op,
    mod_op: Option<Op>,
    id_op: Option<Op>,
    id_rounding: RoundMode,
    result_rounding: RoundMode,
    id_sign: SignMode,
    result_sign: SignMode
) -> GDObject {
    let mod_op = match mod_op {
        Some(op) => op,
        None => Op::Mul
    };
    let id_op = match id_op {
        Some(op) => op,
        None => Op::Add
    };

    let op_1 = match operand1 {
        Some(cfg) => cfg,
        None => (0, ItemType::Counter)
    };
    let op_2 = match operand2 {
        Some(cfg) => cfg,
        None => (0, ItemType::Counter)
    };

    GDObject::new(3619, config, GDObjProperties::from_json(json!({
        "36": 1,
        "51": target_id,
        "80": op_1.0,
        "95": op_2.0,
        "476": op_1.1 as i32,
        "477": op_2.1 as i32,
        "478": target_type as i32,
        "479": modifier,
        "480": assign_op as i32,
        "481": id_op as i32,
        "482": mod_op as i32,
        "485": id_rounding as i32,
        "486": result_rounding as i32,
        "578": id_sign as i32,
        "579": result_sign as i32,
    })))
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
    reset: bool
) -> GDObject {
    GDObject::new(3641, config, GDObjProperties::from_json(json!({
        "51": item_id,
        "491": persistent as i32,
        "492": target_all as i32,
        "493": reset as i32,
        "494": timer as i32,
    })))
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
    chance: f32,
    target_group1: i32,
    target_group2: i32
) -> GDObject {
    GDObject::new(1912, config, GDObjProperties::from_json(json!({
        "51": target_group1,
        "71": target_group2,
        "10": chance
    })))
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
    delay: f32,
    delay_variation: f32,
    reset_remap: bool,
    spawn_ordered: bool,
    preview_disable: bool
) -> GDObject {
    GDObject::new(1268, config, GDObjProperties::from_json(json!({
        "51": spawn_id,
        "63": delay,
        "102": preview_disable as i32,
        "441": spawn_ordered as i32,
        "556": delay_variation,
        "581": reset_remap as i32
    })))
}

/// Returns an on-death trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `target_group`: Spawns this group
/// * `activate_group`: Activate this group (instead of toggling off)?
pub fn on_death(
    config: GDObjConfig,
    target_group: i32,
    activate_group: bool
) -> GDObject {
    GDObject::new(1812, config, GDObjProperties::from_json(json!({
        "51": target_group,
        "56": activate_group as i32
    })))
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
    scale_config: Option<(f32, f32)>,
    match_rotation: bool
) -> GDObject {
    let mut properties = json!({
        "51": particle_group,
        "71": position_group,
        "551": match_rotation
    });

    let map = properties.as_object_mut().unwrap();
    if let Some((x, y)) = position_offsets {
        map.insert("547".to_owned(), Value::from(x));
        map.insert("548".to_owned(), Value::from(y));
    }

    if let Some((x, y)) = position_variation {
        map.insert("549".to_owned(), Value::from(x));
        map.insert("550".to_owned(), Value::from(y));
    }

    if let Some((rot, var)) = rotation_config {
        map.insert("552".to_owned(), Value::from(rot));
        map.insert("553".to_owned(), Value::from(var));
    }

    if let Some((scale, var)) = scale_config {
        map.insert("554".to_owned(), Value::from(scale));
        map.insert("555".to_owned(), Value::from(var));
    }

    GDObject::new(3608, config, GDObjProperties::from_json(properties))
}
 
// collision blocks

/// Returns a collision block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `id`: Collision block ID
/// * `dynamic`: Does this block register collisions with other collision blocks? 
pub fn collision_block(
    config: GDObjConfig,
    id: i32,
    dynamic: bool
) -> GDObject {
    GDObject::new(1816, config, GDObjProperties::from_json(json!({
        "80": id,
        "94": dynamic as i32
    })))
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
    spawn_only: bool
) -> GDObject {
    GDObject::new(3643, config, GDObjProperties::from_json(json!({
        "51": target_group,
        "56": activate_group as i32,
        "99": multi_activate as i32,
        "445": claim_touch as i32,
        "504": spawn_only as i32,
    })))
}

/// Returns a state block object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `state_on`: Group that is activated when the player enters this block's hitbox
/// * `state_off`: Group that is activated when the player exits this block's hitbox
pub fn state_block(
    config: GDObjConfig,
    state_on: i32,
    state_off: i32
) -> GDObject {
    GDObject::new(3640, config, GDObjProperties::from_json(json!({
        "51": state_on,
        "71": state_off
    })))
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
pub fn time_control(
    config: GDObjConfig,
    id: i32,
    stop: bool
) -> GDObject {
    GDObject::new(3617, config, GDObjProperties::from_json(json!({
        "80": id,
        "472": stop as i32
    })))
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
    target_time: f32,
    multi_activate: bool
) -> GDObject {
    GDObject::new(3615, config, GDObjProperties::from_json(json!({
        "80": id,
        "51": target_group,
        "473": target_time,
        "475": multi_activate as i32
    })))
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
    zoom: f32,
    time: f32,
    easing: Option<(MoveEasing, f32)>
) -> GDObject {
    let mut properties = json!({
        "10": time,
        "371": zoom
    });

    let map = properties.as_object_mut().unwrap();
    if let Some((easing, rate)) = easing {
        map.insert("30".to_string(), Value::from(easing as i32));
        map.insert("85".to_string(), Value::from(rate));
    }
    GDObject::new(1913, config, GDObjProperties::from_json(properties))
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
    zoom: f32,
    offset_x: i32,
    offset_y: i32,
    opacity: f32,
) -> GDObject {
    GDObject::new(1913, config, GDObjProperties::from_json(json!({
        "28": offset_x,
        "29": offset_y,
        "371": zoom,
        "506": opacity
    })))
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
 * item compare
 * 
 * Spawner triggers
 * random trigger
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