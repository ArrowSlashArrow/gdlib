//! This module contains the objects necessary for operations relating to the CCLocalLevels file,
//! the Level struct, and its constituent structs excluding the leveldata.
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, read, write},
    io::Cursor,
    path::PathBuf,
};

use crate::{
    cclocallevels::gdlevel::leveldata::{
        DEFAULT_LEVEL_HEADERS, EncryptedLevelData, LevelData, LevelHeader, LevelState,
    },
    core::{GDError, vec_as_str},
};

use plist::{Dictionary, Value};

use crate::{
    cclocallevels::gdobj::GDObject,
    core::io::{decode_levels_to_string, encrypt_savefile_str, stringify_xml},
    core::{b64_decode, b64_encode, get_local_levels_path, proper_plist_tags},
};

pub mod leveldata;

/// This struct contains other values found in the levels savefile that aren't of any particular use
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
// black box
pub struct LevelsFileHeaders {
    // black box
    pub llm02: Value,
    // black box
    pub llm03: Value,
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

impl Levels {
    /// Returns the levels in CCLocalLevels.dat if retrievable
    #[inline(always)]
    pub fn from_local() -> Result<Self, GDError> {
        Levels::from_decrypted(decode_levels_to_string()?)
    }

    /// Parses raw savefile string into this struct
    pub fn from_decrypted(s: String) -> Result<Self, GDError> {
        let mut xmltree = match Value::from_reader_xml(Cursor::new(proper_plist_tags(s).as_bytes()))
        {
            Ok(v) => v.into_dictionary().unwrap(),
            Err(e) => return Err(GDError::BadPlist(e)),
        };

        let levels_dict = xmltree
            .remove("LLM_01")
            .ok_or(GDError::CorruptedSavefile("No LLM_01".into()))?
            .into_dictionary()
            .ok_or(GDError::CorruptedSavefile("LLM_01 is not a dict".into()))?;
        let llm_02 = xmltree
            .remove("LLM_02")
            .ok_or(GDError::CorruptedSavefile("No LLM_02".into()))?;
        let llm_03 = xmltree
            .remove("LLM_03")
            .ok_or(GDError::CorruptedSavefile("No LLM_03".into()))?;

        // these are stored as "k_0": <level>, "k_1": <level>, etc. in the savefile,
        // the vec prserves that order.
        let levels_parsed = levels_dict
            .iter()
            .filter_map(|(k, v)| match k.as_str() {
                "_isArr" => None,
                _ => Some(Level::from_dict(v.as_dictionary().unwrap().clone())),
            })
            .collect::<Vec<Level>>();

        let levels = Levels {
            levels: levels_parsed, // one of these might be for lists. will consider that later
            headers: LevelsFileHeaders {
                llm02: llm_02,
                llm03: llm_03,
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
        for (idx, level) in self.levels.iter().enumerate() {
            levels_dict.insert(format!("k_{idx}"), Value::from(level.to_dict()));
        }

        dict.insert("LLM_01".to_string(), plist::Value::Dictionary(levels_dict));
        dict.insert("LLM_02".to_string(), self.headers.llm02.clone());
        dict.insert("LLM_03".to_string(), self.headers.llm03.clone());

        format!(
            "<?xml version=\"1.0\"?><plist version=\"1.0\" gjver=\"2.0\">{}</plist>",
            stringify_xml(&dict, true)
        )
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat
    pub fn export_to_savefile(&mut self) -> Result<(), GDError> {
        let savefile = get_local_levels_path().unwrap();
        let export_str = encrypt_savefile_str(self.export_to_string());
        write(savefile, export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to a given file
    pub fn export_to_file(&mut self, file: PathBuf) -> Result<(), GDError> {
        let export_str = encrypt_savefile_str(self.export_to_string());
        write(file, export_str)?;
        Ok(())
    }

    /// Exports this struct as encrypted XML to CCLocalLevels.dat and creates a backup, CCLocalLevels.dat.bak
    pub fn export_to_savefile_with_backup(&mut self) -> Result<(), GDError> {
        let savefile = get_local_levels_path().unwrap();
        let backup_path = format!("{}.bak", savefile.to_string_lossy());
        write(backup_path, read(&savefile)?)?;

        let export_str = encrypt_savefile_str(self.export_to_string());
        write(savefile, export_str)?;
        Ok(())
    }
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
            description: description.map(|desc| b64_encode(desc.into().as_bytes().to_vec())),
            data: Some(LevelState::Decrypted(LevelData {
                headers: LevelHeader::parse(DEFAULT_LEVEL_HEADERS).unwrap(),
                objects: vec![],
            })),
            song,
            properties: Level::default_properties(),
        }
    }

    /// Generates a hashmap with default level perties
    pub fn default_properties() -> HashMap<String, Value> {
        let mut ki6_dict = Dictionary::new();
        for i in 0..15 {
            ki6_dict.insert(format!("{i}"), Value::from("0"));
        }

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
        let file = proper_plist_tags(vec_as_str(&fs::read(path.into())?));
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?
            .as_dictionary_mut()
            .unwrap()
            .clone();

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
            properties,
        }
    }

    /// Returns the level data as unencrypted.
    /// Level data is left unencrypted when parsing the level as it is slow.
    pub fn decrypt_level_data(&mut self) -> Result<(), GDError> {
        let raw_data = if let Some(data) = &self.data
            && let LevelState::Encrypted(enc) = data
        {
            enc.data.clone()
        } else {
            return Ok(());
        };

        self.data = Some(LevelState::Decrypted(LevelData::parse(raw_data).ok_or(
            GDError::CorruptedSavefile("Unable to parse level header".into()),
        )?));
        Ok(())
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data.
    pub fn get_decrypted_data(&self) -> Option<LevelData> {
        let raw_data = match self.data.clone() {
            Some(data) => match data {
                LevelState::Encrypted(encrypted) => encrypted.data.clone(),
                LevelState::Decrypted(d) => return Some(d), // already decrypted
            },
            None => return None, // no level data
        };

        LevelData::parse(raw_data)
    }

    /// Returns the decrypted level data as a `LevelData` object if there is data.
    pub fn get_decrypted_data_ref(&mut self) -> Option<&mut LevelData> {
        self.decrypt_level_data().ok()?;
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
                LevelState::Decrypted(data) => data.serialise_to_string(),
                LevelState::Encrypted(data) => data.data,
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

        properties
    }

    /// Adds a GDObject to `self.objects`
    pub fn add_object(&mut self, object: GDObject) {
        if let Some(data) = &mut self.data {
            match data {
                LevelState::Decrypted(state) => {
                    state.objects.push(object);
                }
                LevelState::Encrypted(_) => (),
            };
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
                    .to_vec()
            )),
            self.author
                .clone()
                .unwrap_or("<Unknown author>".to_string()),
            self.song.unwrap_or(0)
        )
    }
}
