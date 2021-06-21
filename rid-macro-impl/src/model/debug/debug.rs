use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, Data, DataEnum, DeriveInput, Token, Variant,
};

use crate::{
    common::{
        abort, extract_variant_names,
        state::{get_state, ImplementationType},
        tokens::cstring_free,
    },
    parse::rust_type::RustType,
    render_rust::{RenderedDebugImpl, RenderedDisplayImpl},
};

#[derive(Clone)]
pub struct RenderDebugConfig {
    render_cstring_free: bool,
    render_dart_extension: bool,
    render_dart_enum: bool,
    render_swift_calls: bool,
}

impl Default for RenderDebugConfig {
    fn default() -> Self {
        Self {
            render_cstring_free: true,
            render_dart_extension: true,
            render_dart_enum: true,
            render_swift_calls: true,
        }
    }
}

impl RenderDebugConfig {
    pub fn for_tests() -> Self {
        Self {
            render_cstring_free: false,
            render_dart_extension: false,
            render_dart_enum: false,
            render_swift_calls: false,
        }
    }
}

pub fn render_debug(
    rust_type: RustType,
    enum_variants: &Option<Vec<String>>,
    config: RenderDebugConfig,
) -> TokenStream {
    let cstring_free_tokens = if config.render_cstring_free {
        cstring_free()
    } else {
        TokenStream::new()
    };

    let RenderedDebugImpl {
        tokens: rust_method_tokens,
        fn_debug_method_ident,
        fn_debug_pretty_method_ident,
    } = rust_type.render_debug_impl(enum_variants);

    let dart_ext_tokens: TokenStream = if config.render_dart_extension {
        rust_type
            .render_dart_debug_extension(
                &fn_debug_method_ident.to_string(),
                &fn_debug_pretty_method_ident.to_string(),
                "///",
            )
            .parse()
            .unwrap()
    } else {
        TokenStream::new()
    };

    let mod_ident = format_ident!("__rid_mod_{}", fn_debug_method_ident);
    let typealias = rust_type.typealias_tokens();
    quote! {
        mod #mod_ident {
            use super::*;
            #typealias
            #dart_ext_tokens
            #rust_method_tokens
            #cstring_free_tokens
        }
    }
}
