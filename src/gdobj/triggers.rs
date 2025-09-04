use serde_json::json;

use crate::{gdobj::{GDObjConfig, GDObject}, utils::properties_from_vec};

// eventually there will be constructors for every trigger here
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

    GDObject::new(901, config, properties_from_vec(properties))
}