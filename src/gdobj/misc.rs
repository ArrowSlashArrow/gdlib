//! This module contains constructor for miscellaneous objects, like text or the default block
//! ## ⚠️ Warning
//! **This file is incomplete. More objects will be added in future releases.**
use base64::{engine::general_purpose, Engine};
use serde_json::json;

use crate::{gdobj::{GDObjConfig, GDObjProperties, GDObject}};

/// Returns a default block object.
/// # Arguments
/// `config`: Object config 
pub fn default_block(config: GDObjConfig) -> GDObject { 
    GDObject::new(1, config, GDObjProperties::new())
}

/// Returns a text object 
/// # Arguments
/// `config`: Object config 
/// `text`: Text in the objecty
/// `kerning`: Spacing between chars. Default is 0
pub fn text<T: AsRef<str>>(config: GDObjConfig, text: T, kerning: i32) -> GDObject {
    GDObject::new(914, config, GDObjProperties::from_json(json!({
        "24": "9",
        "31": general_purpose::STANDARD.encode(text.as_ref().to_string()),
        "488": kerning
    })))
}