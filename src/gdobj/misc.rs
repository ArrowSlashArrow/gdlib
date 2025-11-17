//! This module contains constructor for miscellaneous objects, like text or the default block
//! ## ⚠️ Warning
//! **This file is incomplete. More objects will be added in future releases.**
use base64::{Engine, engine::general_purpose};

use crate::gdobj::{GDObjConfig, GDObject, GDValue, ids::properties::*};

/// Returns a default block object.
/// # Arguments
/// `config`: Object config
pub fn default_block(config: GDObjConfig) -> GDObject {
    GDObject::new(1, config, vec![])
}

/// Returns a text object
/// # Arguments
/// `config`: Object config
/// `text`: Text in the objecty
/// `kerning`: Spacing between chars. Default is 0
pub fn text<T: AsRef<str>>(config: GDObjConfig, text: T, kerning: i32) -> GDObject {
    GDObject::new(
        914,
        config,
        vec![
            (
                BASE64ENCODED_TEXT,
                GDValue::String(general_purpose::STANDARD.encode(text.as_ref().to_string())),
            ),
            (KERNING, GDValue::Int(kerning)),
        ],
    )
}
