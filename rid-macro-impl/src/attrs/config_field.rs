use std::collections::{HashMap, HashSet};

use proc_macro_error::abort;

use crate::attrs::{RidAttr, TypeInfo};

use crate::message::VariantField;

#[derive(Debug)]
pub struct FieldConfig {
    pub types: HashMap<String, TypeInfo>,
}

fn merge(tgt: &mut HashMap<String, TypeInfo>, src: &HashMap<String, TypeInfo>) {
    for (key, val) in src {
        if tgt.contains_key(key) {
            abort!(val.key, "duplicate key")
        }
        tgt.insert(key.clone(), val.clone());
    }
}

impl FieldConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        use RidAttr::*;
        let mut types: HashMap<String, TypeInfo> = HashMap::new();
        for attr in attrs {
            match attr {
                Debug(ident) => {
                    abort!(ident, "debug can only be exposed for model structs")
                }
                Model(ident, _) => {
                    abort!(ident, "debug can only be exposed for model structs")
                }
                Export(ident) => abort!(
                    ident,
                    "export can only be applied to functions and struct impl blocks"
                ),
                Types(ident, hash) => merge(&mut types, hash),
                Wip => {}
            }
        }
        Self { types }
    }
}
