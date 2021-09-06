use syn::{Field, Ident, ItemStruct};

use crate::{
    attrs::{raw_typedef_ident, Category, RidAttr, StructConfig, TypeInfoMap},
    parse_rid_attrs,
};

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
        let fields = item
            .fields
            .iter()
            .filter_map(|f| match should_include_field(f) {
                true => Some(ParsedStructField::new(f, &config.type_infos)),
                fale => None,
            })
            .collect();
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

fn should_include_field(f: &Field) -> bool {
    let parsed_attrs = parse_rid_attrs(&f.attrs);
    !parsed_attrs.iter().any(RidAttr::has_skip)
}
