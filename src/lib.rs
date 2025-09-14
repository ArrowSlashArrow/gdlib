#![doc = include_str!("../README.md")]

pub mod deserialiser;
pub mod serialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{
        deserialiser::{decode_levels_to_string, decompress, decrypt}, 
        gdlevel::{Level, LevelData, LevelState, Levels},
        gdobj::{misc, triggers::{self, colour_trigger, start_pos, stop_trigger}, GDObjConfig}, serialiser::encrypt_level_str, utils::vec_as_str
    };

    #[test]
    fn decode_savefile_test() {
        // 1,500ms on average for 30mb savefile
        assert!(decode_levels_to_string().is_ok())
    }

    #[test]
    fn parse_to_levels_obj() { 
        let raw_levels_savefile = decode_levels_to_string().unwrap();
        // 400ms on average for 30mb savefile
        Levels::from_decrypted(raw_levels_savefile).unwrap();
    }

    #[test]
    fn add_level_and_export() {
        let mut levels = Levels::from_local().unwrap();
        let mut new_level = Level::new("rust websocket tutorial 2", "arrowslasharrow", Some("dont use rust"), Some(857925));
        
        new_level.add_object(triggers::move_trigger(
            GDObjConfig::default().groups(vec![1234]), 10, 10, 0.5, 2, false, 0
        ));

        new_level.add_object(misc::default_block(GDObjConfig::default().x(15.0).y(15.0)));
        
        levels.add_level(new_level);
        // levels.write_to_savefile_with_backup().unwrap();
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
        let mut levels = Levels::from_local().unwrap();
        let level = levels.levels.first_mut().unwrap();
        let objects = &level.get_decrypted_data().unwrap().objects;
        for obj in objects {
            println!("{obj:?}");
        }

        level.add_object(stop_trigger(GDObjConfig::default(), 50, triggers::StopMode::Resume, false));

        levels.write_to_savefile().unwrap();
    }
}