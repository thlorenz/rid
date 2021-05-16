use super::{debug::render_debug, to_dart::render_to_dart};
use crate::{
    attrs::{parse_derive_attrs, StructConfig},
    common::abort,
    model::{
        parsed_struct::ParsedStruct,
        store::{render_store_field_wrapper_extension, render_store_module},
    },
    parse,
    parse::rust_type::RustType,
};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Fields, ItemStruct};

pub fn render_struct(struct_item: &ItemStruct, is_store: bool) -> TokenStream {
    let derive = parse_derive_attrs(&struct_item.attrs);
    let struct_config = StructConfig::from(&struct_item);
    let parsed_struct = parse::ParsedStruct::new(
        &struct_item,
        &struct_item.ident,
        struct_config.clone(),
    );

    // -----------------
    // Dart Class
    // -----------------
    let dart_class_tokens =
        render_to_dart(&parsed_struct, is_store, &derive, Default::default());

    // -----------------
    // derive(Debug)
    // -----------------
    let derive_debug_tokens = if derive.debug {
        let rust_type = RustType::from_owned_struct(&parsed_struct.ident);
        render_debug(rust_type, &None, Default::default())
    } else {
        TokenStream::new()
    };

    // -----------------
    // Store Module
    // -----------------
    let (store_module, store_wrapper_tokens) = if is_store {
        (
            render_store_module(&struct_item.ident),
            render_store_field_wrapper_extension(&parsed_struct),
        )
    } else {
        (TokenStream::new(), TokenStream::new())
    };

    // -----------------
    // rid::export
    // -----------------
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

    // -----------------
    // Combine all the above
    // -----------------
    quote_spanned! { struct_item.ident.span() =>
        #dart_class_tokens
        #derive_debug_tokens
        #store_module
        #exports
        #store_wrapper_tokens
    }
}
