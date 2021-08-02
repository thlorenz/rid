use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, Data, DataEnum, DeriveInput, Token, Variant,
};

use crate::{
    common::{
        abort, extract_variant_names,
        state::{get_state, ImplementationType},
        utils_module_tokens,
    },
    parse::rust_type::RustType,
    render_rust::RenderedDisplayImpl,
};

pub struct DisplayImplConfig {
    render_dart_extension: bool,
    render_dart_enum: bool,
    render_swift_methods: bool,
}

impl Default for DisplayImplConfig {
    fn default() -> Self {
        Self {
            render_dart_extension: true,
            render_dart_enum: true,
            render_swift_methods: true,
        }
    }
}

impl DisplayImplConfig {
    pub fn for_tests() -> Self {
        Self {
            render_dart_extension: false,
            render_dart_enum: false,
            render_swift_methods: false,
        }
    }
}

pub fn rid_display_impl(
    input: &DeriveInput,
    config: DisplayImplConfig,
) -> TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let rust_type = RustType::from_owned_struct(&input.ident);
            let tokens = render_display(rust_type, &config, &None);
            let utils_module = utils_module_tokens();
            quote! {
                #tokens
                #utils_module
            }
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let rust_type = RustType::from_owned_enum(&input.ident);
            let variants = Some(extract_variant_names(variants));
            let tokens = render_display(rust_type, &config, &variants);
            let utils_module = utils_module_tokens();
            quote! {
                #tokens
                #utils_module
            }
        }
        Data::Union(data) => abort!(
            input.ident,
            "Cannot derive display for an untagged Union type"
        ),
    }
}

fn render_display(
    rust_type: RustType,
    config: &DisplayImplConfig,
    enum_variants: &Option<Vec<String>>,
) -> TokenStream {
    let RenderedDisplayImpl {
        tokens: rust_method_tokens,
        fn_display_method_ident,
    } = rust_type.render_display_impl(enum_variants);

    let dart_ext_tokens: TokenStream = if config.render_dart_extension {
        rust_type
            .render_dart_display_extension(
                &fn_display_method_ident.to_string(),
                "///",
            )
            .parse()
            .unwrap()
    } else {
        TokenStream::new()
    };

    let mod_ident = format_ident!("__rid_mod_{}", fn_display_method_ident);
    quote! {
        mod #mod_ident {
            use super::*;
            #dart_ext_tokens
            #rust_method_tokens
        }
    }
}
