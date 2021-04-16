use syn::Ident;

use crate::common::abort;
use std::collections::HashMap;

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct EnumConfig {
    pub type_infos: TypeInfoMap,
    pub to: Ident,
}

fn extract_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    use syn::*;
    match expr {
        Expr::Path(ExprPath { path, .. }) => path.get_ident(),
        _ => None,
    }
}

impl EnumConfig {
    pub fn new(
        enum_ident: &syn::Ident,
        attrs: &[RidAttr],
        model_ident: &Ident,
    ) -> Self {
        // TODO: very much duplicated from ./config_struct.rs
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());
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
                        "cannot have rid::message attribute on structs"
                    );
                }
                RidAttr::Export(attr_ident) => {
                    abort!(
                        attr_ident,
                        "cannot have rid::export attribute on structs"
                    );
                }
            }
        }
        Self {
            type_infos,
            to: model_ident.clone(),
        }
    }
}
