use syn::Variant;

use super::variant_field::MessageVariantField;
use crate::attrs::TypeInfoMap;

pub struct ParsedMessageVariant {
    pub ident: syn::Ident,
    pub method_ident: syn::Ident,
    pub fields: Vec<MessageVariantField>,
}

impl ParsedMessageVariant {
    pub fn new(v: Variant, method_prefix: &str, types: &TypeInfoMap) -> Self {
        let ident = v.ident.clone();
        let method_ident = method_ident_from_variant(method_prefix, &ident);
        let fields = extract_fields(v, types);
        Self {
            ident,
            method_ident,
            fields,
        }
    }
}

fn extract_fields(v: Variant, types: &TypeInfoMap) -> Vec<MessageVariantField> {
    v.fields
        .into_iter()
        .enumerate()
        .map(|(idx, f)| MessageVariantField::new(f, idx, types))
        .collect()
}

fn method_ident_from_variant(
    method_prefix: &str,
    variant_ident: &syn::Ident,
) -> syn::Ident {
    let fn_name = format!("{}_{}", method_prefix, variant_ident);
    syn::Ident::new(&fn_name, variant_ident.span())
}
