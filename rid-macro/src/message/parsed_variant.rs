use syn::Variant;

use super::variant_field::VariantField;

pub struct ParsedVariant {
    pub ident: syn::Ident,
    pub method_ident: syn::Ident,
    pub fields: Vec<VariantField>,
    pub errors: Vec<String>,
}

impl ParsedVariant {
    pub fn new(v: Variant, method_prefix: &str) -> Self {
        let ident = v.ident.clone();
        let method_ident = method_ident_from_variant(method_prefix, &ident);
        let (errors, fields) = extract_fields(v);
        Self {
            ident,
            method_ident,
            fields,
            errors,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }
}

fn extract_fields(v: Variant) -> (Vec<String>, Vec<VariantField>) {
    v.fields
        .into_iter()
        .enumerate()
        .map(|(idx, f)| VariantField::new(f, idx))
        .fold(
            (vec![], vec![]),
            |(mut errors, mut fields): (Vec<String>, Vec<VariantField>), res| {
                match res {
                    Ok(field) => fields.push(field),
                    Err(err) => errors.push(err),
                };
                (errors, fields)
            },
        )
}

fn method_ident_from_variant(method_prefix: &str, variant_ident: &syn::Ident) -> syn::Ident {
    let fn_name = format!("{}_{}", method_prefix, variant_ident);
    syn::Ident::new(&fn_name, variant_ident.span())
}
