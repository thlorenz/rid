use crate::{common::callsite_error, model::parsed_struct::ParsedStruct};
use syn::{self, Fields, FieldsNamed, ItemStruct};

use quote::quote;

pub fn rid_ffi_model_impl(item: syn::Item) -> proc_macro2::TokenStream {
    let tokens = match &item {
        syn::Item::Struct(ItemStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ident,
            ..
        }) => {
            let parsed_struct = ParsedStruct::new(ident.clone(), named.clone());
            parsed_struct.derive_code()
        }
        _ => callsite_error("model can only be attached to structs"),
    };

    quote! {
        #item
        #tokens
    }
}
