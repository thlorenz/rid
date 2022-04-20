use std::collections::HashMap;

use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use super::render_rust_field_access::RenderRustFieldAccessResult;
use crate::{
    accesses::{
        render_collection_accesses, RenderDartAccessConfig,
        RenderRustAccessConfig,
    },
    attrs::TypeInfoMap,
    common::state::{get_state, ImplementationType},
    parse::ParsedStruct,
    render_rust::ffi_prelude,
};

impl ParsedStruct {
    pub fn render_field_access(
        &self,
        rust_config: &RenderRustAccessConfig,
        dart_config: &RenderDartAccessConfig,
    ) -> (TokenStream, String) {
        if self.fields.is_empty() {
            return (TokenStream::new(), String::new());
        }

        let RenderRustFieldAccessResult {
            tokens: rust_tokens,
            collection_accesses,
        } = self.render_rust_fields_access(rust_config);

        let rust_tokens = if rust_config.render {
            rust_tokens
        } else {
            TokenStream::new()
        };

        let type_infos = &self.type_infos();
        let (access_tokens, dart_accesses_string) = render_collection_accesses(
            collection_accesses,
            type_infos,
            &rust_config,
            &dart_config,
        );

        let (dart_tokens, dart_string) = if dart_config.render {
            self.render_dart_fields_access_extension(dart_config)
        } else {
            (TokenStream::new(), "".to_string())
        };

        let mod_name = format_ident!(
            "__{}_field_access",
            self.ident.to_string().to_snake_case()
        );
        (
            quote_spanned! {self.ident.span() =>
                mod #mod_name {
                    use super::*;
                    #access_tokens

                    #dart_tokens
                    #rust_tokens
                }
            },
            format!("{}\n{}", dart_string, dart_accesses_string),
        )
    }
}
