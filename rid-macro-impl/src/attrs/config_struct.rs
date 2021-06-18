use std::collections::HashMap;

use syn::{Ident, ItemStruct};

use crate::{common::abort, parse_rid_attrs};

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug, Clone)]
pub struct StructConfig {
    pub debug: bool,
    pub type_infos: TypeInfoMap,
    pub attrs: Vec<RidAttr>,
}

impl StructConfig {
    pub fn new(attrs: Vec<RidAttr>) -> Self {
        let mut debug = false;
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());

        for attr in &attrs {
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
                RidAttr::Export(attr_ident, _) => {
                    abort!(
                        attr_ident,
                        "cannot have rid::export attribute on structs"
                    );
                }
                RidAttr::DeriveDebug(_) => debug = true,
            }
        }
        Self {
            debug,
            type_infos,
            attrs,
        }
    }

    pub fn from(struct_item: &ItemStruct) -> Self {
        let rid_attrs = parse_rid_attrs(&struct_item.attrs);
        Self::new(rid_attrs)
    }
}
