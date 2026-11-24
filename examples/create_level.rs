//! Builds a short playable level from scratch and saves it as a .gmd file.
//!
//! Run with `cargo run --example create_level`.

use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            GDObject, GDPad, GDPadType, GameplayPortal, GameplayPortalConfig, PortalOptions,
            TextObject, ids::objects, meta::GDObjConfig,
        },
    },
    core::GDError,
};

/// One grid square in GD is 30 units wide. Objects are positioned by their centre.
const GRID: f32 = 30.0;

/// Returns the centre of grid cell (`col`, `row`), where row 0 sits on the ground.
fn cell(col: i32, row: i32) -> (f32, f32) {
    (
        col as f32 * GRID + GRID / 2.0,
        row as f32 * GRID + GRID / 2.0,
    )
}

fn main() -> Result<(), GDError> {
    // Always use `new_latest_version` for new levels: GD needs a valid binary version to open them.
    let mut level = GDLevel::new_latest_version();
    level.identity.name = "gdlib example".into();
    level.identity.description = Some("Generated with gdlib".into());
    level.identity.creator = "gdlib".into();

    let mut objects = Vec::new();

    // A title label near the start.
    let (x, y) = cell(2, 4);
    objects.push(GDObject::from_config(
        &GDObjConfig::new().with_pos(x, y),
        TextObject {
            text: "Made with gdlib!".into(),
            kerning: 0,
        },
    ));

    // A row of spikes every 8 cells, with a block step before each one.
    for col in (10..60).step_by(8) {
        let (x, y) = cell(col, 0);
        let mut step = GDObject::default_from_id(objects::BLACK_GRADIENT_SQUARE); // default block (id 1)
        step.config.set_pos(x, y);
        objects.push(step);

        let (x, y) = cell(col + 3, 0);
        let mut spike = GDObject::default_from_id(objects::BLACK_GRADIENT_SPIKE);
        spike.config.set_pos(x, y);
        objects.push(spike);
    }

    // A yellow jump pad to clear a tall wall of blocks.
    let (x, y) = cell(64, 0);
    objects.push(GDObject::from_config(
        &GDObjConfig::new().with_pos(x, y - 12.0), // pads sit low in their cell
        GDPad::new(GDPadType::Yellow),
    ));
    for row in 0..3 {
        let (x, y) = cell(68, row);
        objects.push(GDObject::default_from_id(objects::BLACK_GRADIENT_SQUARE).with_pos(x, y));
    }

    // A ship portal, then a ceiling and floor to fly between.
    let (x, y) = cell(76, 2);
    objects.push(GDObject::from_config(
        &GDObjConfig::new().with_pos(x, y),
        GameplayPortalConfig {
            portal: GameplayPortal::ShipGamemode,
            multi_activate: false,
            no_plat_multi_activate: false,
            extra_config: PortalOptions::None,
        },
    ));
    for col in 78..110 {
        for row in [0, 6] {
            let (x, y) = cell(col, row);
            objects.push(GDObject::default_from_id(objects::BLACK_GRADIENT_SQUARE).with_pos(x, y));
        }
    }

    let count = objects.len();
    level.add_objects(objects);
    level.content.object_count = count as i32;

    level.export_to_gmd("test_gmds/generated_example_create_level.gmd")?;
    println!("Wrote {count} objects to test_gmds/generated_example_create_level.gmd");
    Ok(())
}

/// Small helper so plain objects can be positioned in one expression.
trait WithPos {
    fn with_pos(self, x: f32, y: f32) -> Self;
}

impl WithPos for GDObject {
    fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.config.set_pos(x, y);
        self
    }
}
