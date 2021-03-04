use syn::ItemEnum;

use super::parsed_enum::ParsedEnum;
use crate::common::callsite_error;
use quote::quote;

// https://stackoverflow.com/a/65182902/97443
pub fn rid_ffi_message_impl(item: syn::Item) -> proc_macro2::TokenStream {
    let tokens = match &item {
        syn::Item::Enum(ItemEnum {
            variants, ident, ..
        }) => {
            let parsed_enum = ParsedEnum::new(ident.clone(), variants.clone());
            parsed_enum.tokens()
        }
        _ => callsite_error("message can only be attached to enums"),
    };

    quote! {
        #item
        #tokens
    }
}
