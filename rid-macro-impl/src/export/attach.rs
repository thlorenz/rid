use crate::{
    parse::{ParsedFunction, ParsedImplBlock},
    render_rust::render_function_export,
};

use crate::{attrs::parse_rid_attrs, common::abort};
use quote::quote;

use proc_macro2::TokenStream;

pub fn rid_export_impl(
    item: syn::Item,
    _args: syn::AttributeArgs,
) -> TokenStream {
    match item {
        syn::Item::Impl(item) => {
            let attrs = parse_rid_attrs(&item.attrs);
            let parsed = ParsedImplBlock::new(item, &attrs);
            let tokens = &parsed
                .methods
                .iter()
                .map(|x| {
                    render_function_export(
                        x,
                        Some(parsed.ty.ident.clone()),
                        Default::default(),
                    )
                })
                .collect::<Vec<TokenStream>>();
            quote! { #(#tokens)* }
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
