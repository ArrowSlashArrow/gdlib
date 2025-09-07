//! This module contains the GDObject struct, used for parsing to/from raw object strings
//! This module also contains the GDObjConfig struct for creating new GDObjects
use std::{collections::HashMap, str::FromStr};
use serde_json::{json, Value};
use crate::utils::strip_default_vals;

pub mod triggers;
pub mod objs;

/// Container for GD Object properties.
/// * `id`: The object's ID.
/// * `config`: General properties like position and scale.
/// * `properties`: Object-specific properties like target group for a move trigger
#[derive(Clone, Debug)]
pub struct GDObject {
    pub id: i32,
    pub config: GDObjConfig,
    pub properties: HashMap<String, Value>
}

fn get_float<T: Into<String> + Clone>(properties: &mut HashMap<String, Value>, key: T) -> f32 {
    match properties.get_mut(&key.clone().into()) {
        Some(v) => {
            let val = v.as_f64().unwrap() as f32;
            properties.remove(&key.clone().into());
            val
        },
        None => 0.0f32
    }
}

fn get_int<T: Into<String> + Clone>(properties: &mut HashMap<String, Value>, key: T) -> i32 {
    match properties.get_mut(&key.clone().into()) {
        Some(v) => {
            let val = v.as_i64().unwrap() as i32;
            properties.remove(&key.clone().into());
            val
        },
        None => 0i32
    }
}

fn get_bool<T: Into<String> + Clone>(properties: &mut HashMap<String, Value>, key: T) -> bool {
    match properties.get_mut(&key.clone().into()) {
        Some(_) => true,
        None => false
    }
}

impl GDObject {
    /// Parses raw object string to GDObject
    /// 
    /// Example:
    /// ```
    /// let obj = GDObject::parse_str("1,1,155,1,67,1,64,1,3,15.0,2,15.0;");
    /// assert_eq!(obj, GDObject::from(1, GDObjConfig::default(), HashMap::new()));
    /// ```
    pub fn parse_str(s: &str) -> Self {
        let mut properties: HashMap<String, Value> = HashMap::new();
        let mut current_property = String::new();
        for (idx, val) in s[..s.len() - 1].split(",").into_iter().enumerate() {
            if idx % 2 == 0 { // key
                current_property = val.to_string();
            } else { // value
                properties.insert(current_property.clone(), Value::from(val));
            }
        }

        let id = get_int(&mut properties, "1");
        let xpos = get_float(&mut properties, "2");
        let ypos = get_float(&mut properties, "3");
        let xscale = get_float(&mut properties, "128");
        let yscale = get_float(&mut properties, "129");
        let angle = get_float(&mut properties, "6");

        let touchable = get_bool(&mut properties, "11");
        let spawnable = get_bool(&mut properties, "62");
        let multitriggerable = get_bool(&mut properties, "87");

        // groups are stored as "1.2.3.4" -> groups 1, 2, 3, 4
        let groups = match properties.get_mut("57") {
            Some(v) => {
                let groups = v.to_string().split(".").filter_map(|g| match g.is_empty() {
                    true => None,
                    false => Some(g.parse::<u16>().unwrap())
                }).collect::<Vec<u16>>();
                properties.remove("57");
                groups
            },
            None => vec![]
        };
        
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
            properties
        }
    }

    /// Returns this object as a property string
    /// 
    /// Example:
    /// ```
    /// let object_str = GDObject::new(1, GDObjConfig::default(), HashMap::new()).to_string();
    /// assert_eq!(object_str, "1,1,155,1,67,1,64,1,3,15.0,2,15.0;");
    /// ```
    pub fn to_string(&self) -> String {
        let mut combined_properties = self.properties.clone();
        combined_properties.extend(self.config.as_properties());
        combined_properties = strip_default_vals(combined_properties);

        let mut raw_str = format!("1,{}", self.id);

        for (k, v) in combined_properties.iter() {
            raw_str += &format!(",{k},{}", v.to_string());
        };

        return raw_str.replace("\"", "") + ";";
    }

    /// Creates a new GDObject from ID, config, and extra proerties
    pub fn new(id: i32, config: GDObjConfig, properties: HashMap<String, Value>) -> Self {
        GDObject {
            id, config, properties: properties
        }
    }
}

/// Trigger config, used for defining general properties of a trigger object:
/// * is touch triggerable?
/// * is spawn triggerable?
/// * is multitriggerable?
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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