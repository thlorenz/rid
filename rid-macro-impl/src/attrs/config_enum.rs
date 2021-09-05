use syn::{Ident, ItemEnum};

use crate::{common::abort, parse_rid_attrs};
use std::collections::HashMap;

use super::{
    add_idents_to_type_map, parse_rid_args, Category, RidAttr, TypeInfo,
    TypeInfoMap,
};

#[derive(Debug, PartialEq)]
pub struct EnumConfig {
    pub debug: bool,
    pub attrs: Vec<RidAttr>,
    pub type_infos: TypeInfoMap,
}

impl EnumConfig {
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
                        "cannot have duplicate rid::message attribute on enums"
                    );
                }
                RidAttr::Export(attr_ident, _) => {
                    abort!(
                        attr_ident,
                        "cannot have rid::export attribute on enums"
                    );
                }
                RidAttr::Rid(attr_ident, _) => {
                    abort!(
                        attr_ident,
                        "cannot have rid() config attributes on enums"
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

    pub fn from(enum_item: &ItemEnum) -> Self {
        let rid_attrs = parse_rid_attrs(&enum_item.attrs);
        Self::new(rid_attrs)
    }
}
