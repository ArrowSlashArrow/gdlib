use std::{collections::HashMap, env, error::Error, fs, path::PathBuf, time::Instant};

use serde_json::Value;

// property, type, default
const DEFAULT_PROPERTY_VALUES: &[(&str, &str, f32)] = &[
    ("6", "float", 0.0),
    ("128", "float", 1.0),
    ("129", "float", 1.0),
    ("11", "bool", 0.0), 
    ("62", "bool", 0.0), 
    ("87", "bool", 0.0), 
];

pub fn get_local_levels_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        Ok(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat").into())
    } else {
        Err("Could not find local levels file".into())
    }
}

pub fn dbg_write<T: Into<String>>(val: T) {
    fs::write(".debug", val.into()).unwrap();
}

pub struct Benchmark {
    inst: Instant,
    desc: String
}

impl Benchmark {
    pub fn start<T: Into<String>>(desc: T) -> Self {
        let start = Instant::now();
        Benchmark { inst: start, desc: desc.into() }
    }

    pub fn stop(&mut self) {
        let elapsed = self.inst.elapsed();
        println!("{} benchmark: {:.3}ms", self.desc, elapsed.as_micros() as f64 / 1000.0)
    }
}
#[macro_export]
macro_rules! timer {
    ($tok:expr) => {
        Benchmark::start($tok)
    };
}

pub fn vec_as_str(v: &Vec<u8>) -> String {
    String::from_utf8(v.to_owned()).unwrap()
}


pub fn properties_from_vec(vals: Value) -> HashMap<String, Value> {
    vals.as_object().unwrap().into_iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn strip_default_vals(properties: HashMap<String, Value>) -> HashMap<String, Value> {
    properties.into_iter().map(|(k, v)| {
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
    }).flatten().collect::<HashMap<String, Value>>()
}