//! This module conatins methods and structs for operations with individual levels

use std::collections::{BTreeSet, HashSet};
use std::fmt::Display;

use crate::cclocallevels::gdobj::lookup::get_level_header_property_type;
use crate::cclocallevels::gdobj::structs::{Gamemode, Speed};

use crate::{
    cclocallevels::gdobj::{
        GDObject,
        lookup::PROPERTY_TABLE,
        structs::{GDObjPropType, GDValue, Group},
    },
    core::{
        io::{decompress, encrypt_level_str},
        vec_as_str,
    },
};

/// Default level header string
pub const DEFAULT_LEVEL_HEADERS: &str = "kS38,1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1000_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1001_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1009_7_1_15_1_18_0_8_1|1_255_2_255_3_255_11_255_12_255_13_255_4_-1_6_1002_5_1_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1013_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1014_7_1_15_1_18_0_8_1|1_0_2_125_3_255_11_255_12_255_13_255_4_-1_6_1005_5_1_7_1_15_1_18_0_8_1|1_0_2_200_3_255_11_255_12_255_13_255_4_-1_6_1006_5_1_7_1_15_1_18_0_8_1|,kA13,0,kA15,0,kA16,0,kA14,,kA6,0,kA7,0,kA25,0,kA17,0,kA18,0,kS39,0,kA2,0,kA3,0,kA8,0,kA4,0,kA9,0,kA10,0,kA22,0,kA23,0,kA24,0,kA27,1,kA40,1,kA41,1,kA42,1,kA28,0,kA29,0,kA31,1,kA32,1,kA36,0,kA43,0,kA44,0,kA45,1,kA46,0,kA33,1,kA34,1,kA35,0,kA37,1,kA38,1,kA39,1,kA19,0,kA26,0,kA20,0,kA21,0,kA11,0";

const KA_SIZE: usize = 64;
const KS_SIZE: usize = 48;

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
    pub headers: LevelHeader,
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

impl LevelData {
    /// Serialises this object to a string by serialising each subsequent component.
    pub fn serialise_to_string(&self) -> String {
        let objstr = self
            .objects
            .iter()
            .map(|obj| obj.serialise_to_string())
            .collect::<Vec<String>>()
            .join("");
        let unencrypted = format!("{};{objstr}", self.headers.clone());
        vec_as_str(&&encrypt_level_str(unencrypted))
    }

    /// Returns a list of all the groups that contain at least one object
    pub fn get_used_groups(&self) -> Vec<Group> {
        if self.objects.is_empty() {
            return vec![];
        }

        let mut groups = HashSet::new();

        for object in self.objects.iter() {
            groups.extend(object.config.groups.iter());
        }
        let mut arr: Vec<Group> = groups.into_iter().collect();
        arr.sort();
        arr
    }

    /// Returns a list of all the groups that do not contain any objects
    pub fn get_unused_groups(&self) -> Vec<Group> {
        let all: BTreeSet<Group> = (1..10000).map(Group::Regular).collect();
        let used: BTreeSet<Group> = self.get_used_groups().into_iter().collect();

        all.difference(&used).cloned().collect::<Vec<Group>>()
    }

