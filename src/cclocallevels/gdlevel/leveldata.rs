//! This module conatins methods and structs for operations with individual levels

use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use crate::{
    cclocallevels::{
        consts::LEVEL_HEADER_PROP_ID_OFFSET,
        gdlevel::leveldata::GDLevelHeaderKey::{A, S},
        gdobj::{
            GDObject,
            ids::metadata::GROUP_PROPERTY_IDS,
            structs::{Colour, GDValue, Gamemode, Group, HSVColour, Speed},
        },
        properties::get_level_header_property_type,
    },
    core::io::{decompress, encrypt_level_str, vec_as_str},
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Default level header string for GD levels.
///
/// This is the state of the level header immediately after initializing new level data.
pub const DEFAULT_LEVEL_HEADERS: &str = "kS38,1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1000_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1001_7_1_15_1_18_0_8_1|1_0_2_102_3_255_11_255_12_255_13_255_4_-1_6_1009_7_1_15_1_18_0_8_1|1_255_2_255_3_255_11_255_12_255_13_255_4_-1_6_1002_5_1_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1013_7_1_15_1_18_0_8_1|1_40_2_125_3_255_11_255_12_255_13_255_4_-1_6_1014_7_1_15_1_18_0_8_1|1_0_2_125_3_255_11_255_12_255_13_255_4_-1_6_1005_5_1_7_1_15_1_18_0_8_1|1_0_2_200_3_255_11_255_12_255_13_255_4_-1_6_1006_5_1_7_1_15_1_18_0_8_1|,kA13,0,kA15,0,kA16,0,kA14,,kA6,0,kA7,0,kA25,0,kA17,0,kA18,0,kS39,0,kA2,0,kA3,0,kA8,0,kA4,0,kA9,0,kA10,0,kA22,0,kA23,0,kA24,0,kA27,1,kA40,1,kA41,1,kA42,1,kA28,0,kA29,0,kA31,1,kA32,1,kA36,0,kA43,0,kA44,0,kA45,1,kA46,0,kA33,1,kA34,1,kA35,0,kA37,1,kA38,1,kA39,1,kA19,0,kA26,0,kA20,0,kA21,0,kA11,0,kA47,0,kA48,1";

/// This struct contains level data that has not yet been decrypted
#[derive(Clone, Debug, PartialEq)]
pub struct GDEncryptedLevelData {
    /// Raw level data
    pub data: String,
}

/// This struct contains the objects of a level and its headers
#[derive(Clone, Debug, PartialEq)]
pub struct GDLevelData {
    /// Headers for a GD level. This field stores all metadata about the level data itself (such as starting colour info and physics options).
    pub headers: HashMap<GDLevelHeaderKey, GDLevelHeaderValue>,
    /// All objects in this level
    pub objects: Vec<GDObject>,
}

/// Enum that contains either a raw encrypted level string or decrypted level object
#[derive(Clone, Debug, PartialEq)]
pub enum GDLevelState {
    /// Raw encrypted data
    Encrypted(GDEncryptedLevelData),
    /// Parsed, structured data
    Decrypted(GDLevelData),
}

#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum GDLevelHeaderValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    Gamemode(Gamemode),
    Speed(Speed),
    GuidelineString(GuidelineString),
    ColourString(Vec<ColourString>),
}

/// Enum for colours of an individual guideline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum GuidelineColour {
    Orange,
    Yellow,
    Green,
    Transparent,
}

/// Descriptor struct for the guildline string
#[derive(Clone, Debug, PartialEq)]
pub struct GuidelineString {
    /// The guidelines themselves as (time, colour) tuples
    pub guidelines: Vec<(String, GuidelineColour)>,
}

