//! This module contains constructor for miscellaneous objects, like text or the default block
//! ## ⚠️ Warning
//! **This file is incomplete. More objects will be added in future releases.**
use base64::{Engine, engine::general_purpose};

use crate::gdobj::{
    GDObjConfig, GDObject, GDValue,
    ids::{
        objects::{DEFAULT_BLOCK, TEXT_OBJECT},
        properties::*,
    },
};

/// Returns a default block object.
/// # Arguments
/// `config`: Object config
#[inline(always)]
pub fn default_block(config: &GDObjConfig) -> GDObject {
    GDObject::new(DEFAULT_BLOCK, config, vec![])
}

/// Returns a text object
/// # Arguments
/// `config`: Object config
/// `text`: Text in the objecty
/// `kerning`: Spacing between chars. Default is 0
pub fn text<T: AsRef<str>>(config: &GDObjConfig, text: T, kerning: i32) -> GDObject {
    GDObject::new(
        TEXT_OBJECT,
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
