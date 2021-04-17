use std::collections::HashMap;

use syn::Ident;

use crate::common::abort;

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct StructConfig {
    pub debug: bool,
    pub type_infos: TypeInfoMap,
}

impl StructConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        let mut debug = false;
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());

        for attr in attrs {
            match attr {
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
                        "cannot have rid::message attribute on structs"
                    );
                }
                RidAttr::Export(attr_ident) => {
                    abort!(
                        attr_ident,
                        "cannot have rid::export attribute on structs"
                    );
                }
                RidAttr::DeriveDebug(_) => debug = true,
            }
        }
        Self { debug, type_infos }
    }
}
