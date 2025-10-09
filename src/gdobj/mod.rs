//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::{collections::{BTreeMap, HashMap}, fmt::{Debug, Display}};
use serde_json::{json, Number, Value};

use crate::utils::properties_from_json;

pub mod triggers;
pub mod misc;

// todo: use these in the property names array
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

pub struct GDObjProperty {
    pub name: &'static str,
    pub desc: &'static str,
    pub arg_type: GDObjPropType,
    pub default: Option<f32>
}

// TODO: fill in all the properties
// btw, if anyone has a better idea to represent property descriptors (they differ from object to object), lmk
/// Names of properties (INCOMPLETE): 
/// (property, name)
pub const OBJECT_PROPERTIES: &[GDObjProperty] = &[
    GDObjProperty{name: "1", desc: "object ID", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "2", desc: "x pos", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "3", desc: "y pos", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "4", desc: "is flipped horizontally?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "5", desc: "is flipped vertically?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "6", desc: "rotation", arg_type: GDObjPropType::Float, default: Some(0.0)},
    GDObjProperty{name: "7", desc: "Red", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "8", desc: "Green", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "9", desc: "Blue", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "10", desc: "Fade time / chance to trigger group 1", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "11", desc: "Touch triggerable", arg_type: GDObjPropType::Bool, default: Some(0.0)},
    GDObjProperty{name: "15", desc: "Using player colour 1", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "16", desc: "Using player colour 2", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "17", desc: "Blending enabled", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "23", desc: "Colour channel", arg_type: GDObjPropType::ColourChannel, default: None},
    GDObjProperty{name: "28", desc: "Move units x", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "29", desc: "Move units y", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "30", desc: "Move easing", arg_type: GDObjPropType::Easing, default: None},
    GDObjProperty{name: "31", desc: "Base64-encoded text", arg_type: GDObjPropType::Text, default: None},
    GDObjProperty{name: "35", desc: "Opacity", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "36", desc: "Is active trigger?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "45", desc: "Pulse fade in time", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "46", desc: "Pulse hold time", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "47", desc: "Pulse fade out time", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "49", desc: "Copy colour specs", arg_type: GDObjPropType::Text, default: None},
    GDObjProperty{name: "50", desc: "Copy colour from channel", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "51", desc: "Target group/item/channel", arg_type: GDObjPropType::Group, default: None},
    GDObjProperty{name: "56", desc: "Activate group", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "58", desc: "Follow player's x movement", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "59", desc: "Follow player's y movement", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "60", desc: "Copy opacity", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "62", desc: "Spawn triggerable", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "71", desc: "Target group 2", arg_type: GDObjPropType::Group, default: None},
    GDObjProperty{name: "75", desc: "Shake strength", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "80", desc: "Group/item 1", arg_type: GDObjPropType::Item, default: None},
    GDObjProperty{name: "84", desc: "Shake interval", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "85", desc: "Easing rate", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "86", desc: "Exclusive pulse mode", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "87", desc: "Multitriggerable", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "94", desc: "Dynamic block?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "95", desc: "Group/item 2", arg_type: GDObjPropType::Item, default: None},
    GDObjProperty{name: "99", desc: "Multi activate", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "100", desc: "Target move mode", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "101", desc: "Target move mode axis lock", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "120", desc: "Timewarp amount", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "128", desc: "X scale", arg_type: GDObjPropType::Float, default: Some(1.0)},
    GDObjProperty{name: "129", desc: "Y scale", arg_type: GDObjPropType::Float, default: Some(1.0)},
    GDObjProperty{name: "138", desc: "Controlling player 1", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "141", desc: "Follow camera's x movement", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "142", desc: "Follow camera's y movement", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "143", desc: "X movement multiplier", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "144", desc: "Y movement multiplier", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "148", desc: "Gravity", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "200", desc: "Controlling player 2", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "201", desc: "Controlling target player", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "210", desc: "No legacy HSV", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "217", desc: "Enter/Exit transition config", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "344", desc: "Target transition channel", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "371", desc: "Camera zoom", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "392", desc: "Song ID", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "393", desc: "Small step", arg_type: GDObjPropType::Bool, default: None}, // this is a UI-only property. interally, move distances are stored the same regardless.
    GDObjProperty{name: "394", desc: "Directional move mode", arg_type: GDObjPropType::Bool, default: None} ,
    GDObjProperty{name: "395", desc: "Center group id", arg_type: GDObjPropType::Group, default: None},
    GDObjProperty{name: "397", desc: "Dynamic move", arg_type: GDObjPropType::Bool, default: None}, 
    GDObjProperty{name: "399", desc: "Prep?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "400", desc: "Load Prep?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "404", desc: "Song speed", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "406", desc: "Song volume", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "408", desc: "Start offset in ms", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "409", desc: "Fade in time in ms", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "410", desc: "End offset in ms", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "411", desc: "Fade out time in ms", arg_type: GDObjPropType::Int, default: None},
    GDObjProperty{name: "413", desc: "Loop song?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "432", desc: "Song channel", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "445", desc: "Claim touch?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "460", desc: "No end effects?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "461", desc: "Instant end?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "467", desc: "No end sound effects?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "472", desc: "Stop time counter?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "473", desc: "Target time for event", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "475", desc: "Multiactivatable time event", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "476", desc: "First item type", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "477", desc: "Second item type", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "479", desc: "Modifier", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "483", desc: "Second modifier", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "491", desc: "Set persistent item", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "492", desc: "Target all persistent items", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "493", desc: "Reset item to 0", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "494", desc: "Timer", arg_type: GDObjPropType::Item, default: None},
    GDObjProperty{name: "504", desc: "Spawn only", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "506", desc: "Camera guide preview opacity", arg_type: GDObjPropType::Float, default: None},
    GDObjProperty{name: "540", desc: "Stop player jump", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "541", desc: "Stop player movement", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "542", desc: "Stop player rotation", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "543", desc: "Stop player sliding", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "544", desc: "Silent move", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "595", desc: "Don't stop song on death", arg_type: GDObjPropType::Bool, default: None},
    // these are all startpos properties:
    GDObjProperty{name: "kA4", desc: "Starting speed", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "kA2", desc: "Starting gamemode", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "kA3", desc: "Starting in mini mode?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "kA8", desc: "Starting in dual mode?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "kA21", desc: "Is disabled?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "kA28", desc: "Starting in mirror mode?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "kA29", desc: "Rotate gameplay?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "kA20", desc: "Reverse gameplay?", arg_type: GDObjPropType::Bool, default: None},
    GDObjProperty{name: "kA19", desc: "Target order", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "kA26", desc: "Target channel", arg_type: GDObjPropType::Unknown, default: None},
    GDObjProperty{name: "kA35", desc: "Reset camera?", arg_type: GDObjPropType::Bool, default: None}
];

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

fn as_number(value: Value) -> Option<Number> {
    match value {
        Value::Number(n) => Some(n),
        Value::String(s) => {
            if let Ok(int) = s.parse::<i64>() {
                Some(Number::from(int))
            } else if let Ok(float) = s.parse::<f64>() {
                Number::from_f64(float)
            } else {
                None
            }
        },
        Value::Null => Some(Number::from(0)),
        _ => None
    }
}

fn get_num(properties: &mut HashMap<String, Value>, key: &str) -> Option<Number> {
    match properties.get_mut(key) {
        Some(v) => {
            // return val if known
            let val = match as_number(v.clone()) {
                None => return None,
                Some(v) => v
            };
            properties.remove(key);
            Some(val)
        },
        None => {
            // otherwise try return known default
            match OBJECT_PROPERTIES.iter().find(|prop| prop.name == key) {
                Some(property) => {
                    as_number(Value::from(property.default.unwrap()))
                },
                None => None
            }
        }
    }
}

fn get_float(properties: &mut HashMap<String, Value>, key: &str, default: f32) -> f32 {
    match get_num(properties, key) {
        Some(n) => n.as_f64().unwrap() as f32,
        None => default
    }
}

fn get_int(properties: &mut HashMap<String, Value>, key: &str, default: i32) -> i32 {
    match get_num(properties, key) {
        Some(n) => n.as_i64().unwrap() as i32,
        None => default
    }
}
fn get_bool(properties: &mut HashMap<String, Value>, key: &str) -> bool {
    properties.get_mut(key).is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct GDObjProperties {
    properties: HashMap<String, Value>
}

impl GDObjProperties {
    pub fn new() -> Self {
        GDObjProperties { properties: HashMap::new() }
    }

    pub fn to_string(&mut self) -> String {
        let mut raw_str = String::new();
        let mut sorted: Vec<_> = self.properties.iter().collect();
        sorted.sort_by_key(|&(k, _)| k);

        for (k, v) in sorted.iter() {
            raw_str += &format!(",{k},{}", v.to_string());
        };
        return raw_str[1..].to_string()
    }

    pub fn from_json(json: Value) -> Self {
        GDObjProperties {
            properties: properties_from_json(json)
        }
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
        for (pname, value) in sorted.iter() {
            let descriptor = match OBJECT_PROPERTIES.iter().find(|&p| p.name == *pname) {
                Some(v) => v.desc,
                None => &pname
            };
            property_str += &format!("\n    - {descriptor}: {value}");
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
    pub fn parse_str(s: &str) -> Self {
        let mut properties: HashMap<String, Value> = HashMap::new();
        let mut current_property = String::new();
        for (idx, val) in s.trim_end_matches(';').split(",").into_iter().enumerate() {
            if idx % 2 == 0 { // key
                current_property = val.to_string();
            } else { // value
                properties.insert(current_property.clone(), Value::from(val));
            }
        }

        let id = get_int(&mut properties, "1", 0);
        let xpos = get_float(&mut properties, "2", 0.0);
        let ypos = get_float(&mut properties, "3", 0.0);
        let xscale = get_float(&mut properties, "128", 0.0);
        let yscale = get_float(&mut properties, "129", 0.0);
        let angle = get_float(&mut properties, "6", 0.0);

        let touchable = get_bool(&mut properties, "11");
        let spawnable = get_bool(&mut properties, "62");
        let multitriggerable = get_bool(&mut properties, "87");

        // groups are stored as "1.2.3.4" -> groups 1, 2, 3, 4
        let groups = match properties.get_mut("57") {
            Some(v) => {
                let str = v.to_string().replace("\"", "");
                let groups = str.split(".").filter_map(|g| match g.is_empty() {
                    true => None,
                    false => {
                        Some(g.parse::<u16>().unwrap())
                    }
                }).collect::<Vec<u16>>();
                properties.remove("57");
                groups
            },
            None => vec![]
        };

        let mut properties_obj = GDObjProperties::new();
        properties_obj.properties = properties;
        
        GDObject { 
            id,
            config: GDObjConfig { 
                pos: (xpos, ypos), 
                scale: (xscale, yscale), 
                angle, 
                groups, 
                trigger_cfg: TriggerConfig { 
                    touchable, 
                    spawnable, 
                    multitriggerable 
                }
            }, 
            properties: properties_obj
        }
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

    /// Converts this config to a properties hashmap
    pub fn as_properties(&self) -> HashMap<String, Value> {
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
            "87": self.trigger_cfg.multitriggerable
        });

        if !self.groups.is_empty() && let Some(map) = properties.as_object_mut() {
            map.insert("57".to_owned(), Value::from(
                self.groups.iter().map(|&g| format!("{g}")).collect::<Vec<String>>().join(".")
            ));
        };

        let hashmap = properties.as_object().unwrap().into_iter().map(|(k, v)| (k.clone(), v.clone())).collect();

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

}