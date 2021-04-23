use syn::Ident;

use crate::common::abort;
use std::collections::HashMap;

use super::{add_idents_to_type_map, Category, RidAttr, TypeInfo, TypeInfoMap};

#[derive(Debug)]
pub struct FunctionConfig {
    pub type_infos: TypeInfoMap,
    pub is_exported: bool,
    pub fn_export_alias: Option<Ident>,
}

impl FunctionConfig {
    pub fn new(
        attrs: &[RidAttr],
        owner: Option<(&syn::Ident, &TypeInfoMap)>,
    ) -> Self {
        // TODO: very much duplicated from ./config_impl_block.rs @see there for more info
        let mut type_infos: TypeInfoMap = TypeInfoMap(HashMap::new());
        let mut is_exported: bool = false;
        let mut fn_export_alias: Option<Ident> = None;
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
                        "cannot have rid::message attribute on functions or methods"
                    );
                }
                RidAttr::Export(attr_ident, name) => {
                    is_exported = true;
                    fn_export_alias = name.clone();
                }
                // The below are invalid on a function but already checked by Rust itself
                RidAttr::DeriveDebug(_) => {}
            }
        }
        if let Some((ident, owner_type_infos)) = owner {
            merge_type_infos(&mut type_infos, owner_type_infos);

            let key = ident.to_string();
            if !type_infos.contains_key(&key) {
                type_infos.insert(
                    key.clone(),
                    TypeInfo {
                        key: ident.clone(),
                        cat: Category::Struct,
                    },
                );
            }
            // Other parts of the type resolution process, i.e. RustType shouldn't need to
            // know about the special case of 'Self' therefore we alias it to the owner type here
            if let Some(type_info) = type_infos.get(&key) {
                let type_info = type_info.clone();
                type_infos.insert("Self".to_string(), type_info);
            }
        }
        Self {
            type_infos,
            is_exported,
            fn_export_alias,
        }
    }
}

fn merge_type_infos(tgt: &mut TypeInfoMap, src: &TypeInfoMap) {
    for (key, val) in src.iter() {
        if tgt.contains_key(key) {
            abort!(val.key, "duplicate type info key")
        }
        tgt.insert(key.clone(), val.clone());
    }
}
