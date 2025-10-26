#![doc = include_str!("../README.md")]

pub mod deserialiser;
pub mod serialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{
        deserialiser::decode_levels_to_string, 
        gdlevel::{Level, Levels},
        gdobj::{misc::{self, default_block}, triggers::{self, move_trigger, DefaultMove, MoveEasing}, GDObjConfig}
    };

    #[test]
    fn decode_savefile_test() {
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
        let mut new_level = Level::new("rust websocket tutorial 2", "arrowslasharrow", Some("dont use rust"), Some(857925));
        
        new_level.add_object(triggers::move_trigger(
            GDObjConfig::default().pos(45.0, 45.0).groups([1234]), 
            triggers::MoveMode::Default(triggers::DefaultMove {
                dx: 45.0,
                dy: 54.0,
                x_lock: None,
                y_lock: None
            }), 
            0.50, 1, false, true, Some((MoveEasing::ElasticInOut, 1.50))
        ));

        new_level.add_object(misc::default_block(GDObjConfig::default().x(15.0).y(15.0)));
        
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
    fn gmd_conversion() {
        let level = Level::from_gmd("GMDS/level.gmd").unwrap();
        level.export_to_gmd("GMDS/level_export.gmd").unwrap();
    }

    #[test]
    fn trigger() {
        let mut level = Level::from_gmd("GMDS/Unnamed 25.gmd").unwrap();
        let objects = &level.get_decrypted_data().unwrap().objects;
        for obj in objects {
            println!("{obj:?}");
        }

        // level.add_object(triggers::item_edit(GDObjConfig::new().pos(45.0, 45.0), 
        //     Some((1, ItemType::Counter)), Some((2, ItemType::Counter)), 3, ItemType::Counter,
        //     0.5, triggers::Op::Set, Some(triggers::Op::Add), Some(triggers::Op::Sub), triggers::RoundMode::Nearest,
        //     triggers::RoundMode::Nearest, triggers::SignMode::Absolute, triggers::SignMode::Negative
        // ));

        // levels.export_to_savefile().unwrap();
    }

    #[test]
    fn move_constructor_test() {
        let mut level = Level::new("move trigger t3st", "arrowslasharrow", None, None);
        level.add_object(move_trigger(
            GDObjConfig::default().pos(45.0, 45.0), 
            triggers::MoveMode::Default(DefaultMove {
                dx: 45.0,
                dy: 54.0,
                x_lock: None,
                y_lock: None
            }), 
            17.38, 
            679, 
            false, 
            true, 
            Some((MoveEasing::ElasticInOut, 1.50))
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
        println!("Unused groups: {:?}", level.get_decrypted_data().unwrap().get_unused_groups());
        println!("Used groups: {:?}", level.get_decrypted_data().unwrap().get_used_groups());
    }

}