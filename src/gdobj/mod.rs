//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::{fmt::{Debug, Display}};
use serde_json::{Value};

pub mod triggers;
pub mod misc;
pub mod ids;

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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct GDObjProperty {
    pub name: u16,
    pub desc: &'static str,
    pub arg_type: GDObjPropType
}

// TODO: fill in all the properties
// btw, if anyone has a better idea to represent property descriptors (they differ from object to object), lmk
/// Names of properties (INCOMPLETE): 
/// (property, name)
pub const OBJECT_PROPERTIES: &[GDObjProperty] = &[
    GDObjProperty { name: 1, desc: "object ID", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 2, desc: "x pos", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 3, desc: "y pos", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 4, desc: "is flipped horizontally?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 5, desc: "is flipped vertically?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 6, desc: "rotation", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 7, desc: "Red", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 8, desc: "Green", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 9, desc: "Blue", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 10, desc: "Fade time / chance to trigger group 1", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 11, desc: "Touch triggerable", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 15, desc: "Using player colour 1", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 16, desc: "Using player colour 2", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 17, desc: "Blending enabled", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 23, desc: "Colour channel", arg_type: GDObjPropType::ColourChannel},
    GDObjProperty { name: 28, desc: "Move units x", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 29, desc: "Move units y", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 30, desc: "Move easing", arg_type: GDObjPropType::Easing},
    GDObjProperty { name: 31, desc: "Base64-encoded text", arg_type: GDObjPropType::Text},
    GDObjProperty { name: 35, desc: "Opacity", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 36, desc: "Is active trigger?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 45, desc: "Pulse fade in time", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 46, desc: "Pulse hold time", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 47, desc: "Pulse fade out time", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 49, desc: "Copy colour specs", arg_type: GDObjPropType::Text},
    GDObjProperty { name: 50, desc: "Copy colour from channel", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 51, desc: "Target group/item/channel", arg_type: GDObjPropType::Group},
    GDObjProperty { name: 56, desc: "Activate group", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 58, desc: "Follow player's x movement", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 59, desc: "Follow player's y movement", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 60, desc: "Copy opacity", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 62, desc: "Spawn triggerable", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 71, desc: "Target group 2", arg_type: GDObjPropType::Group},
    GDObjProperty { name: 75, desc: "Shake strength", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 80, desc: "Group/item 1", arg_type: GDObjPropType::Item},
    GDObjProperty { name: 84, desc: "Shake interval", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 85, desc: "Easing rate", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 86, desc: "Exclusive pulse mode", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 87, desc: "Multitriggerable", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 94, desc: "Dynamic block?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 95, desc: "Group/item 2", arg_type: GDObjPropType::Item},
    GDObjProperty { name: 99, desc: "Multi activate", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 100, desc: "Target move mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 101, desc: "Target move mode axis lock", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 120, desc: "Timewarp amount", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 128, desc: "X scale", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 129, desc: "Y scale", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 138, desc: "Controlling player 1", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 141, desc: "Follow camera's x movement", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 142, desc: "Follow camera's y movement", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 143, desc: "X movement multiplier", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 144, desc: "Y movement multiplier", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 148, desc: "Gravity", arg_type: GDObjPropType::Float},
    // GDObjProperty { name: "155", desc: "Mysterious property 155", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 200, desc: "Controlling player 2", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 201, desc: "Controlling target player", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 210, desc: "No legacy HSV", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 217, desc: "Enter/Exit transition config", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 344, desc: "Target transition channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 371, desc: "Camera zoom", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 392, desc: "Song ID", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 393, desc: "Small step", arg_type: GDObjPropType::Bool}, // this is a UI-only property. interally, move distances are stored the same regardless.
    GDObjProperty { name: 394, desc: "Directional move mode", arg_type: GDObjPropType::Bool} ,
    GDObjProperty { name: 395, desc: "Center group id", arg_type: GDObjPropType::Group},
    GDObjProperty { name: 397, desc: "Dynamic move", arg_type: GDObjPropType::Bool}, 
    GDObjProperty { name: 399, desc: "Prep?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 400, desc: "Load Prep?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 404, desc: "Song speed", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 406, desc: "Song volume", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 408, desc: "Start offset in ms", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 409, desc: "Fade in time in ms", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 410, desc: "End offset in ms", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 411, desc: "Fade out time in ms", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 413, desc: "Loop song?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 432, desc: "Song channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 445, desc: "Claim touch?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 460, desc: "No end effects?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 461, desc: "Instant end?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 467, desc: "No end sound effects?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 472, desc: "Stop time counter?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 473, desc: "Target time for event", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 475, desc: "Multiactivatable time event", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 476, desc: "First item type", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 477, desc: "Second item type", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 479, desc: "Modifier", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 480, desc: "Left operator", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 481, desc: "Right operator", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 482, desc: "Compare operator", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 483, desc: "Second modifier", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 484, desc: "Tolerance", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 485, desc: "Left round mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 486, desc: "Right round mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 491, desc: "Set persistent item", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 492, desc: "Target all persistent items", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 493, desc: "Reset item to 0", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 494, desc: "Timer", arg_type: GDObjPropType::Item},
    GDObjProperty { name: 504, desc: "Spawn only", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 506, desc: "Camera guide preview opacity", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 540, desc: "Stop player jump", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 541, desc: "Stop player movement", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 542, desc: "Stop player rotation", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 543, desc: "Stop player sliding", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 544, desc: "Silent move", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 547, desc: "X offset of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 548, desc: "Y offset of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 549, desc: "X offset variation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 550, desc: "Y offset variation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 551, desc: "Match rotation of spawned particles?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 552, desc: "Rotation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 553, desc: "Rotation variation of spawned particles", arg_type: GDObjPropType::Int},
    GDObjProperty { name: 554, desc: "Scale of spawned particles", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 555, desc: "Scale variation of spawned particles", arg_type: GDObjPropType::Float},
    GDObjProperty { name: 578, desc: "Left sign mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 579, desc: "Right sign mode", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 595, desc: "Don't stop song on death", arg_type: GDObjPropType::Bool},
    // these are all startpos properties:
    // NOTE: startpos properties are NOT actually 100XX, their real values are kAXX
    // i.e. 10004 is actually kA4. this is done so that i can use a u16 instead of a &str for the id.
    GDObjProperty { name: 10004, desc: "Starting speed", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 10002, desc: "Starting gamemode", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 10003, desc: "Starting in mini mode?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 10008, desc: "Starting in dual mode?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 10021, desc: "Is disabled?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 10028, desc: "Starting in mirror mode?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 10029, desc: "Rotate gameplay?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 10020, desc: "Reverse gameplay?", arg_type: GDObjPropType::Bool},
    GDObjProperty { name: 10019, desc: "Target order", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 10026, desc: "Target channel", arg_type: GDObjPropType::Unknown},
    GDObjProperty { name: 10035, desc: "Reset camera?", arg_type: GDObjPropType::Bool}
];

impl GDObjProperty {
    pub fn new(id: u16) -> Self {
        Self {
            name: id,
            desc: "",
            arg_type: GDObjPropType::Unknown
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
    (1932, "Player control trigger"),
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

const DEFAULT_GDOBJPROP: GDObjProperty = GDObjProperty {name: 0, desc: "Unknown property", arg_type: GDObjPropType::Unknown};

fn get_property_by_id(id: u16) -> Option<GDObjProperty> {
    for prop in OBJECT_PROPERTIES.iter() {
        if prop.name == id {
            return Some(prop.clone())
        }
    }
    return None
}

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
    pub properties: Vec<(u16, String)>,
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
        for (property, value) in self.properties.iter() {
            let desc = get_property_by_id(*property).unwrap_or(DEFAULT_GDOBJPROP).desc;
            property_str += &format!("\n    - {desc}: {value}");
        }  

        write!(f, "{} with properties:{property_str}", 
            <Self as ToString>::to_string(self),
        )
    }
}

struct GDObjProperties {}

impl GDObjProperties {
    pub fn from_json(json: Value) -> Vec<(u16, String)> {
        let prop_vec: Vec<(u16, String)> = json.as_object().unwrap().iter().map(|(k, val)|
            match k.parse::<u16>() {
                Ok(n) => (n, val.to_string()),
                Err(_) => match k[2..].parse::<u16>() {
                    Ok(n) => (n, val.to_string()),
                    Err(_) => (65535, val.to_string())
                } 
            }
        ).collect();

        return prop_vec;
    }

    pub fn new() -> Vec<(u16, String)> {
        vec![]
    }
}

impl GDObject {
    /// Parses raw object string to GDObject
    /// 
    /// Example:
    /// ```
    /// use gdlib::gdobj::{GDObject, GDObjConfig};
    /// 
    /// let obj = GDObject::parse_str("1,1,2,0,3,0;");
    /// assert_eq!(obj, GDObject::new(1, GDObjConfig::default(), vec![], vec![]));
    /// ```
    pub fn parse_str(s: &str) -> GDObject {
        let mut obj = GDObject { 
            id: 1,
            config: GDObjConfig { 
                pos: (0.0, 0.0), 
                scale: (1.0, 1.0), 
                angle: 0.0, 
                groups: vec![], 
                trigger_cfg: TriggerConfig { 
                    touchable: false, 
                    spawnable: false, 
                    multitriggerable: false 
                }
            }, 
            properties: vec![],
        };

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
                _ => match idx.parse::<u16>() {
                    Ok(n) => obj.properties.push((n, val.to_string())),
                    Err(_) => match idx[2..].parse::<u16>() {
                        Ok(n) => obj.properties.push((n, val.to_string())),
                        Err(_) => obj.properties.push((65535, val.to_string()))
                    } 
                }
            }
        }

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
        let mut properties_str = String::new();
        for (id, val) in self.properties.iter() {
            properties_str += &format!(",{},{val}", match *id >= 10000 {
                true => "kA".to_owned() + &(id - 10000).to_string(),
                false => id.to_string()
            })
        }

        let raw_str = format!("1,{}{}{properties_str};", self.id, self.config.to_string());
        return raw_str.replace("\"", "");
    }

    pub fn name(&self) -> String {
        OBJ_NAMES.iter().find(|&o| o.0 == self.id)
            .unwrap_or(&(0, format!("Object {}", self.id).as_str())).1.to_string()
    }

    /// Creates a new GDObject from ID, config, and extra proerties
    pub fn new(id: i32, config: GDObjConfig, properties: Vec<(u16, String)>) -> Self {
        GDObject { 
           id, config, properties
        }
    }

    /// Gets the property.
    pub fn get_property(&self, p: u16) -> Option<Value> {
        match p {
            1 => Some(Value::from(self.id)),
            2 => Some(Value::from(self.config.pos.0)),
            3 => Some(Value::from(self.config.pos.1)),
            6 => Some(Value::from(self.config.angle)),
            11 => Some(Value::from(self.config.trigger_cfg.touchable)),
            57 => Some(Value::from(self.config.groups.clone())),
            62 => Some(Value::from(self.config.trigger_cfg.spawnable)),
            87 => Some(Value::from(self.config.trigger_cfg.multitriggerable)),
            128 => Some(Value::from(self.config.scale.0)),
            129 => Some(Value::from(self.config.scale.1)),
            _ => self.properties.iter().find(|&(k, _)| *k == p).map(|(_, v)| Value::from(v.clone()))
        }
    }

    /// Sets the property.
    pub fn set_property(&mut self, p: u16, val: Value) {
        match p {
            1 => self.id = val.as_i64().unwrap() as i32,
            2 => self.config.pos.0 = val.as_f64().unwrap() as f32,
            3 => self.config.pos.1 = val.as_f64().unwrap() as f32,
            6 => self.config.angle = val.as_f64().unwrap() as f32,
            11 => self.config.trigger_cfg.touchable = val.as_bool().unwrap(),
            57 => self.config.groups = val.as_array().unwrap().iter().map(|v| v.as_i64().unwrap() as u16).collect(),
            62 => self.config.trigger_cfg.spawnable = val.as_bool().unwrap(),
            87 => self.config.trigger_cfg.multitriggerable = val.as_bool().unwrap(),
            128 => self.config.scale.0 = val.as_f64().unwrap() as f32,
            129 => self.config.scale.1 = val.as_f64().unwrap() as f32,
            _ => {
                let mut idx = 0;
                while self.properties[idx].0 >= p { 
                    idx += 1;
                }

                if self.properties[idx].0 == p {
                    self.properties[idx].1 = val.to_string()
                } else {
                    self.properties.insert(idx, (p, val.to_string()));
                }

            }
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
    pub trigger_cfg: TriggerConfig
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
            }
        }
    }

    /// Alias for default
    pub fn new() -> Self {
        Self::default()
    }

    // /// Converts this config to a properties hashmap
    // pub fn as_properties(&self) -> HashMap<GDObjProperty, Value> {
    //     let mut properties = json!({
    //         "2": self.pos.0,
    //         "3": self.pos.1,
    //         "64": 1,
    //         "67": 1,
    //         "155": 1,
    //         "6": self.angle,
    //         "128": self.scale.0,
    //         "129": self.scale.1,
    //         "11": self.trigger_cfg.touchable,
    //         "62": self.trigger_cfg.spawnable,
    //         "87": self.trigger_cfg.multitriggerable
    //     });

    //     if !self.groups.is_empty() && let Some(map) = properties.as_object_mut() {
    //         map.insert("57".to_owned(), Value::from(
    //             self.groups.iter().map(|&g| format!("{g}")).collect::<Vec<String>>().join(".")
    //         ));
    //     };

    //     let hashmap = properties.as_object().unwrap().into_iter()
    //         .map(|(k, v)| (GDObjProperty::from_name(k.clone()), v.clone())).collect();

    //     return hashmap
    // }

    pub fn to_string(&self) -> String {
        return format!(",2,{},3,{},64,1,67,1,155,1,6,{},128,{},129,{},11,{},62,{},87,{}", 
            self.pos.0, self.pos.1, self.angle, self.scale.0, self.scale.1, 
            self.trigger_cfg.touchable, self.trigger_cfg.spawnable, self.trigger_cfg.multitriggerable
        );
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

}