use std::{collections::HashMap, env, error::Error, fs, path::{self, Path, PathBuf}, time::Instant};

use serde_json::Value;

/// Default values of GD object properties
/// (property, type, value) 
const DEFAULT_PROPERTY_VALUES: &[(&str, &str, f32)] = &[
    ("6", "float", 0.0),
    ("128", "float", 1.0),
    ("129", "float", 1.0),
    ("11", "bool", 0.0), 
    ("62", "bool", 0.0), 
    ("87", "bool", 0.0), 
];

/// Returns path of CCLocalLevels.dat if it exists, otherwise return Err
pub fn get_local_levels_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") && Path::new(&local_appdata).exists(){
        Ok(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat").into())
    } else {
        Err("Could not find local levels file".into())
    }
}

/// Debug function that writes `val` to `.debug` file
pub fn dbg_write<T: Into<String>>(val: T) {
    fs::write(".debug", val.into()).unwrap();
}

/// desc: descriptor for benchmark
/// inst: when benchmark timer was started 
pub struct Benchmark {
    inst: Instant,
    desc: String
}

impl Benchmark {
    /// Starts the timer by initialising the start time
    pub fn start<T: Into<String>>(desc: T) -> Self {
        let start = Instant::now();
        Benchmark { inst: start, desc: desc.into() }
    }

    /// Stops timer and displays time and descriptior as milliseconds
    pub fn stop(&mut self) {
        let elapsed = self.inst.elapsed();
        println!("{} benchmark: {:.3}ms", self.desc, elapsed.as_micros() as f64 / 1000.0)
    }
}

/// Shorthand for Benchmark::start
#[macro_export]
macro_rules! timer {
    ($tok:expr) => {
        Benchmark::start($tok)
    };
}

/// Converts Vec<u8> utf8 to String
pub fn vec_as_str(v: &Vec<u8>) -> String {
    String::from_utf8(v.to_owned()).unwrap()
}

/// Converts properties in `serde_json::Value` dict to a `HashMap<String, Value>` 
pub fn properties_from_vec(vals: Value) -> HashMap<String, Value> {
    vals.as_object().unwrap().into_iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

/// Removes default values from a property hashmap
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