/// Descriptor struct for the colour string.
///
/// Reference: <https://boomlings.dev/resources/client/level-components/color-string>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ColourString {
    /// The colour itself
    pub from: Colour,
    /// What this colour changes to
    pub to: Colour,
    /// Player colour that is being copied by this colour
    pub player_colour: PlayerColour,
    /// Using blending
    pub blending: bool,
    /// This colour's channel index
    pub colour_ch_idx: i32,
    /// Opacity of this colour
    pub from_opacity: f32,
    /// What opacity this colour becomes
    pub to_opacity: f32,
    /// Toggles changing opacity
    pub opacity_toggled: bool,
    /// The channel index that this colour inherits (if any)
    pub inherited_col_ch_idx: i32,
    /// HSV of the copied colour
    pub copied_hsv: Option<HSVColour>,
    /// Delta used to change colour
    pub deltatime: f32,
    /// Time of transiton from `from` colour to `to` colour
    pub duration: f32,
    /// @nodoc
    pub copy_opacity: bool,
    /// Unknown property with index 18
    pub unknown_property18: bool,
}

#[repr(i32)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Copying one of these player colours. Used in [`ColourString`]
pub enum PlayerColour {
    #[default]
    None = -1,
    First = 1,
    Second = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
/// Type enum for header values
pub enum GDLevelHeaderValType {
    Int,
    Float,
    Bool,
    Gamemode,
    Speed,
    GuidelineString,
    ColourString,
}

/// Parses a raw level header string to a map of parsed header keys and values.
pub fn parse_raw_level_headers(headers: &str) -> HashMap<GDLevelHeaderKey, GDLevelHeaderValue> {
    let mut headers_kv = headers.split(",");
    let mut map = HashMap::new();
    while let (Some(k), Some(v)) = (headers_kv.next(), headers_kv.next()) {
        let key = GDLevelHeaderKey::parse(k);
        let ptype = if let Some(id) = key.to_id() {
            if let Some(t) = get_level_header_property_type(id as u16) {
                t
            } else {
                // assume int
                GDLevelHeaderValType::Int
            }
        } else {
            // assume int
            GDLevelHeaderValType::Int
        };
        map.insert(
            key,
            match GDLevelHeaderValue::parse(v, ptype) {
                Some(v) => v,
                None => continue,
            },
        );
    }

    map
}

impl GDLevelData {
    /// Serializes the level data to an ecnrypted payload ready to be inserted into the level file. The returned payload is encrypted with [`encrypt_level_str`].
    #[must_use]
    pub fn serialise_to_string(&self) -> String {
        vec_as_str(&encrypt_level_str(&self.serialise_to_raw_string()))
    }

    /// Serialises this object to a raw unencrypted string.
    #[must_use]
    pub fn serialise_to_raw_string(&self) -> String {
        #[cfg(feature = "parallel")]
        let object_data = self
            .objects
            .par_iter()
            .map(GDObject::serialise_to_string)
            .collect::<Vec<String>>()
            .join("");

        #[cfg(not(feature = "parallel"))]
        let object_data: String = {
            // the average size of an object is assumed to be 64 bytes. this value is not optimized in any way.
            // a value too small would cause a lot of resizing that could have been prevented,
            // and a value too large would allocate an unnecessarily large amount of space.
            let mut data = String::with_capacity(self.objects.len() * 64);
            for obj in &self.objects {
                data.push_str(&obj.serialise_to_string());
            }
            data
        };

        let header_str = self.serialise_level_headers();

        let mut unencrypted = String::with_capacity(header_str.len() + object_data.len());
        unencrypted.push_str(&header_str);
        unencrypted.push_str(&object_data);

        unencrypted
    }

