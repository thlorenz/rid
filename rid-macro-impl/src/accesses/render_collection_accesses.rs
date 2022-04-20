use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use crate::{
    attrs::TypeInfoMap,
    common::state::{get_state, ImplementationType},
    render_rust::{allow_prelude, ffi_prelude},
};

use super::{
    AccessRender, HashMapAccess, RenderableAccess, RenderedAccessRust,
    VecAccess,
};

// -----------------
// RenderRustAccessConfig
// -----------------
pub struct RenderRustAccessConfig {
    pub accesses: AccessRender,
    pub ffi_prelude_tokens: TokenStream,
    pub render: bool,
}

impl Default for RenderRustAccessConfig {
    fn default() -> Self {
        Self {
            ffi_prelude_tokens: ffi_prelude(),
            render: true,
            accesses: AccessRender::Default,
        }
    }
}
impl RenderRustAccessConfig {
    pub fn for_rust_tests(accesses: AccessRender) -> Self {
        Self {
            ffi_prelude_tokens: TokenStream::new(),
            render: true,
            accesses,
        }
    }
}

impl RenderRustAccessConfig {
    pub fn for_dart_tests(accesses: AccessRender) -> Self {
        Self {
            ffi_prelude_tokens: TokenStream::new(),
            render: false,
            accesses,
        }
    }
}

// -----------------
// RenderRustAccessConfig
// -----------------
pub struct RenderDartAccessConfig {
    pub accesses: AccessRender,
    pub render: bool,
    pub tokens: bool,
    pub comment: String,
}

impl Default for RenderDartAccessConfig {
    fn default() -> Self {
        Self {
            comment: "/// ".to_string(),
            render: true,
            tokens: true,
            accesses: AccessRender::Default,
        }
    }
}

impl RenderDartAccessConfig {
    pub fn for_rust_tests() -> Self {
        Self {
            comment: "".to_string(),
            render: false,
            tokens: false,
            accesses: AccessRender::Omit,
        }
    }
}

impl RenderDartAccessConfig {
    pub fn for_dart_tests(accesses: AccessRender) -> Self {
        Self {
            comment: "".to_string(),
            render: true,
            tokens: false,
            accesses,
        }
    }
}

struct AggregatedRenderedAccesses {
    rust_tokens: Vec<TokenStream>,
    darts: Vec<String>,
}

pub fn render_collection_accesses(
    accesses: HashMap<String, Box<dyn RenderableAccess>>,
    type_infos: &TypeInfoMap,
    rust_config: &RenderRustAccessConfig,
    dart_config: &RenderDartAccessConfig,
) -> (TokenStream, String) {
    if accesses.is_empty() {
        return (TokenStream::new(), String::new());
    }
    let first_key = accesses.keys().next().unwrap().clone();
    let aggregated = aggregate_collection_accesses(
        accesses,
        type_infos,
        rust_config,
        dart_config,
    );
    let rendered_dart = if dart_config.render && !aggregated.darts.is_empty() {
        format!(
            r###"
{comment}```dart
{comment}
{comment} // Access methods for Rust Builtin Types required by the below methods.
{comment}
{rendered_dart}
{comment}```"###,
            comment = dart_config.comment,
            rendered_dart = aggregated.darts.join("\n"),
        )
    } else {
        "".to_string()
    };
    let dart_tokens: TokenStream = if dart_config.render
        && dart_config.tokens
        && !rendered_dart.is_empty()
    {
        let dart_tokens: TokenStream = rendered_dart.parse().unwrap();
        let fn_include_dart_ident =
            format_ident!("__include_dart_for_{}", first_key);
        let ffi_prelude = &rust_config.ffi_prelude_tokens;
        quote! {
            #dart_tokens
            #ffi_prelude
            fn #fn_include_dart_ident() {}
        }
    } else {
        TokenStream::new()
    };

    let rust_tokens = if rust_config.render {
        aggregated.rust_tokens
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

fn aggregate_collection_accesses(
    accesses: HashMap<String, Box<dyn RenderableAccess>>,
    type_infos: &TypeInfoMap,
    rust_config: &RenderRustAccessConfig,
    dart_config: &RenderDartAccessConfig,
) -> AggregatedRenderedAccesses {
    if accesses.is_empty() {
        AggregatedRenderedAccesses {
            rust_tokens: vec![],
            darts: vec![],
        }
    } else {
        let aggregated = accesses.values().into_iter().fold(
            AggregatedRenderedAccesses {
                rust_tokens: vec![],
                darts: vec![],
            },
            |mut accesses, x| {
                let key = x.key();
                let access_needs_implementation = get_state()
                    .needs_implementation(
                        &ImplementationType::CollectionAccess,
                        &key,
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
                    let typedef_tokens: Vec<TokenStream> = type_aliases
                        .values()
                        .into_iter()
                        .map(|x| x.typedef.clone())
                        .collect();

                    // We need to wrap this in these in a module in case the nested accesses
                    // result in rendering the same typedefs.
                    let mod_ident = format_ident!("mod_{}_access", x.key());
                    let allow = allow_prelude();
                    let rust = quote_spanned! { x.span() =>
                        #allow
                        mod #mod_ident {
                            use super::*;
                            #(#typedef_tokens)*
                            #rust_tokens
                        }
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

        aggregated
    }
}
