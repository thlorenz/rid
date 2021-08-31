use std::collections::HashMap;

use crate::{
    attrs::ImplBlockConfig,
    common::{
        state::{get_state, ImplementationType},
        utils_module_tokens_if,
    },
    parse::{ParsedFunction, ParsedImplBlock},
    render_common::{
        render_vec_accesses, PointerTypeAlias, RenderFunctionExportConfig,
        VecAccess,
    },
    render_dart,
    render_rust::{self, ffi_prelude, render_free, RenderedTypeAliasInfo},
};

use super::{
    export_config::ExportConfig,
    process_function_export::{
        extract_tokens, process_function_export, ExtractedTokens,
    },
};
use crate::{attrs::parse_rid_attrs, common::abort};
use quote::{format_ident, quote};

use crate::render_common::RenderableAccess;
use proc_macro2::TokenStream;
use render_dart::render_instance_method_extension;
use syn::Ident;

pub fn rid_export_impl(
    item: syn::Item,
    _args: syn::AttributeArgs,
    config: ExportConfig,
) -> TokenStream {
    match item {
        syn::Item::Impl(item) => {
            let impl_config = ImplBlockConfig::from(&item);
            let parsed = ParsedImplBlock::new(item, impl_config);

            let mut ptr_type_aliases_map =
                HashMap::<String, TokenStream>::new();
            let mut vec_accesses = HashMap::<String, VecAccess>::new();
            let rust_fn_tokens = &parsed
                .methods
                .iter()
                .map(|x| {
                    process_function_export(
                        x,
                        Some(parsed.ty.rust_ident().clone()),
                        config.include_ffi,
                        &mut ptr_type_aliases_map,
                        &mut vec_accesses,
                    )
                })
                .collect::<Vec<TokenStream>>();

            // Make sure we name the module differently for structs that have multiple impl blocks
            let module_ident = get_state().unique_ident(format_ident!(
                "__rid_{}_impl",
                parsed.ty.rust_ident()
            ));

            let ExtractedTokens {
                vec_access_tokens,
                ptr_typedef_tokens,
                utils_module,
            } = extract_tokens(
                vec_accesses,
                &ptr_type_aliases_map,
                parsed.type_infos(),
                &config,
            );

            // -----------------
            // Dart impl Extension
            // -----------------
            let dart_extension_tokens = if config.render_dart_extension {
                render_instance_method_extension(&parsed, None)
            } else {
                TokenStream::new()
            };

            quote! {
                #[allow(non_snake_case)]
                mod #module_ident {
                    use super::*;
                    #(#ptr_typedef_tokens)*
                    #dart_extension_tokens
                    #(#rust_fn_tokens)*
                    #(#vec_access_tokens)*
                    #utils_module
                }
            }
        }
        syn::Item::Fn(syn::ItemFn {
            attrs: _, // Vec<Attribute>,
            vis: _,   // Visibility,
            sig: _,   // Signature,
            block: _, // Box<Block>,
        }) => {
            // TODO: fix this
            // NOTE: at this point we don't support exports on top level functions, but impl
            // methods only.
            // In the future we may allow this again, but might use a different attribute.
            // The reason is that it is hard to know if a function is part of an impl and thus was
            // exported already.
            // An alternative would be to track already exported functions in our store via an id
            // that is based on function name and possibly content.
            // Another alternative is to require users to have a separate impl block with only
            // methods meant to be exported, possibly excluding some via a #[rid::skip] attr.

            // let attrs = attrs::parse_rid_attrs(&attrs);
            // let parsed = ParsedFunction::new(sig, &attrs, None);
            // render_function_export(&parsed, None, Default::default())
            TokenStream::new()
        }

        syn::Item::Const(_)
        | syn::Item::Enum(_)
        | syn::Item::ExternCrate(_)
        | syn::Item::ForeignMod(_)
        | syn::Item::Macro(_)
        | syn::Item::Macro2(_)
        | syn::Item::Mod(_)
        | syn::Item::Static(_)
        | syn::Item::Struct(_)
        | syn::Item::Trait(_)
        | syn::Item::TraitAlias(_)
        | syn::Item::Type(_)
        | syn::Item::Union(_)
        | syn::Item::Use(_)
        | syn::Item::Verbatim(_)
        | syn::Item::__TestExhaustive(_) => {
            abort!(
                item,
                "export attribute can only be applied to impl blocks and functions"
            );
        }
    }
}
