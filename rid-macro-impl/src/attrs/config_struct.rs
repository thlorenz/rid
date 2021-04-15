use std::collections::HashMap;

use syn::Ident;

use crate::common::abort;

use super::{Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct StructConfig {
    pub debug: bool,
    pub type_infos: TypeInfoMap,
}

fn add_idents(type_infos: &mut TypeInfoMap, cat: Category, idents: &[Ident]) {
    for ident in idents {
        type_infos.insert(
            ident.to_string(),
            TypeInfo {
                key: ident.clone(),
                cat: cat.clone(),
            },
        );
    }
}

impl StructConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        let debug = false;
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());

        for attr in attrs {
            match attr {
                // TODO: detect #[derive(Debug)]
                RidAttr::Structs(attr_ident, idents) => {
                    add_idents(&mut type_infos, Category::Struct, idents)
                }
                RidAttr::Enums(attr_ident, idents) => {
                    add_idents(&mut type_infos, Category::Enum, idents)
                }
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
        Self { debug, type_infos }
    }
}
