//! This module conatins methods and structs for operations with individual levels

use std::collections::{BTreeSet, HashSet};

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
    pub fn parse(raw_data: String) -> Self {
        // parse level data
        let raw_data = decompress(raw_data.as_bytes().to_vec()).unwrap();
        let decrypted = std::str::from_utf8(&raw_data[..]).unwrap();
        let split = decrypted.split(";").collect::<Vec<&str>>();

        let headers = split[0].to_string();
        let mut objects = Vec::with_capacity(split.len() - 1);

        for object in &split[1..] {
            if object.len() > 1 {
                objects.push(GDObject::parse_str(object));
            }
        }

        LevelData { headers, objects }
    }
}
