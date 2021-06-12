use syn::{DataStruct, Ident};

use super::{rust_type::RustType, ParsedStructField};

#[derive(Debug)]
pub struct ParsedStruct {
    pub ident: syn::Ident,
    pub fields: Vec<ParsedStructField>,
}

impl ParsedStruct {
    pub fn new(data: &DataStruct, ident: &Ident) -> Self {
        let ident = ident.clone();
        let fields = data.fields.iter().map(ParsedStructField::new).collect();
        Self { ident, fields }
    }
}
