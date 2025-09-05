use std::collections::HashMap;

use crate::gdobj::{GDObjConfig, GDObject};

/// ## ⚠️ Warning
/// **This file is incomplete. More objects will be added in future releases.**

/// Return a default block object from the given config. 
pub fn default_block(config: GDObjConfig) -> GDObject { 
    GDObject::new(1, config, HashMap::new())
}