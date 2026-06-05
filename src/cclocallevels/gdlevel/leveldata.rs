//! This module conatins methods and structs for operations with individual levels

use std::{
    collections::{BTreeSet, HashSet},
    fmt::{Display, Write},
};

use crate::{
    cclocallevels::{
        gdobj::{
            GDObject,
            structs::{Colour, Gamemode, HSVColour, Speed},
            structs::{GDObjPropType, GDValue, Group},
        },
        properties::{PROPERTY_TABLE, get_level_header_property_type},
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
pub struct GDEncryptedLevelData {
    /// Raw level data
    pub data: String,
}

/// This struct contains the objects of a level and its headers
#[derive(Clone, Debug, PartialEq)]
pub struct GDLevelData {
    /// Level header string
    pub headers: GDLevelHeader,
    /// Level objects
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

/// Contains the properties of the level header string.
#[derive(Clone, Debug, PartialEq)]
pub struct GDLevelHeader {
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

/// Descriptor struct for the guildeine string
#[derive(Clone, Debug, PartialEq)]
pub struct GuidelineString {
    /// The guidelines themselves as (time, colour) tuples
    pub guidelines: Vec<(f32, GuidelineColour)>,
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
pub enum HeaderValueType {
    Int,
    Float,
    Bool,
    Gamemode,
    Speed,
    GuidelineString,
    ColourString,
}

impl GDLevelData {
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
        let raw_data = decompress(raw_data.as_bytes().to_vec()).ok()?;
        let decrypted = std::str::from_utf8(&raw_data[..]).ok()?;
        let split = decrypted.split(";").collect::<Vec<&str>>();

        // level start string
        let headers = split[0].to_string();
        let level_headers = GDLevelHeader::parse(&headers)?;
        let mut objects = Vec::with_capacity(split.len() - 1);

        for object in &split[1..] {
            if object.len() > 1 {
                objects.push(GDObject::parse_str(object));
            }
        }

        Some(GDLevelData {
            headers: level_headers,
            objects,
        })
    }
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
            HeaderValueType::ColourString => Some(Self::ColourString({
                // there's usually 14 segments
                let mut segments = Vec::with_capacity(14);
                for segment in val.split("|").into_iter() {
                    segments.push(ColourString::parse(segment)?)
                }
                segments
            })),
            HeaderValueType::GuidelineString => {
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
        let mut guidelines = vec![];
        for g in s.split(".") {
            let mut split_iter = g.split('~');
            guidelines.push((
                split_iter.next()?.parse::<f32>().ok()?,
                GuidelineColour::from_f32(split_iter.next()?.parse::<f32>().ok()?),
            ));
        }

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
                18 => new.unknown_property18 = parse!(v),
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
            (8, self.opacity_toggled as i32 as f32, false),
            (9, self.inherited_col_ch_idx as f32, false),
            /* Serialise property 10 later */
            (11, self.to.red as f32, true),
            (12, self.to.green as f32, true),
            (13, self.to.blue as f32, true),
            (14, self.deltatime, false),
            (15, self.to_opacity, false),
            (16, self.duration, false),
            (17, self.copy_opacity as i32 as f32, false),
            (18, self.unknown_property18 as i32 as f32, false),
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
                Self::ColourString(c) => c
                    .iter()
                    .map(ColourString::to_string)
                    .collect::<Vec<_>>()
                    .join("|"),
                Self::GuidelineString(g) => g.to_string(),
            },
        )
    }
}

impl Display for GDLevelHeader {
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

impl GDLevelHeader {
    /// Parses the input string to this object
    pub fn parse(s: &str) -> Option<Self> {
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
