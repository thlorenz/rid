use std::collections::HashMap;

use crate::{
    common::state::{get_state, ImplementationType},
    parse::{ParsedFunction, ParsedImplBlock},
    render_common::{render_vec_accesses, VecAccess},
    render_dart,
    render_rust::{self, ffi_prelude, TypeAlias},
};

use crate::{attrs::parse_rid_attrs, common::abort};
use quote::{format_ident, quote};

use proc_macro2::TokenStream;
use render_dart::render_instance_method_extension;
use syn::Ident;

pub fn rid_export_impl(
    item: syn::Item,
    _args: syn::AttributeArgs,
) -> TokenStream {
    match item {
        syn::Item::Impl(item) => {
            let attrs = parse_rid_attrs(&item.attrs);
            let parsed = ParsedImplBlock::new(item, &attrs);
            let mut typedefs = HashMap::<Ident, TokenStream>::new();
            let mut frees = HashMap::<String, TypeAlias>::new();
            let mut vec_accesses = HashMap::<Ident, VecAccess>::new();
            let rust_fn_tokens = &parsed
                .methods
                .iter()
                .map(|x| {
                    let render_rust::RenderedFunctionExport {
                        tokens,
                        type_aliases,
                        vec_access,
                    } = render_rust::render_function_export(
                        x,
                        Some(parsed.ty.ident.clone()),
                        Default::default(),
                    );
                    for TypeAlias {
                        alias,
                        typedef,
                        type_name,
                        needs_free,
                    } in type_aliases
                    {
                        typedefs.insert(alias.clone(), typedef.clone());
                        if needs_free {
                            frees.insert(
                                type_name.clone(),
                                TypeAlias {
                                    alias,
                                    typedef,
                                    type_name,
                                    needs_free,
                                },
                            );
                        }
                    }
                    vec_access.map(|x| {
                        vec_accesses.insert(x.vec_type.ident.clone(), x)
                    });

                    tokens
                })
                .collect::<Vec<TokenStream>>();

            // TODO: non-instance method strings
            let dart_extension_tokens =
                render_instance_method_extension(&parsed, None);

            // Make sure we name the module differently for structs that have multiple impl blocks
            let module_ident = get_state()
                .unique_ident(format_ident!("__rid_{}_impl", parsed.ty.ident));

            let needed_vec_accesses = get_state().need_implemtation(
                &ImplementationType::VecAccess,
                vec_accesses,
            );

            let needed_frees = get_state()
                .need_implemtation(&ImplementationType::Free, frees.clone());
            let free_tokens = needed_frees
                .into_iter()
                .map(|x| x.render_free(ffi_prelude()));

            let vec_access_tokens =
                render_vec_accesses(&needed_vec_accesses, "///");

            let typedef_tokens: Vec<&TokenStream> = typedefs.values().collect();

            quote! {
                #[allow(non_snake_case)]
                mod #module_ident {
                    use super::*;
                    #(#typedef_tokens)*
                    #dart_extension_tokens
                    #(#rust_fn_tokens)*
                    #(#vec_access_tokens)*
                    #(#free_tokens)*
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
            // An alternative would be to track already exported functions in our state via an id
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
mod tests {
    use super::*;
    use quote::quote;

    // #[test]
    #[allow(dead_code)]
    fn struct_impl() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
          #[rid::export]
          impl MyStruct {
              #[rid::export]
              pub fn new(id: u8, title: String) -> Self {
                  Self { id, title }
              }

              #[rid::export]
              pub fn dispose(msg: String) {}
          }
        }
        .into();

        let item = syn::parse2::<syn::Item>(input).unwrap();
        let args = syn::AttributeArgs::new();

        let res = rid_export_impl(item, args);

        eprintln!("res: {}", res);
    }
}
