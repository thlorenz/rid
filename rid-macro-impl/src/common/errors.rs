use proc_macro2::{Span, TokenStream};
use syn::spanned::Spanned;

use crate::common::abort;

pub fn type_error(ty: &syn::Type, err: &String) -> TokenStream {
    let full_err = format!("rid: {}", err);
    syn::Error::new(ty.span(), full_err).to_compile_error()
}

pub fn derive_error(ident: &syn::Ident, err: &str) -> TokenStream {
    let full_err = format!("rid: {}", err);
    syn::Error::new(ident.span(), full_err).to_compile_error()
}

pub fn callsite_error(err: &str) -> TokenStream {
    let full_err = format!("rid: {}", err);
    syn::Error::new(Span::call_site(), full_err).to_compile_error()
}

pub fn missing_struct_enum_info(ident: &syn::Ident) -> TokenStream {
    abort!(
        ident,
        "[rid] Missing info for type {0}.\nSpecify it via one of the below:\n\
        \x20- #[rid::structs({0})]\n\
        \x20- #[rid::enums({0})]",
        ident
    )
}

pub fn missing_msg_field_enum_info(ident: &syn::Ident) -> TokenStream {
    abort!(
        ident,
        "[rid] Missing info for type {0}.\nSpecify it via: #[rid::enums({0})]\n\n\
        Note that for message fields only primitive types, strings and enums are supported.",
        ident
    )
}
