use std::collections::HashMap;

use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use crate::{
    attrs::TypeInfoMap,
    common::state::{get_state, ImplementationType},
    parse::ParsedStruct,
    render_common::{
        AccessRender, RenderableAccess, RenderedAccessRust, VecAccess,
    },
    render_rust::ffi_prelude,
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
            collection_accesses,
        } = self.render_rust_fields_access(rust_config);

        let rust_tokens = if rust_config.render {
            rust_tokens
        } else {
            TokenStream::new()
        };

        let type_infos = &self.type_infos();
        let (vec_access_tokens, dart_vec_accesses_string) =
            aggregate_collection_accesses(
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
                    #vec_access_tokens

                    #dart_tokens
                    #rust_tokens
                }
            },
            format!("{}\n{}", dart_string, dart_vec_accesses_string),
        )
    }
}

fn aggregate_collection_accesses(
    accesses: HashMap<String, Box<dyn RenderableAccess>>,
    type_infos: &TypeInfoMap,
    rust_config: &RenderRustFieldAccessConfig,
    dart_config: &RenderDartFieldAccessConfig,
) -> (TokenStream, String) {
    if accesses.is_empty() {
        (TokenStream::new(), "".to_string())
    } else {
        struct RenderedAccesses {
            rust_tokens: Vec<TokenStream>,
            darts: Vec<String>,
        }
        let rendered = accesses.values().into_iter().fold(
            RenderedAccesses {
                rust_tokens: vec![],
                darts: vec![],
            },
            |mut accesses, x| {
                let access_needs_implementation = get_state()
                    .needs_implementation(
                        &ImplementationType::CollectionAccess,
                        &x.key(),
                    );
                let should_render_rust_access = match rust_config.accesses {
                    AccessRender::Force => true,
                    AccessRender::Omit => false,
                    AccessRender::Default => access_needs_implementation,
                };

                let should_render_dart_access = match dart_config.accesses {
                    AccessRender::Force => true,
                    AccessRender::Omit => false,
                    AccessRender::Default => access_needs_implementation,
                };

                if should_render_rust_access {
                    let RenderedAccessRust {
                        tokens: rust_tokens,
                        type_aliases,
                    } = x.render_rust();
                    let typedef_tokens: Vec<TokenStream> =
                        type_aliases.into_iter().map(|x| x.typedef).collect();

                    let rust = quote_spanned! { x.span() =>
                        #(#typedef_tokens)*
                        #rust_tokens
                    };
                    accesses.rust_tokens.push(rust);
                }

                if should_render_dart_access {
                    let dart: String =
                        x.render_dart(type_infos, &dart_config.comment);
                    accesses.darts.push(dart);
                }
                accesses
            },
        );
        let rendered_dart = if dart_config.render && !rendered.darts.is_empty()
        {
            format!(
                r###"
{comment}```dart
{comment}
{comment} // Access methods for Rust Builtin Types required by the below methods.
{comment}
{rendered_dart}
{comment}```"###,
                comment = dart_config.comment,
                rendered_dart = rendered.darts.join("\n"),
            )
        } else {
            "".to_string()
        };
        let dart_tokens: TokenStream =
            if dart_config.render && dart_config.tokens {
                rendered_dart.parse().unwrap()
            } else {
                TokenStream::new()
            };

        let rust_tokens = if rust_config.render {
            rendered.rust_tokens
        } else {
            vec![]
        };

        (
            quote! {
                #dart_tokens
                #(#rust_tokens)*
            },
            rendered_dart,
        )
    }
}
