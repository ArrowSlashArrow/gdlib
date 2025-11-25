#![doc = include_str!("../README.md")]
#![recursion_limit = "256"]

pub mod deserialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod serialiser;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        deserialiser::decode_levels_to_string,
        gdlevel::{Level, Levels},
        gdobj::{
            GDObjConfig, MoveEasing,
            misc::default_block,
            triggers::{self, DefaultMove, move_trigger, start_pos},
        },
    };

    #[test]
    fn decode_savefile() {
        assert!(decode_levels_to_string().is_ok())
    }

    #[test]
    fn parse_to_levels_obj() {
        let raw_levels_savefile = decode_levels_to_string().unwrap();
        Levels::from_decrypted(raw_levels_savefile).unwrap();
    }

    #[test]
    fn add_level_and_export() {
        let mut levels = Levels::from_local().unwrap();
        let mut new_level = Level::new(
            "rust websocket tutorial 2",
            "arrowslasharrow",
            Some("dont use rust"),
            Some(857925),
        );

        new_level.add_object(triggers::move_trigger(
            GDObjConfig::default().pos(45.0, 45.0).groups([1234]),
            triggers::MoveMode::Default(triggers::DefaultMove {
                dx: 45.0,
                dy: 54.0,
                x_lock: None,
                y_lock: None,
            }),
            0.50,
            1,
            false,
            true,
            Some((MoveEasing::ElasticInOut, 1.50)),
        ));

        new_level.add_object(default_block(GDObjConfig::default().x(15.0).y(15.0)));

        levels.add_level(new_level);
        levels.export_to_savefile_with_backup().unwrap();
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
        let mut level = Level::new("move trigger t3st", "arrowslasharrow", None, None);
        level.add_object(move_trigger(
            GDObjConfig::default().pos(45.0, 45.0).dont_fade(true),
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

        let block = default_block(config);
        let mut level = Level::new("porpeties", "arrowslasharrow", None, None);
        level.add_object(block);

        level.export_to_gmd("GMDS/properties.gmd").unwrap();
    }

    #[test]
    fn startpos() {
        // used to test kAXX proerties
        let mut level = Level::new("startpos test", "me", None, None);
        let sp = start_pos(
            GDObjConfig::default().pos(45.0, 45.0),
            0.5,
            triggers::Gamemode::Ball,
            false,
            true,
            true,
            false,
            false,
            false,
            0,
            0,
            false,
        );

        println!("{}", sp.to_string());
        level.add_object(sp);

        level.export_to_gmd("GMDS/startpos.gmd").unwrap();
    }

    #[test]
    fn big_level_parse() {
        let level = Level::from_gmd("GMD_tests/big.gmd");
        let start = Instant::now();
        level.unwrap().get_decrypted_data_ref();
        println!(
            "Parsing took {:.3}ms",
            start.elapsed().as_micros() as f64 / 1000.0
        );
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
    fn level_properties() {
        let levels = Levels::from_local().unwrap();
        let level = &levels.levels[0];
        println!("{level:#?}")
    }
}
