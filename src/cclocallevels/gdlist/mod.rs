//! GD lists

use std::{
    collections::HashMap,
    fs::{read, write},
    io::Cursor,
    path::PathBuf,
};

use anyhow::Result;
use plist::{Dictionary, Value};

use crate::{
    cclocallevels::gdlevel::{
        GDLevel, PLIST_FOOTER, PLIST_HEADER,
        enums::{GDListDifficulty, GDListType},
        parse_csv, serialise_bool_fields, serialise_fields, serialise_optional_fields, to_csv,
    },
    core::{
        GDError, b64_decode, b64_encode,
        io::{stringify_xml, vec_as_str},
        proper_plist_tags,
        structs::KCEKValue,
    },
};

/// GD list object
///
/// Properties sourced from <https://wyliemaster.github.io/gddocs/#/resources/client/gamesave/list>.
#[derive(Debug, Clone)]
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
    /// Given difficulty of the list. See [`GDListDifficulty`]
    ///
    /// Internal key: `k7`
    pub difficulty: GDListDifficulty,
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
    pub original: Option<i32>,
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
    pub level_ids: Vec<i32>,
    /// Dictionary of all the levels in the list. Levels in this dictionary are summaries and are missing many properties, notably, the level's object data.
    ///
    /// Internal key: `k97`
    pub levels: Vec<GDLevel>,
    /// UNIX timestamp of the level's upload time in seconds.
    ///
    /// Internal key: `k98`
    pub upload_time: i32,
    /// UNIX timestamp of the level's update time in seconds.
    ///
    /// Internal key: `k99`
    pub update_time: Option<i32>,
    /// Amount of diamonds awarded upon beating the list.
    ///
    /// Internal key: `k113`
    pub diamond_reward: i32,
    /// Amount of levels needed for the list to be considered beaten.
    ///
    /// Internal key: `k114`
    pub required_levels: i32,
    /// This value is always [`KCEKValue::GDLevelList`]. See [`KCEKValue`]
    ///
    /// Internal key: `kCEK`
    pub kcek: KCEKValue,

    /// Residual properties that are unused or unaccounted for.
    pub other_properties: HashMap<String, Value>,
}

impl Default for GDList {
    fn default() -> Self {
        Self {
            kcek: KCEKValue::GJLevelList,
            id: 0,
            name: String::new(),
            description: String::new(),
            creator: String::new(),
            difficulty: GDListDifficulty::default(),
            downloads: 0,
            is_uploaded: false,
            version: 0,
            list_type: GDListType::default(),
            likes: 0,
            is_demon: false,
            stars: 0,
            featured: false,
            original: None,
            list_revision: 0,
            account_id: 0,
            unlisted: false,
            favourited: false,
            order: 0,
            level_ids: vec![],
            levels: vec![],
            upload_time: 0,
            update_time: None,
            diamond_reward: 0,
            required_levels: 0,
            other_properties: HashMap::new(),
        }
    }
}

impl GDList {
    /// Parses a plist dictionary a GD list object
    pub fn from_dictionary(d: &Dictionary) -> Option<Self> {
        /* Input dict structure
         * kXX: regular list values
         * k97: dict of levels: {"id": level, ...}
         */

        let mut list = Self::default();

        for (k, v) in d {
            if k == "kCEK" {
                list.kcek = KCEKValue::try_from(v.as_signed_integer().unwrap() as i32).unwrap();
                continue;
            }

            let key_id = match k[1..].parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    list.other_properties.insert(k.clone(), v.clone());
                    continue;
                }
            };