    /// Returns the headers of this level in serialized from. Assuming that level headers are properly serialized,
    /// the string returned from this function is readable by GD and ready to be serialized into a GD level.
    pub fn serialise_level_headers(&self) -> String {
        // serialize kS38 first
        let mut out_str = if let Some(ks38) = self.headers.get(&GDLevelHeaderKey::S(38)) {
            format!("kS38,{ks38},")
        } else {
            String::new()
        };

        out_str += &self
            .headers
            .iter()
            .filter_map(|(k, v)| match k {
                GDLevelHeaderKey::S(38) => None, // key was already serialised
                _ => Some(format!("{k},{v}")),
            })
            .collect::<Vec<_>>()
            .join(",");
        out_str += ";";

        out_str
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
        let mut used: [bool; 10_000] = [false; 10_000];
        for object in &self.objects {
            for group in &object.config.groups {
                if let Group::Regular(g) = group
                    && (1..10_000).contains(g)
                {
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

        // this should really be a const map, but that is impossible in the current version of rust.
        // however, the performance cost is negligible since we only generate this list once per search.
        #[cfg(feature = "parallel")]
        let mut groups = self
            .objects
            .par_iter()
            .flat_map_iter(|object| {
                let mut groups = Vec::new();
                for p in GROUP_PROPERTY_IDS {
                    if let Some(val) = object.get_property(*p) {
                        match val {
                            GDValue::Group(g) => groups.push(g),
                            GDValue::GroupList(gs) => groups.extend(gs.iter().copied()),
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
                        GDValue::Group(g) => groups.push(g),
                        GDValue::GroupList(gs) => groups.extend(gs.iter()),
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
    pub fn parse<T: AsRef<str>>(raw_data: T) -> Option<Self> {
        let raw_data = raw_data.as_ref();
        // parse level data
        let raw_data = decompress(raw_data.as_bytes().to_vec()).ok()?;
        let decrypted = std::str::from_utf8(&raw_data[..]).ok()?;
        let split: Vec<&str> = decrypted.split(';').collect();

        // level start string
        let headers = split.first().unwrap_or(&"").to_string();
        let level_headers = parse_raw_level_headers(&headers);

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

        Some(Self {
            headers: level_headers,
            objects,
        })
    }
}

/// Parses a string of raw object strings separated by semicolons to a vector of [`GDObject`]s.
pub fn parse_objects(s: &str) -> Vec<GDObject> {
    #[cfg(feature = "parallel")]
    let objects = {
        let split = s.split(";").collect::<Vec<_>>();

        split
            .par_iter()
            .filter(|obj| obj.len() > 1)
            .map(GDObject::parse_str)
            .collect()
    };

    #[cfg(not(feature = "parallel"))]
    let objects = s
        .split(";")
        .into_iter()
        .filter(|obj| obj.len() > 1)
        .map(GDObject::parse_str)
        .collect();

    objects
}

impl GDLevelHeaderValue {
    /// Parses an input string with a given type to this object
    pub fn parse(val: &str, ptype: GDLevelHeaderValType) -> Option<Self> {
        match ptype {
            GDLevelHeaderValType::Int => Some(Self::Int(val.parse::<i32>().ok()?)),
            GDLevelHeaderValType::Float => Some(Self::Float(val.parse::<f32>().ok()?)),
            GDLevelHeaderValType::Bool => Some(Self::Bool(val.parse::<i32>().ok()? != 0)),
            GDLevelHeaderValType::Gamemode => Some(Self::Gamemode(
                Gamemode::try_from(val.parse::<i32>().ok()?).ok()?,
            )),
            GDLevelHeaderValType::Speed => {
                Some(Self::Speed(Speed::try_from(val.parse::<i32>().ok()?).ok()?))
            }
            GDLevelHeaderValType::ColourString => Some(Self::ColourString({
                // there's usually 14 segments
                let split = val.split("|").collect::<Vec<_>>();
                split
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, segment)| {
                        // this avoids parsing the last element of the split which is an empty string
                        if idx != split.len() - 1 {
                            ColourString::parse(segment)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })),
            GDLevelHeaderValType::GuidelineString => {
                Some(Self::GuidelineString(GuidelineString::parse(val)?))
            }
        }
    }
}

impl GuidelineColour {
    /// Determines the colour of the guideline based on the numerical colour value
    pub fn from_f32(f: f32) -> Self {
        if f == 0.9 {
            Self::Yellow
        } else if f == 1.0 {
            Self::Green
        } else if f == 0.0 || f >= 0.8 {
            Self::Orange
        } else {
            Self::Transparent
        }
    }

    /// Converts this object to its float form.
    /// Note that it may not equal the orignal float since multiple float values may fall under
    /// one variant of this enum.
    pub fn to_float(&self) -> f32 {
        match self {
            Self::Green => 1.0,
            Self::Yellow => 0.9,
            Self::Orange => 0.0,
            Self::Transparent => -1.0, // can be anything
        }
    }
}

impl GuidelineString {
    /// Parses an input string to this object
    pub fn parse(s: &str) -> Option<Self> {
        let guidelines = s
            .split(".")
            .filter_map(|g| {
                let mut split_iter = g.split('~');
                Some((
                    split_iter.next()?.to_owned(),
                    GuidelineColour::from_f32(split_iter.next()?.parse::<f32>().ok()?),
                ))
            })
            .collect::<Vec<_>>();
        Some(Self { guidelines })
    }

    /// Serialises this object to a string
    pub fn to_string(&self) -> String {
        self.guidelines
            .iter()
            .map(|(t, c)| format!("{t}~{}", c.to_float()))
            .collect::<Vec<String>>()
            .join(".")
    }
}

// this macro is used only for parsing in ColourString

macro_rules! parse {
    ($v:expr => $t:ty) => {{
        match $v.parse::<$t>() {
            Ok(v) => v,
            Err(_) => return None,
        }
    }};

    ($v:expr) => {{
        match $v.parse::<i32>() {
            Ok(v) => v != 0,
            Err(_) => return None,
        }
    }};
}

impl ColourString {
    /// Parses a colour string segment into this object
    pub fn parse(s: &str) -> Option<Self> {
        let mut kv_iter = s.split("_");
        let mut new = Self::default();

        while let (Some(k), Some(v)) = (kv_iter.next(), kv_iter.next()) {
            let idx = match k.parse::<i32>() {
                Ok(i) => i,
                Err(_) => return None,
            };

            match idx {
                1 => new.from.red = parse!(v => i32) as u8,
                2 => new.from.green = parse!(v => i32) as u8,
                3 => new.from.blue = parse!(v => i32) as u8,
                4 => {
                    new.player_colour = match v.parse::<i32>() {
                        Ok(v) => match v {
                            1 => PlayerColour::First,
                            2 => PlayerColour::Second,
                            _ => PlayerColour::None,
                        },
                        Err(_) => return None,
                    }
                }
                5 => new.blending = parse!(v),
                6 => new.colour_ch_idx = parse!(v => i32),
                7 => new.from_opacity = parse!(v => f32),
                8 => new.opacity_toggled = parse!(v),
                9 => new.inherited_col_ch_idx = parse!(v => i32),
                10 => {
                    new.copied_hsv = match HSVColour::parse(v) {
                        Some(v) => Some(v),
                        None => return None,
                    }
                }
                11 => new.to.red = parse!(v => i32) as u8,
                12 => new.to.green = parse!(v => i32) as u8,
                13 => new.to.blue = parse!(v => i32) as u8,
                14 => new.deltatime = parse!(v => f32),
                15 => new.to_opacity = parse!(v => f32),
                16 => new.duration = parse!(v => f32),
                17 => new.copy_opacity = parse!(v),
                18 => new.unknown_property18 = parse!(v) as bool,
                _ => {}
            }
        }

        Some(new)
    }

    /// Serialises this object to a String.
    pub fn to_string(&self) -> String {
        // properties 1, 2, 3, 11, 12, 13, 18 are always present

        // casting to f32 is essential to keep all properties as one type
        // list: (id, value, is always present)
        let properties = &[
            (1, self.from.red as f32, true),
            (2, self.from.green as f32, true),
            (3, self.from.blue as f32, true),
            (4, self.player_colour as i32 as f32, false),
            (5, self.blending as i32 as f32, false),
            (6, self.colour_ch_idx as f32, false),
            (7, self.from_opacity, false),
            (8, self.opacity_toggled as i32 as f32, true),
            (9, self.inherited_col_ch_idx as f32, false),
            /* Serialise property 10 later */
            (11, self.to.red as f32, true),
            (12, self.to.green as f32, true),
            (13, self.to.blue as f32, true),
            (14, self.deltatime, false),
            (15, self.to_opacity, false),
            (16, self.duration, false),
            (17, self.copy_opacity as i32 as f32, false),
            (18, self.unknown_property18 as i32 as f32, true),
        ];

        let mut i_buf = itoa::Buffer::new();
        let mut d_buf = dtoa::Buffer::new();

        let mut str_buf = String::with_capacity(64);
        for (idx, val, omnipresent) in properties {
            if !omnipresent && *val == 0.0 {
                // only if the value is empty and not required
                continue;
            }
            let _ = write!(
                str_buf,
                "{idx}_{}_",
                if val.fract() == 0.0 {
                    // is an int; likely not an f32
                    i_buf.format(*val as i32)
                } else {
                    d_buf.format(*val)
                }
            );
        }

        // serialise property 10
        if let Some(ref hsv) = self.copied_hsv {
            let _ = write!(str_buf, "10_{}", hsv);
        }

        // remove trailing _ (that may mess with the kv pairs)
        if str_buf.ends_with('_') {
            str_buf.pop();
        }

        str_buf
    }
}

impl Display for GDLevelHeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool(b) => {
                    if *b { "1".to_owned() } else { "0".to_owned() }
                }
                Self::Float(f) => f.to_string(),
                Self::Int(i) => i.to_string(),
                Self::Speed(s) => (*s as i32).to_string(),
                Self::Gamemode(g) => (*g as i32).to_string(),
                Self::ColourString(c) => c
                    .iter()
                    .map(|cl| format!("{}|", cl.to_string()))
                    .collect::<String>(),
                Self::GuidelineString(g) => g.to_string(),
            },
        )
    }
}

/// Enum for keys in a level header dictionary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GDLevelHeaderKey {
    /// kSxx key
    S(u16),
    /// kAxx key
    A(u16),
    /// Unrecognized value (unaltered)
    Unknown(String),
}

impl GDLevelHeaderKey {
    /// Parses a raw header identifier to a header. This function checks for these three forms:
    ///     - `kAxx` where `xx` is an integer: becomes [`Self::A`]
    ///     - `kSxx` where `xx` is an integer: becomes [`Self::S`]
    ///     - anything else: becomes [`Self::Unknown`]
    pub fn parse(s: &str) -> Self {
        match s {
            a if a.starts_with("kA") && s.len() > 2 && a[2..].parse::<u16>().is_ok() => {
                Self::A(a[2..].parse::<u16>().unwrap())
            }
            s if s.starts_with("kS") && s.len() > 2 && s[2..].parse::<u16>().is_ok() => {
                Self::S(s[2..].parse::<u16>().unwrap())
            }
            unknown => Self::Unknown(unknown.to_string()),
        }
    }

    /// Returns the ID of this key that corresponds to this key's value type. This ID is used to look up this header key's type in [`crate::cclocallevels::properties::LEVEL_HEADER_PROPERTIES`]
    pub fn to_id(&self) -> Option<u16> {
        match self {
            A(n) => Some(*n),
            S(n) => Some(*n + LEVEL_HEADER_PROP_ID_OFFSET),
            _ => None,
        }
    }

    /// Converts an integer ID to this object. All results returned by this function are guaranteed to return the input value when `to_id` is called.
    ///
    /// Please use values defined in [`crate::cclocallevels::gdobj::ids::level_header`] to create instances of this object.
    pub fn from_id(id: u16) -> Self {
        if id > LEVEL_HEADER_PROP_ID_OFFSET {
            Self::S(id - LEVEL_HEADER_PROP_ID_OFFSET)
        } else {
            Self::A(id)
        }
    }
}

impl Display for GDLevelHeaderKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(n) => write!(f, "kA{n}"),
            Self::S(n) => write!(f, "kS{n}"),
            Self::Unknown(n) => write!(f, "{n}"),
        }
    }
}
