use super::{message_args::MessageArgs, parsed_enum::ParsedEnum};
use crate::common::callsite_error;
use quote::quote;

use std::convert::TryFrom;

// https://stackoverflow.com/a/65182902/97443
pub fn rid_ffi_message_impl(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let enum_ident = ast.ident;
    match MessageArgs::try_from(ast.attrs) {
        Ok(args) => match &ast.data {
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                let parsed_enum = ParsedEnum::new(enum_ident, variants.clone(), args);
                parsed_enum.tokens()
            }
            _ => callsite_error("message can only be attached to enums"),
        },

        Err(errors) => {
            let errors = errors.into_iter().map(|err| callsite_error(&err));
            quote! { #(#errors)* }
        }
    }
}
