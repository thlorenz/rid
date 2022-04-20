use syn::{Ident, ItemImpl};

use crate::{common::abort, parse_rid_attrs};
use std::collections::HashMap;

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct ImplBlockConfig {
    pub type_infos: TypeInfoMap,
    pub attrs: Vec<RidAttr>,
    pub is_exported: bool,
}

impl ImplBlockConfig {
    pub fn new(attrs: Vec<RidAttr>) -> Self {
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());
        let mut is_exported = false;
        for attr in &attrs {
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
                RidAttr::Export(attr_ident, name) => {
                    if name.is_some() {
                        abort!(attr_ident, "impl block exports cannot define name and should always be just '#[rid::export]'")
                    } else {
                        is_exported = true;
                    }
                }
                RidAttr::Rid(attr_ident, _) => {
                    abort!(
                        attr_ident,
                        "cannot have config rid() attributes on impl blocks"
                    );
                }
                // The below are invalid on an impl block but already checked by Rust itself
                RidAttr::DeriveDebug(_) => {}
            }
        }
        Self {
            type_infos,
            attrs,
            is_exported,
        }
    }

    pub fn from(impl_item: &ItemImpl) -> Self {
        let attrs = parse_rid_attrs(&impl_item.attrs);
        ImplBlockConfig::new(attrs)
    }
}
