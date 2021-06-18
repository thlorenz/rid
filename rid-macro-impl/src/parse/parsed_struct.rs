use syn::{Ident, ItemStruct};

use crate::attrs::{raw_typedef_ident, Category, StructConfig, TypeInfoMap};

use super::{rust_type::RustType, ParsedStructField};

#[derive(Debug)]
pub struct ParsedStruct {
    pub ident: Ident,
    pub raw_ident: Ident,
    pub fields: Vec<ParsedStructField>,
    pub config: StructConfig,
}

impl ParsedStruct {
    pub fn new(item: &ItemStruct, ident: &Ident, config: StructConfig) -> Self {
        let ident = ident.clone();
        let raw_ident = raw_typedef_ident(&ident);
        let fields = item.fields.iter().map(ParsedStructField::new).collect();
        Self {
            ident,
            raw_ident,
            fields,
            config,
        }
    }

    /// Information about custom types used for fields that are part of this struct
    pub fn type_infos(&self) -> &TypeInfoMap {
        &self.config.type_infos
    }
}
