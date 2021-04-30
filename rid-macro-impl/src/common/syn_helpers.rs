use syn::{punctuated::Punctuated, Token, Variant};

pub fn extract_variant_names(
    variants: &Punctuated<Variant, Token![,]>,
) -> Vec<String> {
    variants.into_iter().map(|x| x.ident.to_string()).collect()
}
