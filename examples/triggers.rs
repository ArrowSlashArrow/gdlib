//! Builds a level driven by triggers: a moving platform, colour changes and a looping spawn chain.
//!
//! Showcased items:
//! - application of groups to objects so triggers can target them
//! - create triggers from typed structs (`MoveTrigger`, `ColourTrigger`, `SpawnTrigger`, ...)
//! - make triggers spawn-triggered (called by other triggers)
//! - use spawn remaps to reuse one set of triggers on several groups
//!
//! Run with `cargo run --example triggers`.

use gdlib::{
    cclocallevels::{
        gdlevel::GDLevel,
        gdobj::{
            AlphaTrigger, ColourTrigger, GDObject, MoveTrigger, SpawnTrigger,
            ids::objects,
            meta::GDObjConfig,
            structs::{Colour, ColourChannel, DefaultMove, Easing, MoveEasing, MoveMode},
        },
    },
    core::GDError,
};

// Group IDs used in this level.
const PLATFORM_A: i16 = 1;
const PLATFORM_B: i16 = 2;
/// Placeholder group used by the bobbing triggers. Spawn remaps replace it with a real platform group.
const BOB_TARGET: i16 = 10;
/// Group holding the triggers that make one platform bob up and down.
const BOB_TRIGGERS: i16 = 11;

fn main() -> Result<(), GDError> {
    let mut level = GDLevel::new_latest_version();
    level.identity.name = "gdlib triggers example".into();

    let mut objects = Vec::new();

    // Two floating platforms, each in its own group.
    for (group, start_x) in [(PLATFORM_A, 300.0), (PLATFORM_B, 600.0)] {
        for i in 0..4 {
            objects.push(GDObject::new(
                objects::BLACK_GRADIENT_SQUARE,
                &GDObjConfig::new()
                    .with_pos(start_x + i as f32 * 30.0, 105.0)
                    .with_groups([group]),
                vec![],
            ));
        }
    }

    // -- Spawn-triggered "bob" routine --
    // These triggers live off-screen and are only run by spawn triggers. They target the
    // placeholder group BOB_TARGET, which each spawn trigger remaps to a real platform.
    let spawned = GDObjConfig::new()
        .with_pos(-200.0, 300.0)
        .with_groups([BOB_TRIGGERS])
        .with_spawnable(true)
        .with_multitrigger(true);

    let bob = |dy: f32| MoveTrigger {
        move_config: MoveMode::Default(DefaultMove {
            dx: 0.0,
            dy,
            x_lock: None,
            y_lock: None,
        }),
        time: 0.5,
        target_group: BOB_TARGET,
        silent: false,
        dynamic: false,
        easing: Easing::from(MoveEasing::EaseInOut, 2.0),
    };
    objects.push(GDObject::from_config(&spawned, bob(60.0)));
    // A spawn trigger with a delay moves the platform back down once the first move finishes.
    objects.push(GDObject::from_config(
        &spawned.clone().translate(0.0, 30.0),
        SpawnTrigger {
            spawn_id: BOB_TRIGGERS + 1,
            delay: 0.5,
            ..Default::default()
        },
    ));
    let down = GDObjConfig::new()
        .with_pos(-170.0, 300.0)
        .with_groups([BOB_TRIGGERS + 1])
        .with_spawnable(true)
        .with_multitrigger(true);
    objects.push(GDObject::from_config(&down, bob(-60.0)));

    // Each of these fires when the player passes its x position.
    let at = |x: f32| GDObjConfig::new().with_pos(x, 300.0);

    // Run the bob routine on each platform by remapping BOB_TARGET to the platform's group.
    for (x, platform) in [(150.0, PLATFORM_A), (450.0, PLATFORM_B)] {
        objects.push(GDObject::from_config(
            &at(x),
            SpawnTrigger {
                spawn_id: BOB_TRIGGERS,
                spawn_remaps: vec![(BOB_TARGET as i32, platform as i32)],
                reset_remap: true,
                ..Default::default()
            },
        ));
    }

    // Fade the background to purple over two seconds.
    objects.push(GDObject::from_config(
        &at(200.0),
        ColourTrigger {
            colour: Colour::from_rgb((106, 44, 145)),
            channel: ColourChannel::Background,
            opacity: 1.0,
            blending: false,
            use_player_col_1: false,
            use_player_col_2: false,
            fade_time: 2.0,
            copy_colour: None,
        },
    ));

    // Make the second platform fade out once the player has passed it.
    objects.push(GDObject::from_config(
        &at(800.0),
        AlphaTrigger {
            target_group: PLATFORM_B,
            opacity: 0.0,
            fade_time: 1.0,
        },
    ));

    level.add_objects(objects);
    level.export_to_gmd("test_gmds/generated_example_triggers.gmd")?;
    println!("Wrote test_gmds/generated_example_triggers.gmd");
    Ok(())
}
