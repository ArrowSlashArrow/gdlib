//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::{collections::{BTreeMap, HashMap}, fmt::{Debug, Display, Write}};
use serde_json::{json, Value};
use internment::Intern;

use crate::{gdobj::triggers::ColourChannel, utils::properties_from_json};

pub mod triggers;
pub mod misc;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum GDObjPropType {
    Int,
    Float,
    Text,
    Bool,
    Group,
    Item,
    Easing,
    ColourChannel,
    Unknown
}

#[repr(i32)]
#[derive(Debug, Clone, PartialEq)]
pub enum ZLayer {
    B5 = -5,
    B4 = -3,
    B3 = -1,
    B2 = 1,
    B1 = 3,
    T1 = 5,
    T2 = 7,
    T3 = 9,
    T4 = 11
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct GDObjProperty {
    pub name: &'static str,
    pub desc: &'static str,
    pub arg_type: GDObjPropType
}

// TODO: fill in all the properties
// btw, if anyone has a better idea to represent property descriptors (they differ from object to object), lmk
/// Names of properties (INCOMPLETE): 
/// (property, name)
pub const OBJECT_PROPERTIES: &[GDObjProperty] = &[
    GDObjProperty{name: "1", desc: "object ID", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "2", desc: "x pos", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "3", desc: "y pos", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "4", desc: "is flipped horizontally?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "5", desc: "is flipped vertically?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "6", desc: "rotation", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "7", desc: "Red", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "8", desc: "Green", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "9", desc: "Blue", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "10", desc: "Fade time / chance to trigger group 1", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "11", desc: "Touch triggerable", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "15", desc: "Using player colour 1", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "16", desc: "Using player colour 2", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "17", desc: "Blending enabled", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "20", desc: "Editor layer 1", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "21", desc: "Object colour", arg_type: GDObjPropType::ColourChannel},
    GDObjProperty{name: "23", desc: "Colour channel", arg_type: GDObjPropType::ColourChannel},
    GDObjProperty{name: "24", desc: "Z layer", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "25", desc: "Z order", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "28", desc: "Move units x", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "29", desc: "Move units y", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "30", desc: "Move easing", arg_type: GDObjPropType::Easing},
    GDObjProperty{name: "31", desc: "Base64-encoded text", arg_type: GDObjPropType::Text},
    GDObjProperty{name: "34", desc: "Is group parent?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "35", desc: "Opacity", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "36", desc: "Is active trigger?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "45", desc: "Pulse fade in time", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "46", desc: "Pulse hold time", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "47", desc: "Pulse fade out time", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "49", desc: "Copy colour specs", arg_type: GDObjPropType::Text},
    GDObjProperty{name: "50", desc: "Copy colour from channel", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "51", desc: "Target group/item/channel", arg_type: GDObjPropType::Group},
    GDObjProperty{name: "56", desc: "Activate group", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "57", desc: "Groups", arg_type: GDObjPropType::Group},
    GDObjProperty{name: "58", desc: "Follow player's x movement", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "59", desc: "Follow player's y movement", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "60", desc: "Copy opacity", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "61", desc: "Editor layer 2", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "62", desc: "Spawn triggerable", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "64", desc: "Don't fade", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "67", desc: "Don't enter", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "71", desc: "Target group 2", arg_type: GDObjPropType::Group},
    GDObjProperty{name: "75", desc: "Shake strength", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "80", desc: "Group/item 1", arg_type: GDObjPropType::Item},
    GDObjProperty{name: "84", desc: "Shake interval", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "85", desc: "Easing rate", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "86", desc: "Exclusive pulse mode", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "87", desc: "Multitriggerable", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "94", desc: "Dynamic block?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "95", desc: "Group/item 2", arg_type: GDObjPropType::Item},
    GDObjProperty{name: "96", desc: "No glow", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "99", desc: "Multi activate", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "100", desc: "Target move mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "101", desc: "Target move mode axis lock", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "103", desc: "Is high detail?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "116", desc: "No object effects", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "117", desc: "Center object effect", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "120", desc: "Timewarp amount", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "121", desc: "No touch?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "128", desc: "X scale", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "129", desc: "Y scale", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "134", desc: "Passable", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "135", desc: "Hidden", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "136", desc: "Non-stick X", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "137", desc: "Is ice block?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "138", desc: "Controlling player 1", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "141", desc: "Follow camera's x movement", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "142", desc: "Follow camera's y movement", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "143", desc: "X movement multiplier", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "144", desc: "Y movement multiplier", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "148", desc: "Gravity", arg_type: GDObjPropType::Float},
    // GDObjProperty{name: "155", desc: "Mysterious property 155", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "193", desc: "Grippy slope?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "200", desc: "Controlling player 2", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "201", desc: "Controlling target player", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "210", desc: "No legacy HSV", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "217", desc: "Enter/Exit transition config", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "274", desc: "Parent groups", arg_type: GDObjPropType::Group},
    GDObjProperty{name: "279", desc: "Is area parent?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "284", desc: "Single player touch", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "289", desc: "Non-stick Y", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "343", desc: "Enter effect channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "344", desc: "Target transition channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "356", desc: "Scale stick", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "369", desc: "Center effect", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "371", desc: "Camera zoom", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "372", desc: "No audio scale", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "392", desc: "Song ID", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "393", desc: "Small step", arg_type: GDObjPropType::Bool}, // this is a UI-only property. interally, move distances are stored the same regardless.
    GDObjProperty{name: "394", desc: "Directional move mode", arg_type: GDObjPropType::Bool} ,
    GDObjProperty{name: "395", desc: "Center group id", arg_type: GDObjPropType::Group},
    GDObjProperty{name: "397", desc: "Dynamic move", arg_type: GDObjPropType::Bool}, 
    GDObjProperty{name: "399", desc: "Prep?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "400", desc: "Load Prep?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "404", desc: "Song speed", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "406", desc: "Song volume", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "408", desc: "Start offset in ms", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "409", desc: "Fade in time in ms", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "410", desc: "End offset in ms", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "411", desc: "Fade out time in ms", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "413", desc: "Loop song?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "432", desc: "Song channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "445", desc: "Claim touch?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "446", desc: "Object material", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "460", desc: "No end effects?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "461", desc: "Instant end?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "467", desc: "No end sound effects?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "472", desc: "Stop time counter?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "473", desc: "Target time for event", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "475", desc: "Multiactivatable time event", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "476", desc: "First item type", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "477", desc: "Second item type", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "479", desc: "Modifier", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "480", desc: "Left operator", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "481", desc: "Right operator", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "482", desc: "Compare operator", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "483", desc: "Second modifier", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "484", desc: "Tolerance", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "485", desc: "Left round mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "486", desc: "Right round mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "491", desc: "Set persistent item", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "492", desc: "Target all persistent items", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "493", desc: "Reset item to 0", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "494", desc: "Timer", arg_type: GDObjPropType::Item},
    GDObjProperty{name: "495", desc: "Extra sticky", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "496", desc: "Don't boost Y?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "504", desc: "Spawn only", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "506", desc: "Camera guide preview opacity", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "507", desc: "No particles", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "509", desc: "Don't boost X?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "511", desc: "Has extended collision", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "540", desc: "Stop player jump", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "541", desc: "Stop player movement", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "542", desc: "Stop player rotation", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "543", desc: "Stop player sliding", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "544", desc: "Silent move", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "547", desc: "X offset of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "548", desc: "Y offset of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "549", desc: "X offset variation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "550", desc: "Y offset variation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "551", desc: "Match rotation of spawned particles?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "552", desc: "Rotation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "553", desc: "Rotation variation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty{name: "554", desc: "Scale of spawned particles", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "555", desc: "Scale variation of spawned particles", arg_type: GDObjPropType::Float},
    GDObjProperty{name: "578", desc: "Left sign mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "579", desc: "Right sign mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "595", desc: "Don't stop song on death", arg_type: GDObjPropType::Bool},
    // these are all startpos properties:
    // also the reason i can't use u16 for the property name
    GDObjProperty{name: "kA4", desc: "Starting speed", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "kA2", desc: "Starting gamemode", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "kA3", desc: "Starting in mini mode?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "kA8", desc: "Starting in dual mode?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "kA21", desc: "Is disabled?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "kA28", desc: "Starting in mirror mode?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "kA29", desc: "Rotate gameplay?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "kA20", desc: "Reverse gameplay?", arg_type: GDObjPropType::Bool},
    GDObjProperty{name: "kA19", desc: "Target order", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "kA26", desc: "Target channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty{name: "kA35", desc: "Reset camera?", arg_type: GDObjPropType::Bool}
];

impl GDObjProperty {
    pub fn from_name(name: String) -> Self {
        match OBJECT_PROPERTIES.iter().find(|prop| prop.name == name) {
            Some(property) => property.clone(),
            None => {
                // put the string into heap to align with types, use internment for caching to not leak memory 
                let leaked: &'static str = Box::leak(name.into_boxed_str());
                let interned = Intern::new(leaked);
                GDObjProperty { name: *interned, desc: *interned, arg_type: GDObjPropType::Unknown }
            }
        }
    }
}

/// Map of all object ids to names: (id, name)
pub const OBJ_NAMES: &[(i32, &str)] = &[
    (1, "Default block"),
    (8, "Spike"),
    (39, "Small spike"),
    (22, "No block transition object"),
    (23, "Blocks from top transition object"),
    (24, "Blocks from bottom transition object"),
    (25, "Blocks from left transition object"),
    (26, "Blocks from right transition object"),
    (27, "Scale in transition object"),
    (28, "Scale out transition object"),
    (32, "Enable player trail"),
    (33, "Disable player trail"),
    (55, "Random direction transition object"),
    (56, "Away to left transition object"),
    (57, "Away to right transition object"),
    (58, "Away from middle transition object"),
    (59, "Away to middle transition object"),
    (31, "Start pos"),
    (899, "Colour trigger"),
    (901, "Move trigger"),
    (914, "Text object"),
    (1006, "Pulse trigger"),
    (1007, "Alpha trigger"),
    (1049, "Toggle trigger"),
    (1268, "Spawn trigger"),
    (1520, "Shake trigger"),
    (1615, "Counter"),
    (1616, "Stop trigger"),
    (1812, "On death trigger"),
    (1815, "Collision trigger"),
    (1816, "Collision block"),
    (1818, "BG effect on"),
    (1819, "BG effect off"),
    (1912, "Random trigger"),
    (1913, "Camera zoom trigger"),
    (1915, "Don't fade + don't enter transition object"),
    (1917, "Reverse gameplay"),
    (1932, "Gravity trigger"),
    (1934, "Song trigger"),
    (1935, "Time warp trigger"),
    (2016, "Camera guide"),
    (2066, "Gravity trigger"),
    (3600, "End trigger"),
    (3606, "BG speed config"),
    (3608, "Spawn particle trigger"),
    (3612, "MG speed config"),
    (3615, "Time event trigger"),
    (3617, "Time control trigger"),
    (3618, "Reset group trigger"),
    (3619, "Item edit trigger"),
    (3620, "Item compare trigger"),
    (3640, "Collision state block"),
    (3641, "Persistent item trigger"),
    (3643, "Toggle block"),
    (3662, "Link visible trigger"),
];

// TODO: UPDATE THIS!!!!!
pub const TRIGGER_OBJ_IDS: &[i32] = &[
    22, 23, 24, 25, 26, 27, 28, 32, 33, 55, 56, 57, 58, 59, 31,
    899, 901, 914, 1006, 1007, 1049, 1268, 1520, 1615, 1616, 1812, 
    1815, 1816, 1818, 1819, 1912, 1913, 1915, 1917, 1932, 1934, 1935, 
    2016, 2066, 3600, 3606, 3612, 3615, 3617, 3618, 3619, 3620, 3640, 
    3641, 3643, 3662, 
];

/// Container for GD Object properties.
/// * `id`: The object's ID.
/// * `config`: General properties like position and scale.
/// * `properties`: Object-specific properties like target group for a move trigger
#[derive(Clone, PartialEq)]
pub struct GDObject {
    pub id: i32,
    pub config: GDObjConfig,
    pub properties: GDObjProperties
}

#[derive(Debug, Clone, PartialEq)]
pub struct GDObjProperties {
    pub properties: HashMap<GDObjProperty, Value>
}

impl GDObjProperties {
    /// Inititalises this class with an empty properties table.
    pub fn new() -> Self {
        GDObjProperties { properties: HashMap::new() }
    }

    /// Converts this properties table to a string
    pub fn to_string(&mut self) -> String {
        let mut sorted: Vec<_> = self.properties.iter().collect();
        sorted.sort_by_key(|&(k, _)| k);

        let mut out_str = String::with_capacity(sorted.len() * 16);

        for (k, v) in sorted.iter() {
            match v {
                Value::Number(n) => write!(&mut out_str, ",{},{}", k.name, n),
                Value::Bool(b) => write!(&mut out_str, ",{},{}", k.name, if *b { "1" } else { "0" }),
                Value::String(s) => write!(&mut out_str, ",{},{}", k.name, s),
                _ => write!(&mut out_str, ",{},{}", k.name, v.to_string())
            }.unwrap()
        };

        out_str.remove(0);
        return out_str
    }

    /// Constructor for this class from a [`serde_json::value::Value`]
    pub fn from_json(json: Value) -> Self {
        GDObjProperties {
            properties: properties_from_json(json)
        }
    }

    /// Gets a property from its properties table.
    pub fn get_property<T: Into<String>>(&self, p: T) -> Option<Value> {
        return self.properties.get(&GDObjProperty::from_name(p.into())).cloned()
    }
}

impl Display for GDObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group_str = match self.config.groups.len() > 0 {
            true => {
                &format!(" with groups: {}",
                    self.config.groups.iter().map(|g| format!("{g}")).collect::<Vec<String>>().join(", ")
                )
            }
            false => ""
        };

        let mut trigger_conf_str = String::new();
        if self.config.trigger_cfg.spawnable || self.config.trigger_cfg.touchable {
            if self.config.trigger_cfg.multitriggerable {
                trigger_conf_str += "Multi"
            }
            if self.config.trigger_cfg.touchable {
                trigger_conf_str += "touchable "
            } else if self.config.trigger_cfg.spawnable {
                trigger_conf_str += "spawnable "
            }
        }

        write!(f, "{trigger_conf_str}{} @ ({}, {}) scaled to ({}, {}){} angled to {}°", 
            self.name(), 
            self.config.pos.0, 
            self.config.pos.1, 
            self.config.scale.0, 
            self.config.scale.1,
            group_str,
            self.config.angle
        )
    }
}

impl Debug for GDObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut property_str = String::new();
        let sorted: BTreeMap<_, _> = self.properties.properties.iter().collect();
        for (property, value) in sorted.iter() {
            property_str += &format!("\n    - {}: {value}", property.desc);
        }  

        write!(f, "{} with properties:{property_str}", 
            <Self as ToString>::to_string(self),
        )
    }
}

impl GDObject {
    /// Parses raw object string to GDObject
    /// 
    /// Example:
    /// ```
    /// use gdlib::gdobj::{GDObject, GDObjConfig, GDObjProperties};
    /// 
    /// let obj = GDObject::parse_str("1,1,2,0,3,0;");
    /// assert_eq!(obj, GDObject::new(1, GDObjConfig::default(), GDObjProperties::new()));
    /// ```
    pub fn parse_str(s: &str) -> GDObject {
        let mut obj = GDObject { 
            id: 1,
            config: GDObjConfig::default(), 
            properties: GDObjProperties::new(),
        };

        let mut vals = json!({});
        let map = vals.as_object_mut().unwrap();

        let mut iter = s.trim_end_matches(';').split(",");
        while let (Some(idx), Some(val)) = (iter.next(), iter.next()) {
            match idx {
                "1"   => obj.id = val.parse().unwrap_or(0),
                "2"   => obj.config.pos.0 = val.parse().unwrap_or(0.0),
                "3"   => obj.config.pos.1 = val.parse().unwrap_or(0.0),
                "6"   => obj.config.angle = val.parse().unwrap_or(0.0),
                "11"  => obj.config.trigger_cfg.touchable = val.parse().unwrap_or(false),
                "62"  => obj.config.trigger_cfg.spawnable = val.parse().unwrap_or(false),
                "87"  => obj.config.trigger_cfg.multitriggerable = val.parse().unwrap_or(false),
                "57"  => obj.config.groups = val.trim_matches('"').split(".").filter_map(|g| g.parse::<u16>().ok()).collect(),
                "128" => obj.config.scale.0 = val.parse().unwrap_or(1.0),
                "129" => obj.config.scale.1 = val.parse().unwrap_or(1.0),
                _ => {
                    match idx.parse::<u16>() {
                        Ok(n) => map.insert(n.to_string(), Value::from(val.to_string())),
                        Err(_) => match idx[2..].parse::<u16>() {
                            Ok(n) => map.insert(n.to_string(), Value::from(val.to_string())),
                            Err(_) => map.insert("65536".to_string(), Value::from(val.to_string()))
                        } 
                    };
                }
            }
        }

        obj.properties = GDObjProperties::from_json(vals);

        return obj
    }

    /// Returns this object as a property string
    /// 
    /// Example:
    /// ```
    /// use gdlib::gdobj::{GDObject, GDObjConfig, GDObjProperties};
    /// 
    /// let object_str = GDObject::new(1, GDObjConfig::default(), GDObjProperties::new()).to_string();
    /// assert_eq!(object_str, "1,1,155,1,2,0.0,3,0.0,64,1,67,1;");
    /// ```
    pub fn to_string(&self) -> String {
        let mut combined_properties = self.properties.clone();
        combined_properties.properties.extend(self.config.as_properties());

        let raw_str = format!("1,{},{}", self.id, combined_properties.to_string());
        return raw_str.replace("\"", "") + ";";
    }

    pub fn name(&self) -> String {
        OBJ_NAMES.iter().find(|&o| o.0 == self.id)
            .unwrap_or(&(0, format!("Object {}", self.id).as_str())).1.to_string()
    }

    /// Creates a new GDObject from ID, config, and extra proerties
    pub fn new(id: i32, config: GDObjConfig, properties: GDObjProperties) -> Self {
        GDObject {
            id, config, properties
        }
    }

    pub fn get_property(&self, p: &str) -> Option<Value> {
        match p {
            "1" => Some(Value::from(self.id)),
            "2" => Some(Value::from(self.config.pos.0)),
            "3" => Some(Value::from(self.config.pos.1)),
            "6" => Some(Value::from(self.config.angle)),
            "11" => Some(Value::from(self.config.trigger_cfg.touchable)),
            "57" => Some(Value::from(self.config.groups.clone())),
            "62" => Some(Value::from(self.config.trigger_cfg.spawnable)),
            "87" => Some(Value::from(self.config.trigger_cfg.multitriggerable)),
            "128" => Some(Value::from(self.config.scale.0)),
            "129" => Some(Value::from(self.config.angle)),
            _ => self.properties.get_property(p)
        }
    }
}

/// Trigger config, used for defining general properties of a trigger object:
/// * is touch triggerable?
/// * is spawn triggerable?
/// * is multitriggerable?
#[derive(Clone, Debug, PartialEq)]
pub struct TriggerConfig {
    pub touchable: bool,
    pub spawnable: bool,
    pub multitriggerable: bool
}

/// Object config, used for defining general properties of an object:
/// * position
/// * scale
/// * rotation angle
/// * groups
/// * trigger_cfg
#[derive(Clone, Debug, PartialEq)]
pub struct GDObjConfig {
    pub pos: (f32, f32),
    pub scale: (f32, f32),
    pub angle: f32,
    pub groups: Vec<u16>,
    pub trigger_cfg: TriggerConfig,
    pub z_order: i32,
    pub z_layer: ZLayer,
    pub editor_layers: (i32, i32),
    pub colour_channel: ColourChannel,
    pub enter_effect_channel: i32,
    pub material_id: i32,
    pub attributes: GDObjAttributes
} 

impl GDObjConfig {
    /// Constructor with default properties:
    /// * position: 0, 0
    /// * scale: 1.0, 1.0
    /// * angle: 0.0,
    /// * groups: none
    /// * not touch triggerable
    /// * not spawn triggerable
    /// * not multi triggerable
    pub fn default() -> Self {
        GDObjConfig { 
            pos: (0.0, 0.0), 
            scale: (1.0, 1.0),
            angle: 0.0, 
            groups: vec![], 
            trigger_cfg: TriggerConfig { 
                touchable: false, 
                spawnable: false, 
                multitriggerable: false 
            },
            z_layer: ZLayer::T1,
            z_order: 0,
            editor_layers: (0, 0),
            colour_channel: ColourChannel::Object,
            enter_effect_channel: 0,
            material_id: 0,
            attributes: GDObjAttributes::new()
        }
    }

    /// Alias for default
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts this config to a properties hashmap
    pub fn as_properties(&self) -> HashMap<GDObjProperty, Value> {
        let mut properties = json!({
            "2": self.pos.0,
            "3": self.pos.1,
            "64": 1,
            "67": 1,
            "155": 1,
            "6": self.angle,
            "128": self.scale.0,
            "129": self.scale.1,
            "11": self.trigger_cfg.touchable,
            "62": self.trigger_cfg.spawnable,
            "87": self.trigger_cfg.multitriggerable,
            "20": self.editor_layers.0,
            "61": self.editor_layers.1,
            "21": self.colour_channel.as_i32(),
            "24": self.z_layer.clone() as i32,
            "25": self.z_order,
            "343": self.enter_effect_channel,
            "446": self.material_id,
            // attributes
            "64" : self.attributes.dont_fade,
            "67" : self.attributes.dont_enter,
            "116": self.attributes.no_effects,
            "34" : self.attributes.is_group_parent,
            "279": self.attributes.is_area_parent,
            "509": self.attributes.dont_boost_x,
            "496": self.attributes.dont_boost_y,
            "103": self.attributes.high_detail,
            "121": self.attributes.no_touch,
            "134": self.attributes.passable,
            "135": self.attributes.hidden,
            "136": self.attributes.non_stick_x,
            "289": self.attributes.non_stick_y,
            "495": self.attributes.extra_sticky,
            "511": self.attributes.extended_collision,
            "137": self.attributes.is_ice_block,
            "193": self.attributes.grip_slope,
            "96" : self.attributes.no_glow,
            "507": self.attributes.no_particles,
            "356": self.attributes.scale_stick,
            "372": self.attributes.no_audio_scale,
            "284": self.attributes.single_ptouch,
            "369": self.attributes.center_effect,
            "117": self.attributes.reverse
        });

        if !self.groups.is_empty() && let Some(map) = properties.as_object_mut() {
            map.insert("57".to_owned(), Value::from(
                self.groups.iter().map(|&g| format!("{g}")).collect::<Vec<String>>().join(".")
            ));
        };

        let hashmap = properties.as_object().unwrap().into_iter()
            .map(|(k, v)| (GDObjProperty::from_name(k.clone()), v.clone())).collect();

        return hashmap
    }

    /// Sets groups of this object
    pub fn groups<T: IntoIterator<Item = u16>>(mut self, groups: T) -> Self {
        self.groups = groups.into_iter().collect();
        self
    }
    /// Sets x position of this object
    pub fn x(mut self, x: f32) -> Self {
        self.pos.0 = x;
        self
    }
    /// Sets y position of this object
    pub fn y(mut self, y: f32) -> Self {
        self.pos.1 = y;
        self
    }
    /// Sets x and y position of this object
    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = (x, y);
        self
    }
    /// Sets x scale of this object
    pub fn xscale(mut self, xscale: f32) -> Self {
        self.scale.0 = xscale;
        self
    }
    /// Sets y scale of this object
    pub fn yscale(mut self, yscale: f32) -> Self {
        self.scale.1 = yscale;
        self
    }
    /// Sets x and y scale of this object
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.scale = (x, y);
        self
    }
    /// Sets rotation angle of this object
    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }
    /// Makes this object touch triggerable
    pub fn touchable(mut self, touchable: bool) -> Self {
        self.trigger_cfg.touchable = touchable;
        self
    }
    /// Makes this object spawn triggerable
    pub fn spawnable(mut self, spawnable: bool) -> Self {
        self.trigger_cfg.spawnable = spawnable;
        self
    }
    /// Makes this object multi-triggerable
    pub fn multitrigger(mut self, multi: bool) -> Self {
        self.trigger_cfg.multitriggerable = multi;
        self
    }

