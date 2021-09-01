use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use crate::{
    attrs::TypeInfoMap,
    common::state::{get_state, ImplementationType},
    render_common::{
        AccessKind, AccessRender, HashMapAccess, RenderableAccess,
        RenderedAccessRust, VecAccess,
    },
};

use super::{
    render_dart_field_access::RenderDartFieldAccessConfig,
    render_rust_field_access::RenderRustFieldAccessConfig,
};

struct AggregatedRenderedAccesses {
    rust_tokens: Vec<TokenStream>,
    darts: Vec<String>,
}

pub fn render_collection_accesses(
    accesses: HashMap<String, Box<dyn RenderableAccess>>,
    type_infos: &TypeInfoMap,
    rust_config: &RenderRustFieldAccessConfig,
    dart_config: &RenderDartFieldAccessConfig,
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
    let dart_tokens: TokenStream = if dart_config.render && dart_config.tokens {
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
    rust_config: &RenderRustFieldAccessConfig,
    dart_config: &RenderDartFieldAccessConfig,
) -> AggregatedRenderedAccesses {
    if accesses.is_empty() {
        AggregatedRenderedAccesses {
            rust_tokens: vec![],
            darts: vec![],
        }
    } else {
        let mut aggregated = accesses.values().into_iter().fold(
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
                    let rust = quote_spanned! { x.span() =>
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
