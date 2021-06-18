use syn::Field;

use crate::{attrs::TypeInfoMap, common::abort};

use super::rust_type::RustType;

#[derive(Debug)]
pub struct ParsedStructField {
    pub ident: syn::Ident,
    pub rust_type: RustType,
}

impl ParsedStructField {
    pub fn new(f: &Field, type_infos: &TypeInfoMap) -> Self {
        // unwrap is ok here since we only support structs with named fields
        let ident = f.ident.as_ref().unwrap().clone();
        let rust_type = RustType::from_type(&f.ty, type_infos);
        let rust_type = match rust_type {
            Some(x) => x,
            None => abort!(
                ident,
                "Not supporting custom types yet when deriving DartState"
            ),
        };
        Self { ident, rust_type }
    }
}
