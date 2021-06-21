use super::to_dart::render_to_dart;
use crate::{
    attrs::StructConfig,
    common::abort,
    model::{
        parsed_struct::ParsedStruct,
        store::{render_store_field_wrapper_extension, render_store_module},
    },
    parse,
};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Fields, ItemStruct};

pub fn render_struct(struct_item: &ItemStruct, is_store: bool) -> TokenStream {
    let struct_config = StructConfig::from(&struct_item);
    let parsed_struct = parse::ParsedStruct::new(
        &struct_item,
        &struct_item.ident,
        struct_config.clone(),
    );
    let dart_class_tokens =
        render_to_dart(&parsed_struct, is_store, Default::default());

    let store_module = if is_store {
        render_store_module(&struct_item.ident)
    } else {
        TokenStream::new()
    };

    let store_wrapper_tokens = if is_store {
        render_store_field_wrapper_extension(&parsed_struct)
    } else {
        TokenStream::new()
    };

    let exports = match &struct_item.fields {
        Fields::Named(fields) => {
            let parsed_struct = ParsedStruct::new(
                struct_item.ident.clone(),
                &fields.named,
                struct_config,
            );
            parsed_struct.tokens()
        }
        Fields::Unnamed(_) => abort!(
            struct_item.ident,
            "not yet supporting structs with unnamed fields"
        ),
        Fields::Unit => abort!(
            struct_item.ident,
            "structs without fields cannot be a rid::model"
        ),
    };
    quote_spanned! { struct_item.ident.span() =>
        #store_module
        #exports
        #dart_class_tokens
        #store_wrapper_tokens
    }
}
