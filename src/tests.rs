//! Unit tests for the crate
use std::{fs, time::Instant};

use plist::Value::{Array, Dictionary, String};

use crate::{
    ccgamemanager::CCGameManager,
    cclocallevels::{
        gdlevel::{CCLocalLevels, GDLevel, leveldata::HeaderValue},
        gdobj::{
            self, GDObject,
            constructors::{
                misc::default_block,
                triggers::{AdvancedRandomTrigger, EventTrigger, move_trigger},
            },
            ids::{
                objects::{TRIGGER_ADVANCED_RANDOM, TRIGGER_EVENT},
                properties::RANDOM_PROBABILITIES_LIST,
            },
            meta::{GDObjAttributes, GDObjConfig},
            structs::{
                ColourChannel, DefaultMove, Event, ExtraID2, Group, MoveEasing, MoveMode, ZLayer,
            },
        },
    },
    core::rand::{check_seed_advanced_random, next_seed_mut},
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
    level.add_object(GDObject::from_config(
        TRIGGER_ADVANCED_RANDOM,
        GDObjConfig::default().pos(45.0, 45.0),
        AdvancedRandomTrigger {
            probabilities: vec![(50, 10), (60, 20), (70, 5), (80, 25), (90, 2)],
        },
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
    level.add_object(GDObject::from_config(
        TRIGGER_EVENT,
        cfg,
        EventTrigger {
            target_group: 123,
            events: vec![Event::BallSwitch, Event::FallSpeedLow],
            extra_id: 0,
            extra_id2: ExtraID2::All,
        },
    ));
}

#[test]
fn advanced_random_predict() {
    // tuple: (object, seed, expected group)
    let tests: &[(&str, u64, i16)] = &[
        (
            "1,2068,2,75,3,75,155,2,11,1,87,1,36,1,152,2.50.3.50;",
            5006,
            2,
        ),
        ("1,2068,2,75,3,75,155,2,11,1,87,1,36,1,152,2.50.3.50;", 1, 2),
        (
            "1,2068,2,75,3,75,155,2,11,1,87,1,36,1,152,1.10.2.10.3.10.4.10.5.10;",
            32321,
            2,
        ),
        (
            "1,2068,2,75,3,75,155,1,11,1,87,1,36,1,152,1.10.2.20.3.15.4.5.5.25.6.50;",
            58392,
            6,
        ),
        ("1,2068,2,285,3,285,155,1,36,1,152,2.10.3.10;", 190824, 2),
    ];

    for &(obj_str, seed, expected) in tests {
        let adv_rand = GDObject::parse_str(obj_str);
        // get probabilities table
        let probabilities = adv_rand.get_property(RANDOM_PROBABILITIES_LIST).unwrap();
        assert_eq!(
            check_seed_advanced_random(seed, &probabilities).unwrap(),
            Group::Regular(expected)
        );
    }
}

#[test]
#[ignore]
fn print_list_info() {
    let cc = CCLocalLevels::from_local().unwrap();
    println!("{:#?}", cc.lists);
}

#[test]
#[ignore]
fn cc_game_manager_parse() {
    let gm = CCGameManager::from_local().unwrap();
    fs::write("ccgamemanager dump 2", format!("{gm:#?}")).unwrap();
    for (k, v) in gm.other_properties {
        println!(
            "{k}: {:?}",
            match v {
                Dictionary(d) => String(format!("{{ length: {} }}", d.len())),
                Array(a) => String(format!("[ length: {} ]", a.len())),
                v => v,
            }
        );
    }
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

#[test]
fn get_seed_from_criteria() -> anyhow::Result<()> {
    let level = GDLevel::from_gmd("test_gmds/Chompstep.gmd")?;
    let mut objects = level.get_decrypted_data().unwrap().objects;

    // advanced random objects do not change the group for some reason
    objects.retain(|o| o.id == TRIGGER_ADVANCED_RANDOM && o.config.pos.0 > 0.0);
    objects.sort_by(|a, b| a.config.pos.0.total_cmp(&b.config.pos.0));

    let expected = vec![3, 2, 2, 2, 2, 2, 2, 2];

    let mut seed = 0;

    'outer: for obj in objects {
        // get probabilities table
        let probabilities = obj.get_property(RANDOM_PROBABILITIES_LIST).unwrap();

        let mut seed_clone = seed;
        for g in &expected {
            let got = check_seed_advanced_random(seed, &probabilities).unwrap();
            next_seed_mut(&mut seed_clone);
            if let Group::Regular(g1) = got
                && g1 != *g
            {
                seed += 1;
                continue 'outer;
            }
        }
        println!("found seed: {}", seed_clone);
    }

    Ok(())
}
