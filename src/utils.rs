//! This module contains various utilities for debugging and processing structs
use std::{collections::HashMap, env, error::Error, fs, ops::Sub, path::{Path, PathBuf}, time::Instant};
use aho_corasick::AhoCorasick;
use serde_json::Value;

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

/// Benchmark struct, used for recording execution time
/// `desc`: descriptor for benchmark
/// `inst`: when benchmark timer was started 
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

/// Converts `Vec<u8>` utf8 to String
pub fn vec_as_str(v: &Vec<u8>) -> String {
    String::from_utf8(v.to_owned()).unwrap()
}

/// Converts properties in `serde_json::Value` dict to a `HashMap<String, Value>` 
pub fn properties_from_json(vals: Value) -> HashMap<String, Value> {
    vals.as_object().unwrap().into_iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn proper_plist_tags(s: String) -> String {
    // replace gd plist with proper plist
    // using aho-corasick for single-pass instead of many .replace()s
    let find = &[
        "<k>", "</k>", "<i>", "</i>", "<d>", "</d>", "<d />","<t/>", "<f/>", 
        "<t />", "<f />", "<s>", "</s>", "<r>", "</r>"  
    ];
    let replace = &[
        "<key>", "</key>", "<integer>", "</integer>", "<dict>", "</dict>", "<dict />","<true/>", "<false/>", 
        "<true />", "<false />", "<string>", "</string>", "<real>", "</real>"  
    ];
    let ac = AhoCorasick::new(find).unwrap();
    let plist = ac.replace_all(&s, replace);
    return plist
}

pub fn clamp_to_values(val: f64, clamps: &[f64]) -> f64 {
    clamps.iter().min_by(|&&a, &&b| {
        let dist_a = (a - val).abs();
        let dist_b = (b - val).abs();
        dist_a.partial_cmp(&dist_b).unwrap()
    }).unwrap().clone()
}