            match key_id {
                1 => list.id = v.as_signed_integer()? as i32,
                2 => list.name = v.as_string()?.to_owned(),
                3 => {
                    // special case: b64 encoded string
                    list.description = vec_as_str(&b64_decode(v.as_string()?).ok()?[..])
                }
                5 => list.creator = v.as_string()?.to_owned(),
                7 => {
                    // special case: difficulty
                    list.difficulty =
                        GDListDifficulty::try_from(v.as_signed_integer()? as i32).ok()?
                }
                11 => list.downloads = v.as_signed_integer()? as i32,
                15 => list.is_uploaded = v.as_boolean()?,
                16 => list.version = v.as_signed_integer()? as i32,
                21 => {
                    // special case: list type
                    list.list_type = GDListType::try_from(v.as_signed_integer()? as i32).ok()?
                }
                22 => list.likes = v.as_signed_integer()? as i32,
                25 => list.is_demon = v.as_boolean()?,
                26 => list.stars = v.as_signed_integer()? as i32,
                27 => list.featured = v.as_boolean()?,
                42 => list.original = Some(v.as_signed_integer()? as i32),
                46 => list.list_revision = v.as_signed_integer()? as i32,
                60 => list.account_id = v.as_signed_integer()? as i32,
                79 => list.unlisted = v.as_boolean()?,
                82 => list.favourited = v.as_boolean()?,
                83 => list.order = v.as_signed_integer()? as i32,
                96 => {
                    // special case: comma-separated list of ids
                    list.level_ids = parse_csv(v)?
                }
                97 => {
                    // special case: levels dictionary
                    // omit id (key) since it is found in the level anyway
                    list.levels = v
                        .as_dictionary()
                        .unwrap()
                        .iter()
                        .map(|(_id, level_dict)| {
                            match GDLevel::from_dict(level_dict.as_dictionary().unwrap()) {
                                Some(l) => l,
                                None => {
                                    panic!("couldnt parse {level_dict:#?}")
                                }
                            }
                        })
                        .collect::<Vec<GDLevel>>();
                }
                98 => list.upload_time = v.as_signed_integer()? as i32,
                99 => list.update_time = Some(v.as_signed_integer()? as i32),
                113 => list.diamond_reward = v.as_signed_integer()? as i32,
                114 => list.required_levels = v.as_signed_integer()? as i32,

                _ => {
                    list.other_properties.insert(k.clone(), v.clone());
                }
            }
        }

        Some(list)
    }

    /// Serialises this object into a [`plist::Dictionary`]
    pub fn to_dict(&self) -> Dictionary {
        /* Field types
         * i32: 1, 11, 16, 22, 26, 46, 60, 83, 98, 113, 114
         * Option<i32>: 42, 99
         * string: 2, 5
         * bool: 15, 25, 27, 79, 82
         *
         * special cases:
         * b64 string: 3
         * list difficulty: 7
         * list type: 21,
         * csv (Vec<i32>): 96
         * {id: level}: 97
         */

        let mut d = Dictionary::new();

        // i32 + difficulty (7) + type (21)
        serialise_fields(
            &mut d,
            &[
                ("k1", self.id),
                ("k7", self.difficulty.to_num()),
                ("k11", self.downloads),
                ("k16", self.version),
                ("k21", self.list_type.to_num()),
                ("k22", self.likes),
                ("k26", self.stars),
                ("k46", self.list_revision),
                ("k60", self.account_id),
                ("k83", self.order),
                ("k98", self.upload_time),
                ("k113", self.diamond_reward),
                ("k114", self.required_levels),
            ],
        );

        // Option<i32>
        serialise_optional_fields(&mut d, &[("k42", self.original), ("k99", self.update_time)]);

        serialise_bool_fields(
            &mut d,
            &[
                ("k15", self.is_uploaded),
                ("k25", self.is_demon),
                ("k27", self.featured),
                ("k79", self.unlisted),
                ("k82", self.favourited),
            ],
        );

        // strings + b64 string (3) + csv (96)
        serialise_fields(
            &mut d,
            &[
                ("k2", &self.name[..]),
                ("k3", &b64_encode(self.description.as_bytes())[..]),
                ("k5", &self.creator[..]),
                ("k96", &to_csv(&self.level_ids)[..]),
            ],
        );

        // k97
        d.insert(
            "k97".into(),
            Value::Dictionary(Dictionary::from_iter(
                self.levels
                    .iter()
                    .map(|l| (l.identity.id.to_string(), l.to_dict())),
            )),
        );

        d.insert("kCEK".into(), Value::from(self.kcek.to_num()));
        d.extend(self.other_properties.clone());

        d
    }

    /// Parses a .gmd file to a `Self` object
    pub fn from_gmdl<T: Into<PathBuf>>(path: T) -> Result<Self, GDError> {
        let file = proper_plist_tags(vec_as_str(&read(path.into())?))?;
        let xmltree = Value::from_reader_xml(Cursor::new(file.as_bytes()))?;

        Ok(
            Self::from_dictionary(xmltree.as_dictionary().unwrap()).ok_or(
                GDError::CorruptedSavefile("Unable to parse file to valid level.".into()),
            )?,
        )
    }

    /// Exports the level to a .gmd file
    pub fn export_to_gmdl<T: Into<PathBuf>>(&self, path: T) -> Result<(), GDError> {
        let export_str = format!(
            "{PLIST_HEADER}{}{PLIST_FOOTER}",
            stringify_xml(&self.to_dict(), true)
        );

        write(path.into(), export_str)?;
        Ok(())
    }
}
