//! This file contains the necessary structs for interfacing with the level(s) themselves
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, read, write},
    io::Cursor,
    path::PathBuf,
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{core::GDError, gdobj::Group};

use plist::{Dictionary, Value};

use crate::{
    core::{b64_decode, b64_encode, get_local_levels_path, proper_plist_tags},
    deserialiser::{decode_levels_to_string, decompress},
    gdobj::{GDObject, ids::metadata::GROUP_PROPERTY_IDS},
    serialiser::{encrypt_level_str, encrypt_savefile_str, stringify_xml},
};

/// The default level header found in GD savefiles.
pub const DEFAULT_LEVEL_HEADERS: &str = "kS38,1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1000_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1001_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1009_7_1_15_1_18_0_8_1|1_255_2_255_3_255_11_255_12_255_13_255_4_-1_6_1002_5_1_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1013_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1014_7_1_15_1_18_0_8_1|1_0_2_125_3_255_11_255_12_255_13_255_4_-1_6_1005_5_1_7_1_15_1_18_0_8_1|1_0_2_200_3_255_11_255_12_255_13_255_4_-1_6_1006_5_1_7_1_15_1_18_0_8_1|,kA13,0,kA15,0,kA16,0,kA14,,kA6,0,kA7,0,kA25,0,kA17,0,kA18,0,kS39,0,kA2,0,kA3,0,kA8,0,kA4,0,kA9,0,kA10,0,kA22,0,kA23,0,kA24,0,kA27,1,kA40,1,kA41,1,kA42,1,kA28,0,kA29,0,kA31,1,kA32,1,kA36,0,kA43,0,kA44,0,kA45,1,kA46,0,kA33,1,kA34,1,kA35,0,kA37,1,kA38,1,kA39,1,kA19,0,kA26,0,kA20,0,kA21,0,kA11,0;";

/// This struct contains other values found in the levels savefile that aren't of any particular use
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct LevelsFileHeaders {
    pub llm_02: Value,
    pub llm_03: Value,
}

/// This struct contains all the levels of the savefile
/// # Fields:
/// * `levels`: The levels. Ones at the beginning are the most recently created.
/// * `headers`: other information necessary for re-encoding
#[derive(Debug, Clone, PartialEq)]
pub struct Levels {
    /// All levels in the savefile
    pub levels: Vec<Level>,
    /// Headers of the level file
    pub headers: LevelsFileHeaders,
}

/// This struct contains level data that has not yet been decrypted
#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedLevelData {
    /// Raw level data
    pub data: String,
}

/// This struct contains the objects of a level and its headers
/// # Fields:
/// * `objects`: Array of objects
/// * `headers`: Other important information about the level
#[derive(Clone, Debug, PartialEq)]
pub struct LevelData {
    /// Level header string
    pub headers: String,
    /// Level objects
    pub objects: Vec<GDObject>,
}

/// Enum that contains either a raw encrypted level string or decrypted level object
#[derive(Clone, Debug, PartialEq)]
pub enum LevelState {
    /// Raw encrypted data
    Encrypted(EncryptedLevelData),
    /// Parsed, structured data
    Decrypted(LevelData),
}

impl Levels {
    /// Returns the levels in CCLocalLevels.dat if retrievable
    #[inline]
    pub fn from_local() -> Result<Self, GDError> {
        Levels::from_decrypted(decode_levels_to_string()?)
    }

    /// Parses raw savefile string into this struct
    pub fn from_decrypted(s: String) -> Result<Self, GDError> {
        let mut xmltree = Value::from_reader_xml(Cursor::new(s.as_bytes()))?
            .into_dictionary()
            .ok_or(GDError::BadParse)?;

        let levels_dict = xmltree
            .remove("LLM_01")
            .ok_or(GDError::BadParse)?
            .into_dictionary()
            .ok_or(GDError::BadParse)?;
        let llm_02 = xmltree.remove("LLM_02")
            .ok_or(GDError::BadParse)?;
        let llm_03 = xmltree.remove("LLM_03")
            .ok_or(GDError::BadParse)?;

        // these are stored as "k_0": <level>, "k_1": <level>, etc. in the savefile,
        // the vec prserves that order.
        let collected_dicts = levels_dict
            .into_iter()
            .filter_map(|(k, v)| match k.as_str() {
                "_isArr" => None,
                _ => v.into_dictionary(),
            })
            .collect::<Vec<Dictionary>>();

        #[cfg(feature = "parallel")]
        let levels_parsed: Vec<Level> = collected_dicts
            .into_par_iter()
            .map(Level::from_dict)
            .collect();

        #[cfg(not(feature = "parallel"))]
        let levels_parsed: Vec<Level> = collected_dicts
            .into_iter()
            .map(Level::from_dict)
            .collect();

        let levels = Levels {
            levels: levels_parsed, // one of these might be for lists. will consider that later
            headers: LevelsFileHeaders {
                llm_02,
                llm_03,
            },
        };

        Ok(levels)
    }

