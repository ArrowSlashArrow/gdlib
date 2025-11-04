//! This module contains various utilities for debugging and processing structs
use std::{collections::HashMap, env, error::Error, fmt::Debug, path::{Path, PathBuf}};
use aho_corasick::AhoCorasick;
use base64::Engine;
use serde_json::Value;

use crate::gdobj::GDObjProperty;

/// Returns path of CCLocalLevels.dat if it exists, otherwise return Err
pub fn get_local_levels_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") && Path::new(&local_appdata).exists(){
        Ok(format!("{local_appdata}/GeometryDash/CCLocalLevels.dat").into())
    } else {
        Err("Could not find local levels file".into())
    }
}

/// Converts properties in `serde_json::Value` dict to a `HashMap<String, Value>` 
pub fn properties_from_json(vals: Value) -> HashMap<GDObjProperty, Value> {
    vals.as_object().unwrap().into_iter()
        .map(|(k, v)| (GDObjProperty::from_name(k.clone()), v.clone())).collect()
}

/// Replaces Robtop's plist format with actual plist tags; i.e. `<s>` becomes `<string>`
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

/// Quick function for clamping `val` to the nearest value in `clamps`
pub fn clamp_to_values(val: f64, clamps: &[f64]) -> f64 {
    clamps.iter().min_by(|&&a, &&b| {
        let dist_a = (a - val).abs();
        let dist_b = (b - val).abs();
        dist_a.partial_cmp(&dist_b).unwrap()
    }).unwrap().clone()
}

/// Quick function for decoding base64 bytes
pub fn b64_decode<T: AsRef<[u8]> + Debug>(encoded: T) -> Vec<u8> {
    base64::engine::general_purpose::URL_SAFE.decode(encoded).unwrap()
}

/// Quick function for encoding base64 bytes
pub fn b64_encode(encoded: Vec<u8>) -> String {
    base64::engine::general_purpose::URL_SAFE.encode(encoded)
}