    /// Enables `dont_fade` on this object.
    pub fn dont_fade(mut self, toggle: bool) -> Self {
        self.attributes.dont_fade = toggle;
        self
    }

    /// Enables `dont_enter` on this object.
    pub fn dont_enter(mut self, toggle: bool) -> Self {
        self.attributes.dont_enter = toggle;
        self
    }

    /// Enables `no_effects` on this object.
    pub fn no_effects(mut self, toggle: bool) -> Self {
        self.attributes.no_effects = toggle;
        self
    }

    /// Enables `is_group_parent` on this object.
    pub fn is_group_parent(mut self, toggle: bool) -> Self {
        self.attributes.is_group_parent = toggle;
        self
    }

    /// Enables `is_area_parent` on this object.
    pub fn is_area_parent(mut self, toggle: bool) -> Self {
        self.attributes.is_area_parent = toggle;
        self
    }

    /// Enables `dont_boost_x` on this object.
    pub fn dont_boost_x(mut self, toggle: bool) -> Self {
        self.attributes.dont_boost_x = toggle;
        self
    }

    /// Enables `dont_boost_y` on this object.
    pub fn dont_boost_y(mut self, toggle: bool) -> Self {
        self.attributes.dont_boost_y = toggle;
        self
    }

    /// Enables `high_detail` on this object.
    pub fn high_detail(mut self, toggle: bool) -> Self {
        self.attributes.high_detail = toggle;
        self
    }

