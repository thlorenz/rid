use crate::{
    attrs::{RidAttrOld, TypeInfoMap},
    common::abort,
};

#[derive(Debug)]
pub struct FieldConfig {
    pub types: TypeInfoMap,
}

fn validate_attrs(attrs: &[RidAttrOld]) {
    use RidAttrOld::*;

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
            Types(_ident, _hash) => {}
            Wip => {}
        }
    }
}

impl FieldConfig {
    pub fn new(attrs: &[RidAttrOld]) -> Self {
        validate_attrs(attrs);
        Self {
            types: attrs.into(),
        }
    }
}
