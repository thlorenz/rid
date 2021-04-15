use std::collections::HashMap;

use crate::{attrs::RidAttrOld, common::abort};

use super::TypeInfoMap;

pub fn merge_type_infos(tgt: &mut TypeInfoMap, src: &TypeInfoMap) {
    for (key, val) in src.iter() {
        if tgt.contains_key(key) {
            abort!(val.key, "duplicate type info key")
        }
        tgt.insert(key.clone(), val.clone());
    }
}

impl From<&[RidAttrOld]> for TypeInfoMap {
    fn from(attrs: &[RidAttrOld]) -> Self {
        use RidAttrOld::*;
        let mut types: TypeInfoMap = TypeInfoMap(HashMap::new());
        for attr in attrs {
            match attr {
                Types(_ident, hash) => merge_type_infos(&mut types, hash),
                _ => {}
            }
        }
        types
    }
}