    /// Adds a level to the beginning of `self.levels`
    pub fn add_level(&mut self, level: Level) {
        self.levels.insert(0, level);
    }

    /// Exports this struct as XML to a String
    pub fn export_to_string(&mut self) -> String {
        let mut dict = Dictionary::new();

        let mut levels_dict = Dictionary::new();
        levels_dict.insert("_isArr".to_string(), Value::from(true));
        // for (idx, level) in self.levels.iter().enumerate() {
        //     levels_dict.insert(format!("k_{idx}"), Value::from(level.to_dict()));
        // }

        #[cfg(feature = "parallel")]
        let level_dict_entries = self
            .levels
            .par_iter()
            .enumerate()
            .map(|(idx, level)| (format!("k_{idx}"), Value::from(level.to_dict())))
            .collect::<Vec<(String, Value)>>();

        #[cfg(not(feature = "parallel"))]
        let level_dict_entries = self
            .levels
            .iter()
            .enumerate()
            .map(|(idx, level)| (format!("k_{idx}"), Value::from(level.to_dict())))
            .collect::<Vec<(String, Value)>>();

        for (key, value) in level_dict_entries {
            levels_dict.insert(key, value);
        }

        dict.insert("LLM_01".to_string(), plist::Value::Dictionary(levels_dict));
        dict.insert("LLM_02".to_string(), self.headers.llm_02.clone());
        dict.insert("LLM_03".to_string(), self.headers.llm_03.clone());

        format!(
            "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\">{}</plist>",
            stringify_xml(&dict, true)
        )
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat
    pub fn export_to_savefile(&mut self) -> Result<(), GDError> {
        let savefile = get_local_levels_path()
            .ok_or(GDError::NoAvailableSavefile)?;
        let export_str = encrypt_savefile_str(&self.export_to_string());
        write(savefile, export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to a given file
    pub fn export_to_file(&mut self, file: PathBuf) -> Result<(), GDError> {
        let export_str = encrypt_savefile_str(&self.export_to_string());
        write(file, export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat and creates a backup, CCLocalLevels.dat.bak
    pub fn export_to_savefile_with_backup(&mut self) -> Result<(), GDError> {
        let savefile = get_local_levels_path()
            .ok_or(GDError::NoAvailableSavefile)?;
        let backup_path = format!("{}.bak", savefile.to_string_lossy());
        write(backup_path, read(&savefile)?)?;

        let export_str = encrypt_savefile_str(&self.export_to_string());
        write(savefile, export_str)?;
        Ok(())
    }
}

/// Warning when using this fn: if the data isn't valid UTF8, the fn WILL panic!
#[inline]
fn vec_as_str(data: &[u8]) -> String {
    core::str::from_utf8(data).unwrap().to_string()
}

/// This struct contains level-specific information
/// # Fields:
/// * `title`: Title of the level
/// * `author`: Author of the level
/// * `description`: Author of the description
/// * `data`: Encrypted or decrypted level data
/// * `properties`: Other unspecified properties of this level
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    /// Title of the level
    pub title: Option<String>,
    /// Author of the level
    pub author: Option<String>,
    /// Level description, which is a base64-encoded string
    pub description: Option<String>,
    /// Level data as a [`LevelState`]
    pub data: Option<LevelState>,
    /// Song used in the level
    pub song: Option<i64>,
    /// Level properties
    pub properties: HashMap<String, Value>,
}

impl Level {
    /// Default constructor
    /// # Arguments:
    /// * `title`: Title of the level
    /// * `author`: Who made the level
    /// * `desciption`: (Optional) description of the level
    /// * `song`: (Optional) Song of the level, defaults to stereo madness
    pub fn new<T: Into<String>>(
        title: T,
        author: T,
        description: Option<T>,
        song: Option<i64>,
    ) -> Self {
        Level {
            title: Some(title.into()),
            author: Some(author.into()),
            description: description.map(|desc| b64_encode(desc.into().as_bytes())),
            data: Some(LevelState::Decrypted(LevelData {
                headers: DEFAULT_LEVEL_HEADERS.to_string(),
                objects: vec![],
            })),
            song,
            properties: Level::default_properties(),
        }
    }

    /// Generates a hashmap with default level perties
    #[must_use]
    pub fn default_properties() -> HashMap<String, Value> {
        let ki6_dict: Dictionary = (0..15)
            .map(|i| (i.to_string(), Value::from("0")))
            .collect();

        // genuienly have no clue wht any of these are
        vec![
            ("kCEK", Value::from(4)),
            ("k18", Value::from(1)),
            (
                "k101",
                Value::from("0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0"),
            ),
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
            ("kI6", Value::from(ki6_dict)),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<HashMap<String, Value>>()
    }

    /// Parses a .gmd file to a `Level` object
    pub fn from_gmd<T: Into<PathBuf>>(path: T) -> Result<Self, GDError> {
        let file = proper_plist_tags(vec_as_str(&fs::read(path.into())?))?;
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?
            .into_dictionary()
            .ok_or(GDError::BadParse)?;

        Ok(Level::from_dict(xmltree))
    }

    /// Exports the level to a .gmd file
    pub fn export_to_gmd<T: Into<PathBuf>>(&self, path: T) -> Result<(), GDError> {
        let export_str = format!(
            "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\">{}</plist>",
            stringify_xml(&self.to_dict(), true)
        );

        fs::write(path.into(), export_str)?;
        Ok(())
    }

    /// Parses a `plist::Dictionary` into a Level object
    pub(crate) fn from_dict(d: Dictionary) -> Self {
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

        for (property, value) in d {
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
            level_data = Some(LevelState::Encrypted(EncryptedLevelData { data: d }));
        }

        Level {
            title,
            author,
            description,
            data: level_data,
            song,
            properties,
        }
    }

    /// Returns the level data as unencrypted.
    /// Level data is left unencrypted when parsing the level as it is slow.
    pub fn decrypt_level_data(&mut self) {
        let parsed = match &self.data {
            Some(data) => match data {
                LevelState::Encrypted(encrypted) => LevelData::parse(&encrypted.data),
                LevelState::Decrypted(_) => return, // already decrypted
            },
            None => return, // no level data
        };

        self.data = Some(LevelState::Decrypted(parsed));
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data.
    pub fn get_decrypted_data(&self) -> Option<LevelData> {
        let parsed = match &self.data {
            Some(data) => match data {
                LevelState::Encrypted(encrypted) => LevelData::parse(&encrypted.data),
                LevelState::Decrypted(d) => return Some(d.clone()), // already decrypted
            },
            None => return None, // no level data
        };

        Some(parsed)
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data.
    pub fn get_decrypted_data_ref(&mut self) -> Option<&mut LevelData> {
        self.decrypt_level_data();
        match &mut self.data {
            Some(d) => {
                if let LevelState::Decrypted(data) = d {
                    Some(data)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Returns this object as a `plist::Dictionary`
    #[must_use]
    pub fn to_dict(&self) -> Dictionary {
        let mut properties = Dictionary::new();
        if let Some(v) = self.title.as_ref() {
            properties.insert("k2".to_string(), Value::from(v as &str));
        }
        if let Some(v) = self.description.as_ref() {
            properties.insert("k3".to_string(), Value::from(v as &str));
        }
        if let Some(v) = self.data.as_ref() {
            let str = match v {
                LevelState::Decrypted(data) => &data.serialise_to_string(),
                LevelState::Encrypted(data) => &data.data,
            };
            properties.insert("k4".to_string(), Value::from(str as &str));
        }
        if let Some(v) = self.author.as_ref() {
            properties.insert("k5".to_string(), Value::from(v as &str));
        }
        if let Some(v) = self.song {
            properties.insert("k45".to_string(), Value::from(v));
        }

        for (p, val) in &self.properties {
            properties.insert(p.clone(), val.clone());
        }

        properties
    }

    /// Adds a `GDObject` to `self.objects`
    pub fn add_object(&mut self, object: GDObject) {
        if let Some(data) = &mut self.data {
            match data {
                LevelState::Decrypted(state) => {
                    state.objects.push(object);
                }
                LevelState::Encrypted(_) => (),
            }
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info_str = match &self.data {
            Some(d) => match d {
                LevelState::Encrypted(enc) => &format!("{} Bytes", enc.data.len()),
                LevelState::Decrypted(dec) => &format!("{} Objects", dec.objects.len()),
            },
            None => "Empty",
        };

        write!(
            f,
            "\"{}\" ({}) by {} using song {}; {info_str}",
            self.title.clone().unwrap_or("<No title>".to_string()),
            vec_as_str(&b64_decode(
                self.description
                    .clone()
                    .unwrap_or("PE5vIGRlc2NyaXB0aW9uPg==".to_string())
                    .as_bytes()
            )?),
            self.author
                .clone()
                .unwrap_or("<Unknown author>".to_string()),
            self.song.unwrap_or(0)
        )
    }
}

impl LevelData {
    /// Serialises this object to a string by serialising each subsequent component.
    #[must_use]
    pub fn serialise_to_string(&self) -> String {
        #[cfg(feature = "parallel")]
        let object_data = self
            .objects
            .par_iter()
            .map(GDObject::serialise_to_string)
            .collect::<Vec<String>>()
            .join("");

        #[cfg(not(feature = "parallel"))]
        let object_data = {
            let mut data = String::with_capacity(self.objects.len() * 64);
            for obj in &self.objects {
                data.push_str(&obj.serialise_to_string());
            }
            data
        };

        let mut unencrypted = String::with_capacity(self.headers.len() + object_data.len() + 1);
        unencrypted.push_str(&self.headers);
        unencrypted.push(';');
        unencrypted.push_str(&object_data);

        vec_as_str(&encrypt_level_str(&unencrypted))
    }

    /// Returns a list of all the groups that contain at least one object
    #[must_use]
    pub fn get_used_groups(&self) -> Vec<Group> {
        if self.objects.is_empty() {
            return vec![];
        }

        // let mut groups = HashSet::new();
        #[cfg(feature = "parallel")]
        let mut groups = self
            .objects
            .par_iter()
            .flat_map_iter(|obj| obj.config.groups.iter())
            .copied()
            .collect::<Vec<Group>>();

        #[cfg(not(feature = "parallel"))]
        let mut groups = self
            .objects
            .iter()
            .flat_map(|obj| obj.config.groups.iter())
            .copied()
            .collect::<Vec<Group>>();

        groups.sort();
        groups.dedup();
        groups
    }

    /// Returns a list of all the groups that do not contain any objects
    #[must_use]
    pub fn get_unused_groups(&self) -> Vec<Group> {
        // let all: BTreeSet<Group> = (1..10000).map(Group::Regular).collect();
        // let used: BTreeSet<Group> = self.get_used_groups().into_iter().collect();

        // all.difference(&used).cloned().collect::<Vec<Group>>()
        let mut used: [bool; 10_000] = [false; 10_000];
        for object in &self.objects {
            for group in &object.config.groups {
                if let Group::Regular(g) = group
                    && (1..10_000).contains(g) {
                    used[*g as usize] = true;
                }
            }
        }

        let mut unused: Vec<Group> = Vec::with_capacity(9_999);
        for id in 1..10_000 {
            if !used[id as usize] {
                unused.push(Group::Regular(id));
            }
        }

        unused
    }

    /// Returns a list of all groups used as arguments in triggers
    #[must_use]
    pub fn get_argument_groups(&self) -> Vec<i16> {
        if self.objects.is_empty() {
            return vec![];
        }

        #[cfg(feature = "parallel")]
        let mut groups = self
            .objects
            .par_iter()
            .flat_map_iter(|object| {
                let mut groups = Vec::new();
                for p in GROUP_PROPERTY_IDS {
                    if let Some(val) = object.get_property(*p) {
                        match val {
                            crate::gdobj::GDValue::Group(g) => groups.push(g),
                            crate::gdobj::GDValue::GroupList(gs) => groups.extend(gs.iter().copied()),
                            _ => {}
                        }
                    }
                }
                groups
            })
            .collect::<Vec<i16>>();

        #[cfg(not(feature = "parallel"))]
        let mut groups = Vec::with_capacity(self.objects.len());

        #[cfg(not(feature = "parallel"))]
        for object in &self.objects {
            for p in GROUP_PROPERTY_IDS {
                if let Some(val) = object.get_property(*p) {
                    match val {
                        crate::gdobj::GDValue::Group(g) => groups.push(g),
                        crate::gdobj::GDValue::GroupList(gs) => groups.extend(gs.iter()),
                        _ => {}
                    }
                }
            }
        }

        groups.sort();
        groups.dedup();
        groups
    }

    /// Parse raw level data to this struct
    #[must_use]
    pub fn parse<T: AsRef<str>>(raw_data: T) -> Self {
        let raw_data = raw_data.as_ref();
        // parse level data
        let raw_data = decompress(raw_data.as_bytes().to_vec()).unwrap();
        let decrypted = std::str::from_utf8(&raw_data[..]).unwrap();
        
        let split: Vec<&str> = decrypted.split(';').collect();
        let headers = split.first().unwrap_or(&"").to_string();
        let object_slice = split.get(1..).unwrap_or(&[]);
        
        #[cfg(feature = "parallel")]
        let objects = object_slice
            .par_iter()
            .filter(|obj| obj.len() > 1)
            .map(GDObject::parse_str)
            .collect();

        #[cfg(not(feature = "parallel"))]
        let objects = object_slice
            .iter()
            .filter(|obj| obj.len() > 1)
            .map(GDObject::parse_str)
            .collect();

        LevelData { headers, objects }
    }
}
