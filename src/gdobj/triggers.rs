//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use serde_json::{json, Value};
use crate::{gdobj::{GDObjConfig, GDObjProperties, GDObject}, utils::clamp_to_values};

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

/// Enum for operators
#[repr(i32)]
pub enum Op {
    Set = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4
}

/// Enum for round modes
#[repr(i32)]
pub enum RoundMode {
    None = 0,
    Nearest = 1,
    Floor = 2,
    Ceiling = 3
}

/// Enum for sign modes
#[repr(i32)]
pub enum SignMode {
    None = 0,
    Absolute = 1,
    Negative = 2
}

/// Enum for colour channels
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

// tehcnically this aint the full thing but yk its good enough (for now...)
/// Returns a move trigger object
/// 
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `dX`: How much to move the target group in the X direction. 
/// * `dY`: How much to move the target group in the Y direction. 
/// * `time`: Move time for target object.
/// * `targetGroup`: Group that the move trigger is moving.
/// * `targetMode`: Enabled if the group is moving to the location of another group.
/// * `aim`: The other group that the object would move
/// 
/// Returns a GDObject object with the corresponding properties.
pub fn move_trigger(
    config: GDObjConfig,
    dx: i32,
    dy: i32,
    time: f32,
    target_group: i32,
    target_mode: bool,
    aim: i32
) -> GDObject { 
    let properties = if target_mode {
        json!({
            "28": 0,
            "29": 0,
            "30": 0,
            "85": 2,
            "71": aim,
            "100": 1,
            "51": target_group,
            "10": time
        })
    } else {
        json!({
            "28": dx,
            "29": dy,
            "51": target_group,
            "10": time
        })
    };

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
/// Returns a GDObject object with the corresponding properties.
/// 
/// # ⚠️ Warning
/// This object is VERY WEIRD. There are 25 properties that serve an unknown purpose. 
/// This is also the only object with non-integer properties (kA1, kA2, ...)
/// The reverse gameplay option is always on for some unknown reason. USE AT YOUR OWN RISK!!
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
/// * `stop_mode`: Stop mode (see `StopMode` struct)
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
/// * `transition`: Type of transition. See `TransitionType` struct
/// * `mode`: Mode for transition (enter/exit only). See `TransitionMode` struct
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

// items and counters

/// Returns a counter object
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `item_id`: ID of the counter
/// * `timer`: Is a timer?
/// * `align`: Visual alignment of counter object. See `ItemAlign` struct.
/// * `seconds_only`: Show only seconds if timer?
/// * `special_mode`: Other special mode of timer. See `CounterMode` struct.
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

/// Returns an item edit trigger
/// # Arguments
/// * `config`: General object options, such as position and scale
/// * `operand1`: Optional first operand, tuple of (ID, item type)
/// * `operand2`: Optional second operand, tuple of (ID, item type)
/// * `target_id`: Target item id
/// * `target_type`: Target item type
/// * `modifier`: f32 modifier; default is 1.0
/// * `assign_op`: operator for assigning to result; see `Op` enum.
/// * `mod_op`: operator for applying mod to result; see `Op` enum.
/// * `id_op`: operator between operands 1 and 2; see `Op` enum.
/// * `id_rounding`: operand rounding function; see `RoundMode` enum.
/// * `result_rounding`: final rounding function; see `RoundMode` enum.
/// * `id_sign`: operand signing function; see `SignMode` enum.
/// * `result_sign`: final signing function; see `SignMode` enum.
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
 * bg speed config
 * mg speed config
 * bg effect on
 * bg effect off
 * 
 * Item triggers
 * touch trigger
 * count trigger
 * instant count trigger
 * pickup trigger
 * item compare
 * persistent item
 * 
 * Spawner triggers
 * random trigger
 * advanced random
 * sequence
 * event trigger
 * spawn particle
 * on death
 * 
 * Camera
 * zoom camera
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
 * timewarp
 * time trigger
 * time event trigger
 * time control trigger
 * 
 * Misc.
 * ui config
 * end trigger
 * bpm marker
 * gradient
 * 
 * Collision blocks
 * collision trigger
 * instant collision
 * state block
 * collision block
 * toggle block
 * 
 * Player triggers
 * disable player trail
 * enable player trail
 * player control
 * options
 * gravity trigger
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