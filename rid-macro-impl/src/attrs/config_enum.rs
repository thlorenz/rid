use syn::Ident;

use crate::common::abort;
use std::collections::HashMap;

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct EnumConfig {
    // TODO: does not yet generate any ffi code for this yet since enums are only used for messages
    // for now which aren't printed in Dart
    pub debug: bool,
    pub type_infos: TypeInfoMap,
    pub to: Ident,
    pub reply: Ident,
}

impl EnumConfig {
    pub fn new(
        attrs: &[RidAttr],
        model_ident: &Ident,
        reply_ident: &Ident,
    ) -> Self {
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
                        "cannot have rid::message attribute on enums"
                    );
                }
                RidAttr::Export(attr_ident, _) => {
                    abort!(
                        attr_ident,
                        "cannot have rid::export attribute on enums"
                    );
                }
                RidAttr::DeriveDebug(_) => debug = true,
            }
        }
        Self {
            debug,
            type_infos,
            to: model_ident.clone(),
            reply: reply_ident.clone(),
        }
    }
}
