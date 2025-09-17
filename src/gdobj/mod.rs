//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::{collections::{BTreeMap, HashMap}, fmt::{Debug, Display}};
use serde_json::{json, Number, Value};

use crate::utils::properties_from_json;

pub mod triggers;
pub mod misc;

/// Default values of GD object properties: 
/// (property, type, value) 
pub const DEFAULT_PROPERTY_VALUES: &[(&str, &str, f32)] = &[
    ("6", "float", 0.0),
    ("128", "float", 1.0),
    ("129", "float", 1.0),
    ("11", "bool", 0.0), 
    ("62", "bool", 0.0), 
    ("87", "bool", 0.0), 
];

// TODO: fill in all the properties
// btw, if anyone has a better idea to represent property descriptors (they differ from object to object), lmk
/// Names of properties (INCOMPLETE): 
/// (property, name)
pub const PROPERTY_NAMES: &[(&str, &str)] = &[
    ("2", "x pos"),
    ("3", "y pos"),
    ("7", "Red"),
    ("8", "Green"),
    ("9", "Blue"),
    ("10", "Fade time/duration"),
    ("15", "Using player colour 1"),
    ("16", "Using player colour 2"),
    ("17", "Blending enabled"),
    ("23", "Colour channel"),
    ("31", "Base64-encoded text"),
    ("35", "Opacity"),
    ("36", "Is active trigger?"),
    ("45", "Pulse fade in time"),
    ("46", "Pulse hold time"),
    ("47", "Pulse fade out time"),
    ("49", "Copy colour specs"),
    ("50", "Copy colour from channel"),
    ("51", "Target group/item/channel"),
    ("60", "Copy opacity"),
    ("80", "Group/item 1"),
    ("86", "Exclusive pulse mode"),
    ("95", "Group/item 2"),
    ("210", "No legacy HSV"),
    ("217", "Enter/Exit transition config"),
    ("344", "Target transition channel"),
    ("392", "Song ID"),
    ("399", "Prep?"),
    ("400", "Load Prep?"),
    ("404", "Song speed"),
    ("406", "Song volume"),
    ("408", "Start offset in ms"),
    ("409", "Fade in time in ms"),
    ("410", "End offset in ms"),
    ("411", "Fade out time in ms"),
    ("413", "Loop song?"),
    ("432", "Song channel"),
    ("476", "First item type"),
    ("477", "Second item type"),
    ("479", "Modifier"),
    ("483", "Second modifier"),
    ("595", "Don't stop song on death"),
    // these are all startpos properties:
    ("kA4", "Starting speed"),
    ("kA2", "Starting gamemode"),
    ("kA3", "Starting in mini mode?"),
    ("kA8", "Starting in dual mode?"),
    ("kA21", "Is disabled?"),
    ("kA28", "Starting in mirror mode?"),
    ("kA29", "Rotate gameplay?"),
    ("kA20", "Reverse gameplay?"),
    ("kA19", "Target order"),
    ("kA26", "Target channel"),
    ("kA35", "Reset camera?")
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
    (1615, "Counter"),
    (1616, "Stop trigger"),
    (1815, "Collision trigger"),
    (1816, "Collision block"),
    (1915, "Don't fade + don't enter transition object"),
    (1917, "Reverse gameplay"),
    (1934, "Song trigger"),
    (1935, "Time warp trigger"),
    (3618, "Reset group trigger"),
    (3619, "Item edit trigger"),
    (3620, "Item compare trigger"),
    (3641, "Persistent item trigger"),
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
            match DEFAULT_PROPERTY_VALUES.iter().find(|(p, _t, _v)| *p == key) {
                Some((_key, _type, fallback)) => {
                    as_number(Value::from(*fallback))
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

    /// Removes default values from a property hashmap
    pub fn strip_default_vals(&mut self) {
        self.properties = self.properties.clone().into_iter().map(|(k, v)| {
            let property_values = match DEFAULT_PROPERTY_VALUES.iter().find(|&&p| p.0 == k) {
                Some(p) => p,
                None => return Some((k, v))
            };
            let default_value = match property_values.1 {
                "bool" => Value::from(property_values.2 != 0.0),
                "float" => Value::from(property_values.2),
                "int" => Value::from(property_values.2 as i32),
                _ => return None
            };

            if v == default_value {
                return None
            }

            Some((k, v))
        }).flatten().collect::<HashMap<String, Value>>();
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

        write!(f, "{trigger_conf_str}{} @ ({}, {}) scaled to ({}, {}){} angled {}°", 
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
            let descriptor = match PROPERTY_NAMES.iter().find(|&p| p.0 == *pname) {
                Some(v) => v.1,
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
        combined_properties.strip_default_vals();

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
    pub fn groups(mut self, groups: Vec<u16>) -> Self {
        self.groups = groups;
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
    /// Sets x and y sacle of this object
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.pos = (x, y);
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