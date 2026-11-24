//! Edits every object in an existing level at once.
//!
//! Run with `cargo run --example bulk_edit`.

use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            ids::{objects, properties},
            meta::GDObjAttributes,
            structs::{ColourChannel, GDValue},
        },
    },
    core::GDError,
};

fn main() -> Result<(), GDError> {
    let mut level = GDLevel::from_gmd("test_gmds/Chompstep.gmd")?;

    // Level data stays compressed until you ask for it. This decompresses it in place.
    let Some(data) = level.get_decrypted_data_ref() else {
        println!("level has no data");
        return Ok(());
    };
    let before = data.objects.len();

    // 1. Remove every start position, e.g. before sharing a level.
    data.objects.retain(|obj| obj.id != objects::START_POSITION);

    // 2. Shift the whole level 10 blocks to the right to make room for an intro.
    for obj in data.objects.iter_mut() {
        obj.config.pos.0 += 300.0;
    }

    // 3. Put every object that uses the default object colour onto channel 50,
    //    so the whole level can be recoloured with a single colour trigger.
    for obj in data.objects.iter_mut() {
        if obj.config.colour_channels.0 == ColourChannel::Object {
            obj.config.set_base_colour(ColourChannel::Channel(50));
        }
    }

    // 4. Stop objects from fading in and out at the screen edges.
    for obj in data.objects.iter_mut() {
        obj.config.set_attributes(
            GDObjAttributes::dont_fade | GDObjAttributes::dont_enter,
            true,
        );
    }

    // 5. Make every move trigger twice as fast by halving its duration.
    //    Object-specific properties are read and written by property ID.
    let mut retimed = 0;
    for obj in data
        .objects
        .iter_mut()
        .filter(|o| o.id == objects::MOVE_TRIGGER)
    {
        if let Some(GDValue::Float(time)) =
            obj.get_property(properties::DURATION_GROUP_TRIGGER_CHANCE)
        {
            obj.set_property(
                properties::DURATION_GROUP_TRIGGER_CHANCE,
                GDValue::Float(time / 2.0),
            );
            retimed += 1;
        }
    }

    let after = data.objects.len();
    println!("removed {} start positions", before - after);
    println!("retimed {retimed} move triggers");

    level.identity.name = format!("{} (edited)", level.identity.name);
    level.export_to_gmd("test_gmds/generated_example_bulk_edit.gmd")?;
    println!("Wrote test_gmds/generated_example_bulk_edit.gmd");
    Ok(())
}