    /// Enables `no_touch` on this object.
    pub fn no_touch(mut self, toggle: bool) -> Self {
        self.attributes.no_touch = toggle;
        self
    }

    /// Enables `passable` on this object.
    pub fn passable(mut self, toggle: bool) -> Self {
        self.attributes.passable = toggle;
        self
    }

    /// Enables `hidden` on this object.
    pub fn hidden(mut self, toggle: bool) -> Self {
        self.attributes.hidden = toggle;
        self
    }

    /// Enables `non_stick_x` on this object.
    pub fn non_stick_x(mut self, toggle: bool) -> Self {
        self.attributes.non_stick_x = toggle;
        self
    }

    /// Enables `non_stick_y` on this object.
    pub fn non_stick_y(mut self, toggle: bool) -> Self {
        self.attributes.non_stick_y = toggle;
        self
    }

    /// Enables `extra_sticky` on this object.
    pub fn extra_sticky(mut self, toggle: bool) -> Self {
        self.attributes.extra_sticky = toggle;
        self
    }

    /// Enables `extended_collision` on this object.
    pub fn extended_collision(mut self, toggle: bool) -> Self {
        self.attributes.extended_collision = toggle;
        self
    }

    /// Enables `is_ice_block` on this object.
    pub fn is_ice_block(mut self, toggle: bool) -> Self {
        self.attributes.is_ice_block = toggle;
        self
    }

