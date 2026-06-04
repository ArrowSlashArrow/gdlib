//! GD lists

use std::collections::HashMap;

use anyhow::{Result, anyhow};
use plist::{Dictionary, Value};

use crate::{
    cclocallevels::gdlevel::GDLevel,
    core::{b64_decode, structs::Difficulty, vec_as_str},
};

/// Container for all the levels saved in the savefile
#[derive(Debug, Clone)]
pub struct GDLists {
    /// List of levels from most recent to least recent
    pub lists: Vec<GDList>,
}

/// GD list object
///
/// Properties sourced from <https://wyliemaster.github.io/gddocs/#/resources/client/gamesave/list>.
#[derive(Debug, Default, Clone)]
pub struct GDList {
    /// ID of the list.
    ///
    /// Internal key: `k1`
    pub id: i32,
    /// The name of the list.
    ///
    /// Internal key: `k2`
    pub name: String,
    /// Description of the list decoded from base-64. The description is internally stored as the base-64 encoded version of the original string, however it is decoded in this struct and re-encoded when serialising.
    ///
    /// Internal key: `k3`
    pub description: String,
    /// Creator of the list
    ///
    /// Internal key: `k5`
    pub creator: String,
    /// Given difficulty of the list.
    ///
    /// Internal key: `k7`
    pub difficulty: Difficulty,
    /// Amount of times this list has been downloaded.
    ///
    /// Internal key: `k11`
    pub downloads: i32,
    /// Whether the list is uploaded to the servers.
    ///
    /// Internal key: `k15`
    pub is_uploaded: bool,
    /// Version number of this list.
    ///
    /// Internal key: `k16`
    pub version: i32,
    /// Type of GDList: [`GDListType`].
    ///
    /// Internal key: `k21`
    pub list_type: GDListType,
    /// The list's like rating (likes - dislikes).
    ///
    /// Internal key: `k22`
    pub likes: i32,
    /// Whether this list is rated a demon or not.
    ///
    /// Internal key: `k25`
    pub is_demon: bool,
    /// Amount of stars rewarded for beating the list.
    ///
    /// Internal key: `k26`
    pub stars: i32,
    /// Whether the list is featured or not.
    ///
    /// Internal key: `k27`
    pub featured: bool,
    /// ID of original list (if the list was copied).
    ///
    /// Internal key: `k42`
    pub original: i32,
    /// The revision of the list.
    ///
    /// Internal key: `k46`
    pub list_revision: i32,
    /// The creator's account ID.
    ///
    /// Internal key: `k60`
    pub account_id: i32,
    /// Whether the list can only be found by searching the ID.
    ///
    /// Internal key: `k79`
    pub unlisted: bool,
    /// Whether the list is favourited.
    ///
    /// Internal key: `k82`
    pub favourited: bool,
    /// List ordering.
    ///
    /// Internal key: `k83`
    pub order: i32,
    /// Comma-separated list of all IDs in the list in order.
    ///
    /// Internal key: `k96`
    pub level_ids: String,
    /// Dictionary of all the levels in the list. Levels in this dictionary are summaries and are missing many properties, notably, the level's object data.
    ///
    /// Internal key: `k97`
    pub levels: Vec<GDLevel>, // TODO: use new GDLevel
    /// UNIX timestamp of the level's upload time in seconds.
    ///
    /// Internal key: `k98`
    pub upload_time: i32,
    /// UNIX timestamp of the level's update time in seconds.
    ///
    /// Internal key: `k99`
    pub update_time: i32,
    /// Amount of diamonds awarded upon beating the list.
    ///
    /// Internal key: `k113`
    pub diamond_reward: i32,
    /// Amount of levels needed for the list to be considered beaten.
    ///
    /// Internal key: `k114`
    pub required_levels: i32,
    /// Residual properties that are unused or unaccounted for.
    pub other_properties: HashMap<String, Value>,
}

impl GDLists {
    /// Parses the given plist dictionary to this struct
    pub fn parse_from_value(v: &Value) -> Result<Self> {
        let dict = v
            .as_dictionary()
            .ok_or(anyhow!("Input value is not a dictionary."))?;
        println!("{:#?}", dict);

        let mut lists = Self {
            lists: Vec::with_capacity(dict.len()),
        };

        for (_, v) in dict {
            if let Value::Dictionary(d) = v {
                lists.lists.push(GDList::from_dictionary(d))
            }
        }

        Ok(lists)
    }
}

impl GDList {
    /// Parses a plist dictionary a GD list object
    pub fn from_dictionary(d: &Dictionary) -> Self {
        let mut list = Self::default();

        for (k, v) in d {
            let key_id = match k[1..].parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    list.other_properties.insert(k.clone(), v.clone());
                    continue;
                }
            };

            // TODO: parse all keys.
            match key_id {
                1 => list.id = v.as_signed_integer().unwrap() as i32,
                2 => list.name = v.as_string().unwrap().to_owned(),
                3 => list.description = vec_as_str(&b64_decode(v.as_string().unwrap())[..]),
                5 => list.creator = v.as_string().unwrap().to_owned(),
                _ => {
                    list.other_properties.insert(k.clone(), v.clone());
                }
            }
        }

        list
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[repr(i32)]
pub enum GDListType {
    #[default]
    None = 0,
    Local = 2,
    Saved = 3,
    Online = 4,
}
