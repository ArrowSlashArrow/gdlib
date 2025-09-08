//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use serde_json::json;
use crate::{gdobj::{GDObjConfig, GDObjProperties, GDObject}};


/// Returns a move trigger object
/// 
/// # Arguments
/// * `config`: General object options, such as positionn and scale
/// * `dX`: How much to move the target group in the X direction. 
/// * `dY`: How much to move the target group in the Y direction. 
/// * `time`: Move time for target object.
/// * `targetGroup`: Group that the move trigger is moving.
/// * `targetMode`: Enabled if the group is moving to the location of another group.
/// * `aim`: The other group that the object would move
/// 
/// Returns a GDObject object with the corresponding properties.
/// 
pub fn move_trigger(
    config: GDObjConfig,
    dX: i32,
    dY: i32,
    time: f32,
    targetGroup: i32,
    targetMode: bool,
    aim: i32
) -> GDObject { 
    let properties = if targetMode {
        json!({
            "28": 0,
            "29": 0,
            "30": 0,
            "85": 2,
            "71": aim,
            "100": 1,
            "51": targetGroup,
            "10": time
        })
    } else {
        json!({
            "28": dX,
            "29": dY,
            "51": targetGroup,
            "10": time
        })
    };

    GDObject::new(901, config, GDObjProperties::from_json(properties))
}

/* TODO: trigger constructors
 * start pos
 * colour trigger
 * move trigger (all options)
 * stop trigger
 * pulse trigger
 * alpha trigger
 * toggle trigger
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