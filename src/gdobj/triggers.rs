//! This file contains constructors for trigger objects.
//! # ⚠️ Warning
//! **This file is incomplete. More triggers will be added in the future.**

use serde_json::json;
use crate::{gdobj::{GDObjConfig, GDObject}, utils::properties_from_json};


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

    GDObject::new(901, config, properties_from_json(properties))
}