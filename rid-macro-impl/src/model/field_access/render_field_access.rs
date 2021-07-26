use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use crate::{
    attrs::TypeInfoMap, parse::ParsedStruct, render_common::VecAccess,
    render_rust::vec::RenderedVecRust,
};

use super::{
    render_dart_field_access::RenderDartFieldAccessConfig,
    render_rust_field_access::{
        RenderRustFieldAccessConfig, RenderRustFieldAccessResult,
    },
};

impl ParsedStruct {
    pub fn render_field_access(
        &self,
        rust_config: &RenderRustFieldAccessConfig,
        dart_config: &RenderDartFieldAccessConfig,
    ) -> (TokenStream, String) {
        let RenderRustFieldAccessResult {
            tokens: rust_tokens,
            vec_accesses,
        } = self.render_rust_fields_access(rust_config);

        let rust_tokens = if rust_config.render {
            rust_tokens
        } else {
            TokenStream::new()
        };

        let type_infos = &self.type_infos();
        let vec_access_tokens: TokenStream = aggregate_vec_accesses(
            vec_accesses,
            type_infos,
            &rust_config,
            &dart_config,
        );

        let (dart_tokens, dart_string) = if dart_config.render {
            self.render_dart_fields_access_extension(dart_config)
        } else {
            (TokenStream::new(), "".to_string())
        };

        let mod_name = format_ident!("__{}_field_access", self.ident);
        (
            quote_spanned! {self.ident.span() =>
                mod #mod_name {
                    use super::*;
                    #vec_access_tokens

                    #dart_tokens
                    #rust_tokens
                }
            },
            dart_string,
        )
    }
}

fn aggregate_vec_accesses(
    vec_accesses: HashMap<String, VecAccess>,
    type_infos: &TypeInfoMap,
    rust_config: &RenderRustFieldAccessConfig,
    dart_config: &RenderDartFieldAccessConfig,
) -> TokenStream {
    if vec_accesses.is_empty() {
        TokenStream::new()
    } else {
        struct RenderedVecAccesses {
            rust_tokens: Vec<TokenStream>,
            darts: Vec<String>,
        }
        let rendered = vec_accesses.values().into_iter().fold(
            RenderedVecAccesses {
                rust_tokens: vec![],
                darts: vec![],
            },
            |mut accesses, x| {
                let dart: String =
                    x.render_dart(type_infos, &dart_config.comment);

                let RenderedVecRust {
                    tokens: rust_tokens,
                    type_aliases,
                } = x.render_rust();
                let typedef_tokens: Vec<TokenStream> =
                    type_aliases.into_iter().map(|x| x.typedef).collect();

                let rust = quote_spanned! { x.vec_type_ident.span() =>
                    #(#typedef_tokens)*
                    #rust_tokens
                };
                accesses.rust_tokens.push(rust);
                accesses.darts.push(dart);
                accesses
            },
        );
        // TODO(thlorenz): return the rendered access string for tests
        let dart_tokens: TokenStream =
            if dart_config.render && dart_config.tokens {
                format!(
                    r###"
{comment}
{comment} Access methods for Rust Builtin Types required by the below methods.
{comment}
{comment} ```dart
{rendered_darts}
{comment} ```"###,
                    comment = dart_config.comment,
                    rendered_darts = rendered.darts.join("\n"),
                )
                .parse()
                .unwrap()
            } else {
                TokenStream::new()
            };

        let rust_tokens = if rust_config.render {
            rendered.rust_tokens
        } else {
            vec![]
        };

        quote! {
            #dart_tokens
            #(#rust_tokens)*
        }
    }
}
