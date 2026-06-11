//! Unit tests for the crate
use std::{fs, time::Instant};

use crate::{
    ccgamemanager::CCGameManager,
    cclocallevels::{
        gdlevel::{CCLocalLevels, GDLevel, leveldata::HeaderValue},
        gdobj::{
            self,
            constructors::{
                misc::default_block,
                triggers::{advanced_random_trigger, event_trigger, move_trigger},
            },
            ids::{objects::TRIGGER_ADVANCED_RANDOM, properties::RANDOM_PROBABILITIES_LIST},
            meta::{GDObjAttributes, GDObjConfig},
            structs::{
                ColourChannel, DefaultMove, Event, ExtraID2, Group, MoveEasing, MoveMode, ZLayer,
            },
        },
    },
    core::rand::check_seed_advanced_random,
};

fn benchmark<F: Fn() -> R, R>(name: &str, f: F) -> R {
    let start = Instant::now();
    let result = f();
    println!(
        "{name}: {:.3}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
    result
}

#[test]
fn read_objs() {
    let level = GDLevel::from_gmd("test_gmds/All Object IDs.gmd").unwrap();
    let data = level.get_decrypted_data().unwrap();

    for (idx, obj) in data.objects.iter().enumerate() {
        println!("{idx}: {obj:?}");
    }
}

#[test]
fn move_constructor() {
    let mut level = GDLevel::default();
    level.identity.name = "move trigger t3st".into();
    level.identity.creator = "gdlib".into();
    level.add_object(move_trigger(
        &GDObjConfig::default().pos(45.0, 45.0),
        MoveMode::Default(DefaultMove {
            dx: 45.0,
            dy: 54.0,
            x_lock: None,
            y_lock: None,
        }),
        17.38,
        679,
        false,
        true,
        Some((MoveEasing::ElasticInOut, 1.50)),
    ));

    level
        .export_to_gmd("test_gmds/generated_move_trigger_test.gmd")
        .unwrap();
}

#[test]
fn level_display_test() {
    let level = GDLevel::from_gmd("test_gmds/big.gmd").unwrap();
    println!("GDLevel info: {level}");
    println!(
        "Unused groups: {:?}",
        level.get_decrypted_data().unwrap().get_unused_groups()
    );
    println!(
        "Used groups: {:?}",
        level.get_decrypted_data().unwrap().get_used_groups()
    );
}

#[test]
fn obj_properties() {
    let config = GDObjConfig::new()
        .editor_layer_1(4)
        .set_attribute_flag(GDObjAttributes::dont_fade, true)
        .groups([2, 3, 1738])
        .set_attribute_flag(GDObjAttributes::extra_sticky, true)
        .set_attribute_flag(GDObjAttributes::no_glow, true)
        .set_z_layer(ZLayer::B3)
        .set_base_colour(ColourChannel::Background);

    let block = default_block(&config);
    let mut level = GDLevel::default();
    level.add_object(block);

    level
        .export_to_gmd("test_gmds/generated_properties.gmd")
        .unwrap();
}

#[test]
fn adv_random() {
    let mut level = GDLevel::default();
    level.add_object(advanced_random_trigger(
        &GDObjConfig::default().pos(45.0, 45.0),
        vec![(50, 10), (60, 20), (70, 5), (80, 25), (90, 2)],
    ));
    let _ = level.export_to_gmd("test_gmds/generated_adv_random.gmd");
}

#[test]
fn big_level_parse() {
    let level = GDLevel::from_gmd("test_gmds/big.gmd").unwrap();
    benchmark("Big level parse", || level.get_decrypted_data());
}

#[test]
fn ref_vs_copy_benchmark() {
    let count = 50;
    let mut ref_time: u128 = 0;
    let mut copy_time: u128 = 0;

    let level = GDLevel::from_gmd("test_gmds/All Object IDs.gmd").unwrap();
    for _ in 0..count {
        {
            let start = Instant::now();
            let _ = level.get_decrypted_data();
            copy_time += start.elapsed().as_nanos();
        }
        {
            let mut level = GDLevel::from_gmd("test_gmds/All Object IDs.gmd").unwrap();
            let start = Instant::now();
            let _ = level.get_decrypted_data_ref();
            ref_time += start.elapsed().as_nanos();
        }
    }
    let objs = level.get_decrypted_data().unwrap().objects.len();
    let avg_copy_time = copy_time as f64 / (1_000 * count) as f64;
    let avg_ref_time = ref_time as f64 / (1_000 * count) as f64;

    println!(
        "Objects: {objs}; {count} tests\nAverage copy time: {:.3}us\nAverage ref time: {:.3}us\nAverage copy time per object: {:.3}us\nAverage ref time per object: {:.3}us",
        avg_copy_time,
        avg_ref_time,
        avg_copy_time / objs as f64,
        avg_ref_time / objs as f64,
    );
}

#[test]
fn serialise_level_benchmark() {
    let mut level = GDLevel::from_gmd("test_gmds/big.gmd").unwrap();
    level.decrypt_level_data().unwrap();
    let _ = benchmark("big.gmd serialise", || {
        level.export_to_gmd("test_gmds/generated_big2.gmd")
    });
}

#[test]
fn event_trigger_test() {
    let mut level = GDLevel::default();
    let cfg = GDObjConfig::new().pos(45.0, 45.0);
    level.add_object(event_trigger(
        &cfg,
        123,
        vec![Event::BallSwitch, Event::FallSpeedLow],
        0,
        ExtraID2::All,
    ));
}

#[test]
fn advanced_random_predict() {
    let level = GDLevel::from_gmd("test_gmds/advrand test.gmd").unwrap();
    // find adv random trigger
    let data = level.get_decrypted_data().unwrap();
    let adv_rand = data
        .objects
        .iter()
        .find(|o| o.id == TRIGGER_ADVANCED_RANDOM)
        .unwrap();

    // get probabilities table
    let probabilities = adv_rand.get_property(RANDOM_PROBABILITIES_LIST).unwrap();

    // set input params
    let seed = 123;
    // predict outcome
    assert_eq!(
        check_seed_advanced_random(seed, &probabilities).unwrap(),
        Group::Regular(1)
    );
}

#[test]
fn print_list_info() {
    let mut cc = CCLocalLevels::from_local().unwrap();
    println!("{:#?}", cc.lists);
}

#[test]
fn cc_game_manager_parse() {
    let gm = CCGameManager::from_local().unwrap();
    fs::write("ccgamemanager dump 2", format!("{gm:#?}")).unwrap();
}

#[test]
#[ignore]
fn _temp_read_objs() {
    let level = GDLevel::from_gmd("test_gmds/empty test level.gmd").unwrap();
    let data = level.get_decrypted_data().unwrap();

    for (idx, obj) in data.objects.iter().enumerate() {
        println!("{idx}: {obj:?}");
    }
}

#[test]
#[ignore]
fn _temp_level_header() -> anyhow::Result<()> {
    let level = GDLevel::from_gmd("test_gmds/level.gmd")?;
    let data = level.get_decrypted_data().unwrap();
    let colour_string = data
        .headers
        .get_property(gdobj::ids::level_header::COLOURS)
        .unwrap();

    if let HeaderValue::ColourString(cs) = colour_string {
        for c in cs {
            println!("{:?}", c);
        }
    }

    Ok(())
}
