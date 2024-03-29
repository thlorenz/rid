use syn::Ident;

use std::collections::HashMap;

use crate::{
    attrs::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap},
    common::abort,
};

#[derive(Debug)]
pub struct MessageEnumConfig {
    pub debug: bool,
    pub type_infos: TypeInfoMap,
    pub to: Ident,
    pub reply: Ident,
}

impl MessageEnumConfig {
    pub fn new(
        attrs: &[RidAttr],
        model_ident: Ident,
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
                        "cannot have config rid() attributes on message enum"
                    );
                }
                RidAttr::DeriveDebug(_) => debug = true,
            }
        }
        Self {
            debug,
            type_infos,
            to: model_ident,
            reply: reply_ident.clone(),
        }
    }
}
