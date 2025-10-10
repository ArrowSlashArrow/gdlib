//! This file contains the necessary structs for interfacing with the level(s) themselves
use std::{collections::{BTreeSet, HashMap, HashSet}, error::Error, fmt::Display, fs::{self, read, write}, io::Cursor, path::PathBuf};

use plist::{Dictionary, Value};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{deserialiser::{decode_levels_to_string, decompress}, gdobj::GDObject, serialiser::{encrypt_level_str, encrypt_savefile_str, stringify_xml}, utils::{b64_decode, b64_encode, get_local_levels_path, proper_plist_tags, vec_as_str}};

/// This is the default level header 
pub const DEFAULT_LEVEL_HEADERS: &str = "kS38,1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1000_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1001_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1009_7_1_15_1_18_0_8_1|1_255_2_255_3_255_11_255_12_255_13_255_4_-1_6_1002_5_1_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1013_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1014_7_1_15_1_18_0_8_1|1_0_2_125_3_255_11_255_12_255_13_255_4_-1_6_1005_5_1_7_1_15_1_18_0_8_1|1_0_2_200_3_255_11_255_12_255_13_255_4_-1_6_1006_5_1_7_1_15_1_18_0_8_1|,kA13,0,kA15,0,kA16,0,kA14,,kA6,0,kA7,0,kA25,0,kA17,0,kA18,0,kS39,0,kA2,0,kA3,0,kA8,0,kA4,0,kA9,0,kA10,0,kA22,0,kA23,0,kA24,0,kA27,1,kA40,1,kA41,1,kA42,1,kA28,0,kA29,0,kA31,1,kA32,1,kA36,0,kA43,0,kA44,0,kA45,1,kA46,0,kA33,1,kA34,1,kA35,0,kA37,1,kA38,1,kA39,1,kA19,0,kA26,0,kA20,0,kA21,0,kA11,0;";

/// This struct contains other values found in the levels savefile that aren't of any particular use
pub struct LevelsFileHeaders {
    pub llm02: Value,
    pub llm03: Value
}

/// This struct contains all the levels of the savefile
/// # Fields:
/// * `levels`: The levels. Ones at the beginning are the most recently created.
/// * `headers`: other information necessary for re-encoding
pub struct Levels {
    pub levels: Vec<Level>,
    pub headers: LevelsFileHeaders
}

/// This struct contains level data that has not yet been decrypted 
#[derive(Clone, Debug)]
pub struct EncryptedLevelData {
    pub data: String
}

/// This struct contains the objects of a level and its headers
/// # Fields:
/// * `objects`: Array of objects
/// * `headers`: Other important information about the level 
#[derive(Clone, Debug)]
pub struct LevelData {
    pub headers: String,
    pub objects: Vec<GDObject>
}

/// Enum that contains either a encrypted level string or decrypted level object 
#[derive(Clone, Debug)]
pub enum LevelState {
    Encrypted(EncryptedLevelData),
    Decrypted(LevelData)
}

/// This struct contains level-specific information
/// # Fields:
/// * `title`: Title of the level
/// * `author`: Author of the level
/// * `description`: Author of the description
/// * `data`: Encrypted or decrypted level data
/// * `properties`: Other unspecified properties of this level
#[derive(Debug, Clone)]
pub struct Level {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub data: Option<LevelState>,
    pub song: Option<i64>,
    pub properties: HashMap<String, Value>
}

impl Levels {
    /// Returns the levels in CCLocalLevels.dat if retrievable
    pub fn from_local() -> Result<Self, Box<dyn Error>> {
        match decode_levels_to_string() {
            Ok(v) => Levels::from_decrypted(v),
            Err(e) => Err(e)
        }
    }

    /// Parses raw savefile string into this struct
    pub fn from_decrypted(s: String) -> Result<Self, Box<dyn Error>> {
        let xmltree = Value::from_reader_xml(
            Cursor::new(proper_plist_tags(s).as_bytes())
        )?.as_dictionary_mut().unwrap().clone();
        

        let levels_dict = xmltree.get("LLM_01").unwrap().clone().as_dictionary().unwrap().clone();
        let llm_02 = xmltree.get("LLM_02").unwrap().clone();
        let llm_03 = xmltree.get("LLM_03").unwrap().clone();

        // these are stored as "k_0": <level>, "k_1": <level>, etc. in the savefile, 
        // the vec prserves that order.
        let levels_parsed = levels_dict.iter().filter_map(|(k, v)| {
            match k.as_str() {
                "_isArr" => None,
                _ => Some(Level::from_dict(v.as_dictionary().unwrap().clone()))
            } 
        }).collect::<Vec<Level>>();


        let levels = Levels { 
            levels: levels_parsed,  // one of these might be for lists. will consider that later
            headers: LevelsFileHeaders { llm02: llm_02, llm03: llm_03 }
        };

        Ok(levels)
    }

    /// Adds a level to the beginning of `self.levels`
    pub fn add_level(&mut self, level: Level) {
        self.levels.insert(0, level);
    }

