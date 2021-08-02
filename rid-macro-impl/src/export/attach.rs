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

use crate::{attrs::parse_rid_attrs, common::abort};
use quote::{format_ident, quote};

use proc_macro2::TokenStream;
use render_dart::render_instance_method_extension;
use syn::Ident;

fn unpack_tuples<T, U>(tpls: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
    let mut xs: Vec<T> = Vec::with_capacity(tpls.len());
    let mut ys: Vec<U> = Vec::with_capacity(tpls.len());
    for (x, y) in tpls {
        xs.push(x);
        ys.push(y);
    }

    (xs, ys)
}

pub struct ExportConfig {
    render_dart_extension: bool,
    render_vec_access: bool,
    render_dart_free_extension: bool,
    render_frees: bool,
    include_ffi: bool,
    render_utils_module: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            render_dart_extension: true,
            render_vec_access: true,
            render_dart_free_extension: true,
            render_frees: true,
            include_ffi: true,
            render_utils_module: true,
        }
    }
}

impl ExportConfig {
    pub fn for_tests() -> Self {
        Self {
            render_dart_extension: false,
            render_vec_access: false,
            render_dart_free_extension: false,
            render_frees: false,
            include_ffi: false,
            render_utils_module: false,
        }
    }
}

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
            let mut frees = HashMap::<String, PointerTypeAlias>::new();
            let mut vec_accesses = HashMap::<Ident, VecAccess>::new();
            let rust_fn_tokens = &parsed
                .methods
                .iter()
                .map(|x| {
                    let render_rust::RenderedFunctionExport {
                        tokens,
                        ptr_type_aliases,
                        vec_access,
                    } = render_rust::render_function_export(
                        x,
                        Some(parsed.ty.rust_ident().clone()),
                        Some(RenderFunctionExportConfig {
                            include_ffi: config.include_ffi,
                            ..Default::default()
                        }),
                    );
                    for PointerTypeAlias {
                        alias,
                        typedef,
                        type_name,
                        needs_free,
                    } in ptr_type_aliases
                    {
                        ptr_type_aliases_map
                            .insert(alias.to_string(), typedef.clone());
                        if needs_free {
                            frees.insert(
                                type_name.clone(),
                                PointerTypeAlias {
                                    alias,
                                    typedef,
                                    type_name,
                                    needs_free,
                                },
                            );
                        }
                    }
                    vec_access.map(|x| {
                        vec_accesses.insert(x.vec_type.rust_ident().clone(), x)
                    });

                    tokens
                })
                .collect::<Vec<TokenStream>>();

            // Make sure we name the module differently for structs that have multiple impl blocks
            let module_ident = get_state().unique_ident(format_ident!(
                "__rid_{}_impl",
                parsed.ty.rust_ident()
            ));

            let vec_access_tokens = if config.render_vec_access {
                let needed_vec_accesses = get_state().need_implemtation(
                    &ImplementationType::VecAccess,
                    vec_accesses,
                );
                render_vec_accesses(
                    &needed_vec_accesses,
                    parsed.type_infos(),
                    "///",
                )
            } else {
                vec![]
            };

            let rendered_frees = if config.render_frees {
                let needed_frees = get_state().need_implemtation(
                    &ImplementationType::Free,
                    frees.clone(),
                );
                needed_frees
                    .into_iter()
                    .map(|x| x.render_free(ffi_prelude()))
                    .collect()
            } else {
                vec![]
            };

            let (infos, free_tokens) = unpack_tuples(rendered_frees);

            // TODO: non-instance method strings
            let dart_free_extensions_tokens =
                if config.render_frees && config.render_dart_free_extension {
                    infos
                        .into_iter()
                        .map(|RenderedTypeAliasInfo { alias, fn_ident }| {
                            parsed.ty.render_dart_dispose_extension(
                                fn_ident, &alias, "///",
                            )
                        })
                        .collect()
                } else {
                    vec![]
                };

            let dart_extension_tokens = if config.render_dart_extension {
                render_instance_method_extension(&parsed, None)
            } else {
                TokenStream::new()
            };

            let ptr_typedef_tokens: Vec<&TokenStream> =
                ptr_type_aliases_map.values().collect();

            let utils_module =
                utils_module_tokens_if(config.render_utils_module);

            quote! {
                #[allow(non_snake_case)]
                mod #module_ident {
                    use super::*;
                    #(#ptr_typedef_tokens)*
                    #dart_extension_tokens
                    #(#rust_fn_tokens)*
                    #(#vec_access_tokens)*
                    #(#dart_free_extensions_tokens)*
                    #(#free_tokens)*
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

#[cfg(test)]
mod tests {}
