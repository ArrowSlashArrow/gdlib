use std::collections::HashMap;

use crate::gdobj::{GDObjConfig, GDObject};

pub fn default_block(config: GDObjConfig) -> GDObject { 
    GDObject::new(1, config, HashMap::new())
}