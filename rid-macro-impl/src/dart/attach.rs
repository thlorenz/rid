use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, Data, DataEnum, DeriveInput, ItemStruct, Token,
    Variant,
};

use crate::{
    attrs::{self, add_idents_to_type_map, StructConfig, TypeInfoMap},
    common::{
        abort,
        state::{get_state, ImplementationType},
        tokens::cstring_free,
    },
    parse::{rust_type::RustType, ParsedStruct},
    render_dart::ParsedStructRenderConfig,
    render_rust::RenderedDisplayImpl,
};

pub struct DartRenderImplConfig {
    render_cstring_free: bool,
    render_dart_only: bool,
}

impl Default for DartRenderImplConfig {
    fn default() -> Self {
        Self {
            render_cstring_free: true,
            render_dart_only: false,
        }
    }
}

impl DartRenderImplConfig {
    pub fn for_tests() -> Self {
        Self {
            render_cstring_free: false,
            render_dart_only: false,
        }
    }
}

pub fn rid_dart_impl(
    struct_item: &ItemStruct,
    struct_config: StructConfig,
    render_config: DartRenderImplConfig,
) -> TokenStream {
    let parsed_struct =
        ParsedStruct::new(&struct_item, &struct_item.ident, struct_config);

    // TODO: continue here by using type_infos when converting to DartType
    let comment = if render_config.render_dart_only {
        ""
    } else {
        "///"
    };
    let render_class_config = ParsedStructRenderConfig {
        comment: comment.to_string(),
        dart_class_only: false,
        include_equality: true,
        include_to_string: true,
    };
    let dart_extension = parsed_struct
        .render_struct_pointer_to_class_extension(&render_class_config);
    let dart_tokens: TokenStream = dart_extension.parse().unwrap();
    let ident = &struct_item.ident;
    let mod_ident = format_ident!("__rid_{}_dart_mod", ident);
    let fn_ident = format_ident!("_to_dart_for_{}", ident);
    quote_spanned! { ident.span() =>
        #[allow(non_snake_case)]
        mod #mod_ident {
            #dart_tokens
            #[no_mangle]
            pub extern "C" fn #fn_ident() {}
        }
    }
}