    /// Enables `grip_slope` on this object.
    pub fn grip_slope(mut self, toggle: bool) -> Self {
        self.attributes.grip_slope = toggle;
        self
    }

    /// Enables `no_glow` on this object.
    pub fn no_glow(mut self, toggle: bool) -> Self {
        self.attributes.no_glow = toggle;
        self
    }

    /// Enables `no_particles` on this object.
    pub fn no_particles(mut self, toggle: bool) -> Self {
        self.attributes.no_particles = toggle;
        self
    }

    /// Enables `scale_stick` on this object.
    pub fn scale_stick(mut self, toggle: bool) -> Self {
        self.attributes.scale_stick = toggle;
        self
    }

    /// Enables `no_audio_scale` on this object.
    pub fn no_audio_scale(mut self, toggle: bool) -> Self {
        self.attributes.no_audio_scale = toggle;
        self
    }

    /// Enables `single_ptouch` on this object.
    pub fn single_ptouch(mut self, toggle: bool) -> Self {
        self.attributes.single_ptouch = toggle;
        self
    }

    /// Enables `center_effect` on this object.
    pub fn center_effect(mut self, toggle: bool) -> Self {
        self.attributes.center_effect = toggle;
        self
    }

    /// Enables `reverse` on this object.
    pub fn reverse(mut self, toggle: bool) -> Self {
        self.attributes.reverse = toggle;
        self
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct GDObjAttributes {
    pub dont_fade: bool,
    pub dont_enter: bool,
    pub no_effects: bool,
    pub is_group_parent: bool,
    pub is_area_parent: bool,
    pub dont_boost_x: bool,
    pub dont_boost_y: bool,
    pub high_detail: bool,
    pub no_touch: bool,
    pub passable: bool,
    pub hidden: bool,
    pub non_stick_x: bool,
    pub non_stick_y: bool,
    pub extra_sticky: bool,
    pub extended_collision: bool,
    pub is_ice_block: bool,
    pub grip_slope: bool,
    pub no_glow: bool,
    pub no_particles: bool,
    pub scale_stick: bool,
    pub no_audio_scale: bool,
    pub single_ptouch: bool,
    pub center_effect: bool,
    pub reverse: bool,
}

impl GDObjAttributes {
    pub fn new() -> Self {
        Self { 
            dont_fade: false,
            dont_enter: false,
            no_effects: false,
            is_group_parent: false,
            is_area_parent: false,
            dont_boost_x: false,
            dont_boost_y: false,
            high_detail: false,
            no_touch: false,
            passable: false,
            hidden: false,
            non_stick_x: false,
            non_stick_y: false,
            extra_sticky: false,
            extended_collision: false,
            is_ice_block: false,
            grip_slope: false,
            no_glow: false,
            no_particles: false,
            scale_stick: false,
            no_audio_scale: false,
            center_effect: false,
            single_ptouch: false,
            reverse: false
        }
    }

    /// Alias for `new()`
    pub fn default() -> Self {
        Self::new()
    }
}