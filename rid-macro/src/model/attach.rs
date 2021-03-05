use crate::{
    common::{callsite_error, ParsedDerive},
    model::parsed_struct::ParsedStruct,
};
use syn::{self, Fields, FieldsNamed, ItemStruct};

use quote::quote;

pub fn rid_ffi_model_impl(item: syn::Item) -> proc_macro2::TokenStream {
    let model_struct = match &item {
        syn::Item::Struct(s) => s,
        _ => return callsite_error("model can only be attached to structs"),
    };

    let derive: ParsedDerive = match model_struct {
        ItemStruct { attrs, .. } => ParsedDerive::from_attrs(attrs),
    };

    let tokens = match model_struct {
        ItemStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ident,
            ..
        } => {
            let parsed_struct = ParsedStruct::new(ident.clone(), named.clone(), derive);
            parsed_struct.derive_code()
        }
        ItemStruct {
            fields: Fields::Unit,
            ..
        } => callsite_error("model attribute makes no sense on empty structs"),
        _ => unimplemented!(),
    };

    quote! {
        #item
        #tokens
    }
}
