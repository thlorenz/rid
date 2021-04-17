use syn::Ident;

use crate::common::abort;
use std::collections::HashMap;

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct ImplBlockConfig {
    pub type_infos: TypeInfoMap,
    pub is_exported: bool,
}

impl ImplBlockConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        // TODO: exactly duplicated from ./config_enum.rs
        // We may do a TypeInfoMap::From(&[RidAttr]), but then we still should validate + warn
        // for invalid attrs on a specific type
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());
        let mut is_exported = false;
        for attr in attrs {
            match attr {
                // TODO: detect #[derive(Debug)]
                RidAttr::Structs(attr_ident, idents) => add_idents_to_type_map(
                    &mut type_infos,
                    Category::Struct,
                    idents,
                ),
                RidAttr::Enums(attr_ident, idents) => add_idents_to_type_map(
                    &mut type_infos,
                    Category::Enum,
                    idents,
                ),
                RidAttr::Message(attr_ident, _) => {
                    abort!(
                        attr_ident,
                        "cannot have rid::message attribute on impl blocks"
                    );
                }
                RidAttr::Export(attr_ident) => is_exported = true,
                // The below are invalid on an impl block but already checked by Rust itself
                RidAttr::DeriveDebug(_) => {}
            }
        }
        Self {
            type_infos,
            is_exported,
        }
    }
}
