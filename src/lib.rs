#![doc = include_str!("../README.md")]
// #![recursion_limit = "256"]

pub mod core;
pub mod deserialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod serialiser;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        deserialiser::decode_levels_to_string,
        gdlevel::{Level, Levels},
        gdobj::{
            Event, GDObjConfig, MoveEasing,
            misc::default_block,
            triggers::{self, DefaultMove, advanced_random_trigger, move_trigger},
        },
    };

    fn benchmark<F: Fn() -> R, R>(name: &str, f: F) -> R {
        let start = Instant::now();
        let result = f();
        println!(
            "{name}: {:.3}ms",
            start.elapsed().as_micros() as f64 / 1000.0
        );
        return result;
    }

    #[test]
    fn decode_savefile() {
        benchmark("savefile parse", || {
            assert!(decode_levels_to_string().is_ok())
        })
    }

    #[test]
    fn parse_to_levels_obj() {
        let raw_levels_savefile = decode_levels_to_string().unwrap();
        Levels::from_decrypted(raw_levels_savefile).unwrap();
    }

    #[test]
    fn read_objs() {
        let mut levels = Levels::from_local().unwrap();
        let first_level = levels.levels.first_mut().unwrap();
        let data = first_level.get_decrypted_data().unwrap();

        for (idx, obj) in data.objects.iter().enumerate() {
            println!("{idx}: {obj:?}");
        }
    }

    #[test]
    fn move_constructor() {
        let mut level = Level::new("move trigger t3st", "gdlib", None, None);
        level.add_object(move_trigger(
            &GDObjConfig::default().pos(45.0, 45.0).dont_fade(true),
            triggers::MoveMode::Default(DefaultMove {
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

        let mut levels = Levels::from_local().unwrap();
        levels.add_level(level);
        levels.export_to_savefile_with_backup().unwrap();
    }

    #[test]
    fn level_display_test() {
        let mut levels = Levels::from_local().unwrap();
        let level = levels.levels.get_mut(0).unwrap();
        println!("Level info: {level}");
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
            .center_effect(true)
            .editor_layer_1(4)
            .dont_fade(true)
            .groups([2, 3, 1738])
            .extra_sticky(true)
            .no_glow(true)
            .set_z_layer(crate::gdobj::ZLayer::B3)
            .set_base_colour(crate::gdobj::ColourChannel::Background);

        let block = default_block(&config);
        let mut level = Level::new("porpeties", "gdlib", None, None);
        level.add_object(block);

        level.export_to_gmd("GMDS/properties.gmd").unwrap();
    }

    #[test]
    fn adv_random() {
        let mut level = Level::new("adv random", "gdlib", None, None);
        level.add_object(advanced_random_trigger(
            &GDObjConfig::default().pos(45.0, 45.0),
            vec![(50, 10), (60, 20), (70, 5), (80, 25), (90, 2)],
        ));
        let _ = level.export_to_gmd("GMDS/adv_random.gmd");
    }

    #[test]
    fn big_level_parse() {
        let level = Level::from_gmd("GMD_tests/big.gmd").unwrap();
        benchmark("Big level parse", || level.get_decrypted_data());
    }

    #[test]
    fn ref_vs_copy_benchmark() {
        let count = 50;
        let mut ref_time: u128 = 0;
        let mut copy_time: u128 = 0;

        let level = Level::from_gmd("GMD_tests/All Object IDs.gmd").unwrap();
        for _ in 0..count {
            {
                let start = Instant::now();
                let _ = level.get_decrypted_data();
                copy_time += start.elapsed().as_nanos();
            }
            {
                let mut level = Level::from_gmd("GMD_tests/All Object IDs.gmd").unwrap();
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
    fn level_info() {
        let levels = Levels::from_local().unwrap();
        let level = &levels.levels[0];
        println!("{level:#?}")
    }

    #[test]
    fn trigger() {
        let level = Level::from_gmd("GMDS/Unnamed 25.gmd").unwrap();
        let data = level.get_decrypted_data().unwrap();
        for obj in data.objects {
            println!("{obj:?}");
            println!("{}", obj.to_string())
        }
    }

    #[test]
    fn serialise_level_benchmark() {
        let mut level = Level::from_gmd("GMD_tests/big.gmd").unwrap();
        level.decrypt_level_data();
        let _ = benchmark("big.gmd serialise", || {
            level.export_to_gmd("GMD_tests/big2.gmd")
        });
    }

    #[test]
    fn local_levels_parse_benchmark() {
        benchmark("local levels parse", || Levels::from_local().unwrap());
    }

    #[test]
    fn _temp_trigger_from_savefile() {
        let levels = Levels::from_local().unwrap();
        let level = &levels.levels[0];
        let data = level.get_decrypted_data().unwrap();
        for obj in data.objects {
            println!("{obj:?}");
            println!("{}", obj.to_string())
        }
    }

    #[test]
    fn event_trigger_test() {
        let mut level = Level::new("event trigger test", "gdlib", None, None);
        let cfg = GDObjConfig::new().pos(45.0, 45.0);
        level.add_object(crate::gdobj::triggers::event_trigger(
            &cfg,
            123,
            vec![Event::BallSwitch, Event::FallSpeedLow],
            0,
            crate::gdobj::ExtraID2::All,
        ));
    }
}
