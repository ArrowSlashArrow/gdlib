

pub mod deserialiser;
pub mod serialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{
        deserialiser::{decode_levels_to_string}, 
        gdlevel::{Level, Levels}, 
        gdobj::{self, GDObjConfig}
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
        
        new_level.add_object(gdobj::triggers::move_trigger(
            GDObjConfig::default().groups(vec![1234]), 10, 10, 0.5, 2, false, 0
        ));

        new_level.add_object(gdobj::objs::default_block(GDObjConfig::default().x(15.0).y(15.0)));
        
        levels.add_level(new_level);
        levels.write_to_savefile().unwrap();
    }
}