    /// Returns a list of all groups used as arguments in triggers
    pub fn get_argument_groups(&self) -> Vec<i16> {
        if self.objects.is_empty() {
            return vec![];
        }

        // this should really be a const map, but that is impossible in the current version of rust.
        // however, the performance cost is negligible since we only generate this list once per search.
        let group_properties = PROPERTY_TABLE
            .entries()
            .filter_map(|(id, info)| {
                if info.1 == GDObjPropType::Group {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect::<Vec<u16>>();

        let mut groups = Vec::with_capacity(self.objects.len());

        for object in self.objects.iter() {
            for p in group_properties.iter() {
                if let Some(val) = object.get_property(*p) {
                    match val {
                        GDValue::Group(g) => groups.push(g),
                        GDValue::GroupList(gs) => groups.extend(gs.to_vec()),
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
    pub fn parse(raw_data: String) -> Option<Self> {
        // parse level data
        let raw_data = decompress(raw_data.as_bytes().to_vec()).unwrap();
        let decrypted = std::str::from_utf8(&raw_data[..]).unwrap();
        let split = decrypted.split(";").collect::<Vec<&str>>();

        // level start string
        let headers = split[0].to_string();
        let level_headers = LevelHeader::parse(&headers)?;
        let mut objects = Vec::with_capacity(split.len() - 1);

        for object in &split[1..] {
            if object.len() > 1 {
                objects.push(GDObject::parse_str(object));
            }
        }

        Some(LevelData {
            headers: level_headers,
            objects,
        })
    }
}

/// Contains the properties of the level header string.
#[derive(Clone, Debug, PartialEq)]
pub struct LevelHeader {
    /// All properties that are in kAxx format. There are at least 50 kA properties.
    pub ka: [Option<HeaderValue>; KA_SIZE],
    /// All properties that are in kSxx format. There are 39 known kS properties.
    pub ks: [Option<HeaderValue>; KS_SIZE],
}

#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum HeaderValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    Gamemode(Gamemode),
    Speed(Speed),
    // todo: parse
    GuidelineString(String),
    // todo: parse
    ColourString(String),
}

impl HeaderValue {
    /// Parses an input string with a given type to this object
    pub fn parse(val: &str, ptype: HeaderValueType) -> Option<Self> {
        match ptype {
            HeaderValueType::Int => Some(Self::Int(val.parse::<i32>().ok()?)),
            HeaderValueType::Float => Some(Self::Float(val.parse::<f32>().ok()?)),
            HeaderValueType::Bool => Some(Self::Bool(val.parse::<i32>().ok()? != 0)),
            HeaderValueType::Gamemode => Some(Self::Gamemode(
                Gamemode::try_from(val.parse::<i32>().ok()?).ok()?,
            )),
            HeaderValueType::Speed => {
                Some(Self::Speed(Speed::try_from(val.parse::<i32>().ok()?).ok()?))
            }
            HeaderValueType::ColourString => Some(Self::ColourString(val.to_owned())),
            HeaderValueType::GuidelineString => Some(Self::GuidelineString(val.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
/// Type enum for header values
pub enum HeaderValueType {
    Int,
    Float,
    Bool,
    Gamemode,
    Speed,
    GuidelineString,
    ColourString,
}

impl Display for HeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => {
                    if *b {
                        "true".to_owned()
                    } else {
                        "false".to_owned()
                    }
                }
                Self::Float(f) => f.to_string(),
                Self::Int(i) => i.to_string(),
                Self::Speed(s) => (*s as i32).to_string(),
                Self::Gamemode(g) => (*g as i32).to_string(),
                Self::ColourString(c) => c.clone(),
                Self::GuidelineString(g) => g.clone(),
            },
        )
    }
}

impl Display for LevelHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kas = self
            .ka
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if let Some(val) = v {
                    Some(format!("kA{idx},{val}"))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        let kss = self
            .ks
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if let Some(val) = v {
                    Some(format!(",kS{idx},{val}"))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{kas},{kss}")
    }
}

impl LevelHeader {
    /// Parses the input string to this object
    pub fn parse(s: &str) -> Option<Self> {
        println!("{s}");
        let mut headers_kv = s.split(",");
        let mut ka_props: [Option<HeaderValue>; KA_SIZE] = [const { None }; KA_SIZE];
        let mut ks_props: [Option<HeaderValue>; KS_SIZE] = [const { None }; KS_SIZE];

        while let (Some(k), Some(v)) = (headers_kv.next(), headers_kv.next()) {
            // assume that all properties start with either kA or kS
            let is_ks = k.starts_with("kS");
            let prop_idx = match k[2..].parse::<u16>() {
                Ok(n) => n,
                Err(_) => {
                    continue;
                }
            };

            let ptype = if let Some(t) =
                get_level_header_property_type(1000u16 * (is_ks as u16) + prop_idx)
            {
                t
            } else {
                // assume int
                HeaderValueType::Int
            };
            match is_ks {
                true => ks_props[prop_idx as usize] = HeaderValue::parse(v, ptype),
                false => ka_props[prop_idx as usize] = HeaderValue::parse(v, ptype),
            }
        }

        Some(Self {
            ka: ka_props,
            ks: ks_props,
        })
    }

    /// Gets a property based on the index. The index must be obtained from `gdobj::ids::level_header`.
    pub fn get_property(&self, property: u16) -> Option<&HeaderValue> {
        if property > 1000 {
            self.ks
                .get((property - 1000) as usize)
                .map(|o| o.as_ref())
                .flatten()
        } else {
            self.ka.get(property as usize).map(|o| o.as_ref()).flatten()
        }
    }
    /// Sets a property based on the index. The index must be obtained from `gdobj::ids::level_header`.
    pub fn set_property(&mut self, property: u16, value: HeaderValue) {
        if property > 1000 {
            self.ks[(property - 1000) as usize] = Some(value);
        } else {
            self.ka[property as usize] = Some(value);
        }
    }

    /// Removes a property based on the index. The index must be obtained from `gdobj::ids::level_header`.
    pub fn del_property(&mut self, property: u16) {
        if property > 1000 {
            self.ks[(property - 1000) as usize] = None;
        } else {
            self.ka[property as usize] = None;
        }
    }
}
