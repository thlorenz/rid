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

pub fn render_to_dart(
    parsed_struct: &ParsedStruct,
    is_store: bool,
    render_config: DartRenderImplConfig,
) -> TokenStream {
    let comment = if render_config.render_dart_only {
        ""
    } else {
        "///"
    };

    // -----------------
    // Dart Store API
    // -----------------
    let dart_store_api = if is_store {
        parsed_struct.render_store_api(comment)
    } else {
        "".to_string()
    };

    // -----------------
    // toDart() including Dart Class
    // -----------------
    let render_class_config = ParsedStructRenderConfig {
        comment: comment.to_string(),
        dart_class_only: false,
        include_equality: true,
        include_to_string: true,
        is_store,
    };

    let to_dart_extension = parsed_struct
        .render_struct_pointer_to_class_extension(&render_class_config);

    // -----------------
    // Dart Code Block
    // -----------------
    let dart_code = format!(
        r###"
{comment} ```dart
{dart_store_api}
{to_dart_extension}
{comment} ```"###,
        dart_store_api = dart_store_api,
        to_dart_extension = to_dart_extension,
        comment = comment
    );
    let dart_tokens: TokenStream = dart_code.parse().unwrap();

    let ident = &parsed_struct.ident;
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
