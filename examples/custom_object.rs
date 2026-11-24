//! Creates objects that gdlib has no ready-made struct for.
//!
//! There are three ways to do this, from least to most effort:
//! 1. `GDObject::default_from_id` then `set_property`, for one-off tweaks
//! 2. `GDObject::new` with an explicit list of properties
//! 3. Your own struct that implements `ObjectProperties`, for objects you create often.
//!    It then works with `GDObject::from_config` just like the built-in structs.
//!
//! Run with `cargo run --example custom_object`.

use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            GDObject, ObjectProperties,
            ids::{objects, properties},
            meta::GDObjConfig,
            structs::GDValue,
        },
        properties::get_obj_property_type,
    },
    core::GDError,
};

/// A shake trigger that only stores what this project cares about.
/// gdlib already has `ShakeTrigger`; this one exists to show how the trait works.
#[derive(Debug, Clone, PartialEq)]
struct QuickShake {
    strength: f32,
    seconds: f32,
}

impl ObjectProperties for QuickShake {
    fn object_id(&self) -> i32 {
        objects::SHAKE_TRIGGER
    }

    fn serialise(&self) -> Vec<(u16, GDValue)> {
        vec![
            (properties::SHAKE_STRENGTH, GDValue::Float(self.strength)),
            (
                properties::DURATION_GROUP_TRIGGER_CHANCE,
                GDValue::Float(self.seconds),
            ),
        ]
    }

    // Optional: lets you read these objects back out of a level.
    fn from_object(obj: &GDObject) -> Option<Self> {
        if obj.id != objects::SHAKE_TRIGGER {
            return None;
        }
        let float = |p| match obj.get_property(p) {
            Some(GDValue::Float(f)) => f,
            _ => 0.0,
        };
        Some(Self {
            strength: float(properties::SHAKE_STRENGTH),
            seconds: float(properties::DURATION_GROUP_TRIGGER_CHANCE),
        })
    }
}

fn main() -> Result<(), GDError> {
    // Not sure which GDValue a property expects? Look it up.
    println!(
        "SHAKE_STRENGTH expects {:?}",
        get_obj_property_type(properties::SHAKE_STRENGTH)
    );

    // 1. Start from the in-game defaults and change one property.
    let mut timewarp = GDObject::default_from_id(objects::TIMEWARP_TRIGGER);
    timewarp.config.set_pos(150.0, 300.0);
    timewarp.set_property(properties::TIMEWARP_AMOUNT, GDValue::Float(0.5));

    // 2. Spell out every property yourself.
    let toggle_off = GDObject::new(
        objects::TOGGLE_TRIGGER,
        &GDObjConfig::new().with_pos(210.0, 300.0),
        vec![(properties::TARGET_ITEM, GDValue::Group(7))],
    );

    // 3. Use your own struct.
    let shake = GDObject::from_config(
        &GDObjConfig::new().with_pos(270.0, 300.0),
        QuickShake {
            strength: 2.0,
            seconds: 0.4,
        },
    );

    // Objects serialise to GD's raw format, which is useful for debugging.
    for obj in [&timewarp, &toggle_off, &shake] {
        println!("{:<20} {}", obj.get_name(), obj.serialise_to_string());
    }

    // Round trip through a level to show `from_object` reading it back.
    let mut level = GDLevel::new_latest_version();
    level.add_objects([timewarp, toggle_off, shake]);
    let read_back: Vec<QuickShake> = level
        .get_objects_ref()
        .into_iter()
        .flatten()
        .filter_map(QuickShake::from_object)
        .collect();
    println!("read back: {read_back:?}");

    level.export_to_gmd("test_gmds/generated_example_custom_object.gmd")?;
    Ok(())
}
