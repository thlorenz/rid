use syn::Variant;

use crate::common::ParsedField;

pub struct ParsedVariant {
    pub ident: syn::Ident,
    pub method_ident: syn::Ident,
    pub fields: Vec<ParsedField>,
}

impl ParsedVariant {
    pub fn new(v: Variant, method_prefix: &str) -> Self {
        let ident = v.ident.clone();
        let method_ident = method_ident_from_variant(method_prefix, &ident);
        let fields: Vec<ParsedField> = extract_fields(v);
        Self {
            ident,
            method_ident,
            fields,
        }
    }
}

fn extract_fields(v: Variant) -> Vec<ParsedField> {
    // TODO: maybe method_prefix doesn't belong on ParsedField?
    v.fields
        .into_iter()
        .filter_map(|f| match f.ident {
            Some(_) => Some(ParsedField::new(f, "")),
            None => None,
        })
        .collect()
}

fn method_ident_from_variant(method_prefix: &str, variant_ident: &syn::Ident) -> syn::Ident {
    let fn_name = format!("{}_{}", method_prefix, variant_ident);
    syn::Ident::new(&fn_name, variant_ident.span())
}
