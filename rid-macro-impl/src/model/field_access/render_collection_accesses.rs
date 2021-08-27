use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

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
        rendered_dart.parse().unwrap()
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
    let mut all_nested_accesses: HashMap<String, Box<dyn RenderableAccess>> =
        HashMap::new();

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
                        nested_accesses,
                    } = x.render_rust();
                    if let Some(nested_accesses) = nested_accesses {
                        for (k, v) in nested_accesses {
                            all_nested_accesses.insert(k, v);
                        }
                    }
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

        // Append accesses that are needed to support the accesses we just aggregated
        if !all_nested_accesses.is_empty() {
            let mut nested_aggregated = aggregate_collection_accesses(
                all_nested_accesses,
                type_infos,
                rust_config,
                dart_config,
            );
            aggregated
                .rust_tokens
                .append(&mut nested_aggregated.rust_tokens);
            aggregated.darts.append(&mut nested_aggregated.darts);
        }

        aggregated
    }
}
