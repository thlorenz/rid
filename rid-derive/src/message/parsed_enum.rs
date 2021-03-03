use super::parsed_variant::ParsedVariant;
use crate::common::resolvers::{instance_ident, resolve_ptr};
use quote::{format_ident, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Variant};

type Tokens = proc_macro2::TokenStream;

pub struct ParsedEnum {
    pub ident: syn::Ident,
    pub method_prefix: String,
    pub parsed_variants: Vec<ParsedVariant>,
}

impl ParsedEnum {
    pub fn new(ident: syn::Ident, variants: Punctuated<Variant, Comma>) -> Self {
        let method_prefix = format!("rid_{}", ident.to_string().to_lowercase()).to_string();
        let parsed_variants = parse_variants(variants, &method_prefix);
        Self {
            method_prefix,
            ident,
            parsed_variants,
        }
    }

    pub fn derive_code(&self) -> Tokens {
        if self.parsed_variants.is_empty() {
            return Tokens::new();
        }
        let method_tokens: Tokens = self
            .parsed_variants
            .iter()
            .map(|v| self.rust_method(v))
            .collect();
        quote_spanned! { self.ident.span() =>
            #method_tokens
        }
    }

    //
    // Rust Module
    //

    fn rust_method(&self, variant: &ParsedVariant) -> Tokens {
        let variant_ident = &variant.ident;
        let fn_ident = &variant.method_ident;

        // TODO: how do we know what the model is?
        // If Msg is parsed first then we haven't even seen it yet.
        // Letting the user provide it as an attribute is easiest, but also makes him think that
        // there are options.
        // Possibly we could check the Model for update methods??? Complicated.
        let struct_ident = format_ident!("Model");
        let struct_instance_ident = instance_ident(&struct_ident);
        let resolve_struct_ptr = resolve_ptr(&struct_ident);

        let enum_ident = &self.ident;
        eprintln!("fields {:?}", variant.fields);

        let args: Vec<syn::Ident> = variant
            .fields
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                format_ident!(
                    "arg{} {}",
                    idx,
                    f.rust_ty.as_ref().map(|x| x.to_string()).unwrap()
                )
            })
            .collect();

        quote_spanned! { variant_ident.span() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_ident(ptr: *mut #struct_ident, #(#args)*) {
                let mut #struct_instance_ident = #resolve_struct_ptr;
                let msg = #enum_ident::#variant_ident(#(#args)*);
                #struct_instance_ident.update(msg);
            }
        }
    }
}

fn parse_variants(variants: Punctuated<Variant, Comma>, method_prefix: &str) -> Vec<ParsedVariant> {
    variants
        .into_iter()
        .map(|v| ParsedVariant::new(v, &method_prefix))
        .collect()
}
