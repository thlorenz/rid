use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, Data, DataEnum, DeriveInput, Token, Variant,
};

use crate::{
    common::{
        abort, extract_variant_names,
        state::{get_state, ImplementationType},
        tokens::cstring_free,
    },
    parse::{rust_type::RustType, ParsedStruct},
    render_dart::ParsedStructRenderConfig,
    render_rust::RenderedDisplayImpl,
};

pub struct DartObjectImplConfig {
    render_cstring_free: bool,
    render_dart_only: bool,
}

impl Default for DartObjectImplConfig {
    fn default() -> Self {
        Self {
            render_cstring_free: true,
            render_dart_only: false,
        }
    }
}

impl DartObjectImplConfig {
    pub fn for_tests() -> Self {
        Self {
            render_cstring_free: false,
            render_dart_only: false,
        }
    }
}

pub fn rid_dart_impl(
    input: &DeriveInput,
    config: DartObjectImplConfig,
) -> TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let parsed_struct = ParsedStruct::new(&data, &input.ident);

            let comment = if config.render_dart_only { "" } else { "///" };
            let render_class_config = ParsedStructRenderConfig {
                comment: comment.to_string(),
                dart_class_only: false,
                include_equality: true,
            };
            let dart_extension = parsed_struct
                .render_struct_pointer_to_class_extension(&render_class_config);
            let dart_tokens: TokenStream = dart_extension.parse().unwrap();
            let ident = &input.ident;
            let mod_ident = format_ident!("__rid_{}_dart_mod", ident);
            let fn_ident = format_ident!("_to_dart_for_{}", ident);
            quote_spanned! { input.ident.span() =>
                #[allow(non_snake_case)]
                mod #mod_ident {
                    #dart_tokens
                    #[no_mangle]
                    pub extern "C" fn #fn_ident() {}
                }
            }
        }
        Data::Enum(_) => {
            abort!(input.ident, "Cannot derive DartObject for an enum")
        }
        Data::Union(data) => abort!(
            input.ident,
            "Cannot derive DartObject for an untagged Union type"
        ),
    }
}
