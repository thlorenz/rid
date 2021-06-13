use syn::{DataStruct, Ident};

use crate::attrs::{raw_typedef_ident, Category};

use super::{rust_type::RustType, ParsedStructField};

#[derive(Debug)]
pub struct ParsedStruct {
    pub ident: Ident,
    pub raw_ident: Ident,
    pub fields: Vec<ParsedStructField>,
}

impl ParsedStruct {
    pub fn new(data: &DataStruct, ident: &Ident) -> Self {
        let ident = ident.clone();
        let raw_ident = raw_typedef_ident(&ident);
        let fields = data.fields.iter().map(ParsedStructField::new).collect();
        Self {
            ident,
            raw_ident,
            fields,
        }
    }
}
