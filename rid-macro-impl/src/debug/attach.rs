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

pub struct DebugImplConfig {
    render_cstring_free: bool,
    render_dart_extension: bool,
    render_dart_enum: bool,
}

impl Default for DebugImplConfig {
    fn default() -> Self {
        Self {
            render_cstring_free: true,
            render_dart_extension: true,
            render_dart_enum: true,
        }
    }
}

impl DebugImplConfig {
    pub fn for_tests() -> Self {
        Self {
            render_cstring_free: false,
            render_dart_extension: false,
            render_dart_enum: false,
        }
    }
}

pub fn rid_debug_impl(
    input: &DeriveInput,
    config: DebugImplConfig,
) -> TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let rust_type = RustType::from_owned_struct(&input.ident);
            render_debug(rust_type, &config, &None)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let rust_type = RustType::from_owned_enum(&input.ident);
            let variants = Some(extract_variant_names(variants));
            render_debug(rust_type, &config, &variants)
        }
        Data::Union(data) => abort!(
            input.ident,
            "Cannot derive debug for an untagged Union type"
        ),
    }
}

fn render_debug(
    rust_type: RustType,
    config: &DebugImplConfig,
    enum_variants: &Option<Vec<String>>,
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

    // TODO: once model/parsed_struct.rs is normalized to parse::RustType we need to render
    // this enum there as well once we see a field of its type.
    let dart_enum_tokens: TokenStream = if config.render_dart_enum
        && rust_type.is_enum()
        && get_state().needs_implementation(
            &ImplementationType::DartEnum,
            &rust_type.ident.to_string(),
        ) {
        rust_type
            .render_dart_enum(
                enum_variants
                    .as_ref()
                    .expect("Need variants to render enum"),
                "///",
            )
            .parse()
            .unwrap()
    } else {
        TokenStream::new()
    };

    let mod_ident = format_ident!("__rid_mod_{}", fn_debug_method_ident);
    quote! {
        mod #mod_ident {
            use super::*;
            #dart_enum_tokens
            #dart_ext_tokens
            #rust_method_tokens
            #cstring_free_tokens
        }
    }
}
