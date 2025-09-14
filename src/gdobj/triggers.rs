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
/// (original channel, hue shift, saturation multiplier, brightness multiplier, use legacy hsv?, copy opacity?) 
pub fn colour_trigger(
    config: GDObjConfig,
    colour: (u8, u8, u8),
    fade_time: f32,
    opacity: f32,
    blending: bool,
    use_player_col_1: bool,
    use_player_col_2: bool,
    copy_colour: Option<(i32, i32, f32, f32, bool, bool)>
) -> GDObject {
    let mut properties = json!({
        "7": colour.0,
        "8": colour.1,
        "9": colour.2,
        "10": fade_time,
        "15": use_player_col_1 as i32,
        "16": use_player_col_2 as i32,
        "35": opacity
    });

    let map = properties.as_object_mut().unwrap();

    if blending {
        map.insert("17".to_string(), Value::from(""));
    }

    if let Some((channel, hue, saturation, lightness, legacy_hsv, copy_opacity)) = copy_colour {
        let mut cfg_string = format!("{hue}a{saturation}a{lightness}a0a");
        if !legacy_hsv {
            cfg_string += "0";
            map.insert("210".to_string(), Value::from(""));
        }
        if copy_opacity {
            map.insert("60".to_string(), Value::from("1"));
        }
        map.insert("49".to_string(), Value::from(cfg_string));
        map.insert("50".to_string(), Value::from(channel));
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

/* TODO: trigger constructors
 * move trigger (all options)
 * pulse trigger
 * spawn trigger 
 * rotate trigger
 * scale trigger
 * follow trigger
 * shake trigger
 * animate trigger
 * frame trigger
 * follor player y
 * advanced follow
 * edit advanced follor
 * re target advanced follow
 * keyframe setup trigger
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
 * switch bg
 * sdwitch ground
 * switch mg
 * touch trigger
 * count trigger
 * instant count trigger
 * pickup trigger
 * time trigger
 * time event trigger
 * time control trigger
 * item edit
 * item compare
 * persistnet item
 * random trigger
 * advanced random
 * sequence
 * spawn particle
 * reset
 * zoom camera
 * static camera
 * offset camera
 * gameplay offset camera
 * rotate camera
 * edge camera
 * camera mode
 * reverse gameplay
 * rotate gameplay
 * song trigger
 * edit song trigger
 * sfx trigger
 * edit sfx trigger
 * event trigger
 * timewarp
 * middleground config
 * bg speed config
 * mg speed config
 * counter
 * ui config
 * link visible
 * collision trigger
 * instant collision
 * state block
 * collision block
 * toggle block
 * on death
 * disable player trail
 * enable player trail
 * show player
 * hide player
 * bg effect on
 * bg effect off
 * end trigger
 * player control
 * options
 * bpm marker
 * gradient
 * gravity trigger
 * teleport trigger
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
 * no block transitions
 * object from top transition
 * object from bottom transition
 * object from left transition
 * object from right transition
 * object scale in transition
 * object scale out transition
 * object random direction transition
 * object away left transition
 * object away right transition
 * object from middle transition
 * object to middle transition
 * no object transition
 */