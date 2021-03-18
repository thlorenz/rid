use std::collections::{HashMap, HashSet};

use crate::{
    attrs::{RidAttr, TypeInfo, TypeInfoMap},
    common::abort,
    message::VariantField,
};

#[derive(Debug)]
pub struct FieldConfig {
    pub types: TypeInfoMap,
}

fn validate_attrs(attrs: &[RidAttr]) {
    use RidAttr::*;

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
            Types(ident, hash) => {}
            Wip => {}
        }
    }
}

impl FieldConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        validate_attrs(attrs);
        Self {
            types: attrs.into(),
        }
    }
}