    fn default_properties() -> HashMap<String, Value> {
        let mut ki6_dict = Dictionary::new();
        for i in 0..15 {
            ki6_dict.insert(format!("{i}"), Value::from("0"));
        }

        // genuienly have no clue wht any of these are
        let properties = vec![
            ("kCEK", Value::from(4)),
            ("k18", Value::from(1)),
            ("k101", Value::from("0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0")),
            ("k11", Value::from(4598)),
            ("k13", Value::from(true)),
            ("k21", Value::from(2)),
            ("k16", Value::from(1)),
            ("k27", Value::from(4598)),
            ("k50", Value::from(45)),
            ("k47", Value::from(true)),
            ("kI1", Value::from(100.0)),
            ("kI2", Value::from(100.0)),
            ("kI3", Value::from(1.0)),
            ("kI6", Value::from(ki6_dict))
        ].into_iter().map(|(k, v)| (k.to_string(), v)).collect::<HashMap<String, Value>>();

        return properties
    }

    /// Exports this struct as XML to a String
    pub fn export_to_string(&mut self) -> String {
        let mut dict = Dictionary::new();

        let mut levels_dict = Dictionary::new();
        levels_dict.insert("_isArr".to_string(), Value::from(true));
        for (idx, level) in self.levels.iter().enumerate() {
            levels_dict.insert(format!("k_{idx}"), Value::from(level.to_dict()));
        }

        dict.insert("LLM_01".to_string(), plist::Value::Dictionary(levels_dict));
        dict.insert("LLM_02".to_string(), self.headers.llm02.clone());
        dict.insert("LLM_03".to_string(), self.headers.llm03.clone());
        
        return format!("<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\">{}</plist>", stringify_xml(&dict, true));
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat
    pub fn export_to_savefile(&mut self) -> Result<(), Box<dyn Error>>{
        let savefile = get_local_levels_path()?;
        let export_str = encrypt_savefile_str(self.export_to_string());
        write(savefile, export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat and creates a backup, CCLocalLevels.dat.bak
    pub fn export_to_savefile_with_backup(&mut self) -> Result<(), Box<dyn Error>>{
        let savefile = get_local_levels_path()?;
        let backup_path = format!("{}.bak", savefile.to_string_lossy());
        write(backup_path, read(&savefile)?)?;

        let export_str = encrypt_savefile_str(self.export_to_string());
        write(savefile, export_str)?;
        Ok(())
    }
}

impl Level {
    /// Default constructor
    /// # Arguments:
    /// * `title`: Title of the level
    /// * `author`: Who made the level
    /// * `desciption`: (Optional) description of the level
    /// * `song`: (Optional) Song of the level, defaults to stereo madness
    pub fn new<T: Into<String>>(title: T, author: T, description: Option<T>, song: Option<i64>) -> Self {
        Level { 
            title: Some(title.into()), 
            author: Some(author.into()), 
            description: description.map(|desc| b64_encode(desc.into().as_bytes().to_vec())), 
            data: Some(LevelState::Decrypted(LevelData { headers: DEFAULT_LEVEL_HEADERS.to_string(), objects: vec![] })), 
            song, 
            properties: Levels::default_properties()
        }
    }

    /// Parses a .gmd file to a `Level` object
    pub fn from_gmd<T: Into<PathBuf>>(path: T) -> Result<Self, Box<dyn Error>> {
        let file = proper_plist_tags(vec_as_str(&fs::read(path.into())?));
        let xmltree = Value::from_reader_xml(
            Cursor::new(file.as_bytes())
        )?.as_dictionary_mut().unwrap().clone();

        return Ok(Level::from_dict(xmltree));
    }

    pub fn export_to_gmd<T: Into<PathBuf>>(&self, path: T) -> Result<(), Box<dyn Error>> {
        let export_str =  format!(
            "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\">{}</plist>", 
            stringify_xml(&self.to_dict(), true)
        );

        fs::write(path.into(), export_str)?;
        Ok(())
    }

    /// Parses a `plist::Dictionary` into a Level object
    pub fn from_dict(d: Dictionary) -> Self {
        // level data kv pairs
        // k2: level name
        // k3: description
        // k4: level str (encrypted) 
        // k5: author
        // k45: song

        let mut song = None;
        let mut author = None;
        let mut description = None;
        let mut title = None;
        let mut data = None;

        // residual properties
        let mut properties: HashMap<String, Value> = HashMap::new();

        for (property, value) in d.into_iter() {
            match property.as_str() {
                "k2" => title = Some(value.as_string().unwrap().to_owned()),
                "k3" => description = Some(value.as_string().unwrap().to_owned()),
                "k4" => data = Some(value.as_string().unwrap().to_owned()),
                "k5" => author = Some(value.as_string().unwrap().to_owned()),
                "k45" => song = Some(value.as_signed_integer().unwrap()), 
                _ => {
                    properties.insert(property, value);
                }
            }
        }

        let mut level_data: Option<LevelState> = None;
        if let Some(d) = data {
            level_data = Some(LevelState::Encrypted(EncryptedLevelData { data: d }))
        }
       
        Level {
            title,
            author,
            description,
            data: level_data,
            song,
            properties
        }
    }

    /// Returns the level data as unencrypted. 
    /// Level data is left unencrypted when parsing the level as it is slow.
    pub fn decrypt_level_data(&mut self) {
        let raw_data = match &self.data {
            Some(data) => match data {
                LevelState::Encrypted(encrypted) => {
                    encrypted.data.clone()
                },
                LevelState::Decrypted(_) => return // already decrypted
            },
            None => return // no level data
        };

        // parse level data
        let decrypted = vec_as_str(&decompress(raw_data.as_bytes().to_vec()).unwrap());

        let split = decrypted.split(";").collect::<Vec<&str>>();

        let headers = split.first().unwrap().to_owned().to_owned();
        let objects = split[1..].par_iter().filter_map(|objstr| {
            match objstr.len() > 1 {
                true => Some(GDObject::parse_str(*objstr)),
                false => None
            }
        }).collect::<Vec<GDObject>>();
        self.data = Some(LevelState::Decrypted(LevelData { headers, objects }));
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data. 
    pub fn get_decrypted_data(&mut self) -> Option<&mut LevelData> {
        self.decrypt_level_data();
        match &mut self.data {
            Some(d) => {
                if let LevelState::Decrypted(data) = d {
                    Some(data)
                } else {
                    None
                }
            },
            None => return None
        }
    }

    /// Returns this object as a `plist::Dictionary`
    pub fn to_dict(&self) -> Dictionary {
        let mut properties = Dictionary::new();
        if let Some(v) = self.title.clone() {
            properties.insert("k2".to_string(), Value::from(v));
        };
        if let Some(v) = self.description.clone() {
            properties.insert("k3".to_string(), Value::from(v));
        };
        if let Some(v) = self.data.clone() {
            let str = match v {
                LevelState::Decrypted(data) => data.to_string(),
                LevelState::Encrypted(data) => data.data
            };
            properties.insert("k4".to_string(), Value::from(str));
        };
        if let Some(v) = self.author.clone() {
            properties.insert("k5".to_string(), Value::from(v));
        };
        if let Some(v) = self.song {
            properties.insert("k45".to_string(), Value::from(v));
        };

        for (p, val) in self.properties.clone().into_iter() {
            properties.insert(p, val);
        }

        return properties;
    }

    /// Adds a GDObject to `self.objects`
    pub fn add_object(&mut self, object: GDObject) {
        if let Some(data) = &mut self.data {
            match data {
                LevelState::Decrypted(state) => {
                    state.objects.push(object);
                },
                LevelState::Encrypted(_) => return
            };
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info_str = match &self.data {
            Some(d) => match d {
                LevelState::Encrypted(enc) => {
                    &format!("{} Bytes", enc.data.len())
                },
                LevelState::Decrypted(dec) => {
                    &format!("{} Objects", dec.objects.len())
                }
            },
            None => "Empty"
        };

        write!(
            f, 
            "\"{}\" ({}) by {} using song {}; {info_str}", 
            self.title.clone().unwrap_or("<No title>".to_string()), 
            vec_as_str(&b64_decode(
                self.description.clone().unwrap_or("PE5vIGRlc2NyaXB0aW9uPg==".to_string())
            .as_bytes().to_vec())),
            self.author.clone().unwrap_or("<Unknown author>".to_string()),
            self.song.unwrap_or(0)
        )
    }
}

impl LevelData {
    pub fn to_string(&self) -> String {
        let objstr = self.objects.iter()
            .map(|obj| obj.to_string()).collect::<Vec<String>>().join("");
        let unencrypted = format!("{};{objstr}", self.headers.clone());
        return vec_as_str(&encrypt_level_str(unencrypted))
    }

    /// Returns a list of all the groups that contain at least one object
    pub fn get_used_groups(&self) -> Vec<u16> {
        if self.objects.len() == 0 {
            return vec![];
        }

        let mut groups = HashSet::new();
        
        for object in self.objects.iter() {
            groups.extend(object.config.groups.iter());
        };
        let mut arr: Vec<u16> = groups.into_iter().collect();
        arr.sort();
        return arr
    }

    /// Returns a list of all the groups that do not contain any objects
    pub fn get_unused_groups(&self) -> Vec<u16> {
        let all: BTreeSet<u16> = (1..10000).collect();
        let used: BTreeSet<u16> = self.get_used_groups().into_iter().collect();

        all.difference(&used).cloned().collect::<Vec<u16>>()
    }

    /// Returns a list of all groups used as arguments in triggers
    pub fn get_argument_groups(&self) -> Vec<u16> {
        if self.objects.len() == 0 {
            return vec![];
        }

        let mut groups = HashSet::new();
        
        for object in self.objects.iter() {
            for (prop, value) in object.properties.properties.iter() {
                if prop.arg_type == crate::gdobj::GDObjPropType::Group {
                    groups.insert(value);
                }
            }
        };
        let mut arr: Vec<u16> = groups.into_iter().map(|v| v.as_str().unwrap().parse::<u16>().unwrap()).collect();
        arr.sort();
        return arr
    }
}