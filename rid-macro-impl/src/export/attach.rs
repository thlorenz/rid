use super::{ParsedFunction, ParsedImplBlock};

use crate::attrs;
use crate::common::abort;

use proc_macro2::TokenStream;

pub fn rid_export_impl(item: syn::Item, _args: syn::AttributeArgs) -> TokenStream {
    match item {
        syn::Item::Impl(item) => {
            let attrs = attrs::parse_rid_attrs(&item.attrs);
            if attrs.iter().any(|x| x.is_export()) {
                let _parsed = ParsedImplBlock::new(item, &attrs);
                todo!("convert parsed impl block to rendered wrapper")
            } else {
                TokenStream::new()
            }
        }
        syn::Item::Fn(syn::ItemFn {
            attrs,    // Vec<Attribute>,
            vis: _,   // Visibility,
            sig,      // Signature,
            block: _, // Box<Block>,
        }) => {
            let attrs = attrs::parse_rid_attrs(&attrs);
            if attrs.iter().any(|x| x.is_export()) {
                let _parsed = ParsedFunction::new(sig, &attrs, None);
                todo!("convert parsed function to rendered wrapper")
            } else {
                TokenStream::new()
            }
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

        eprintln!("{}", res);
    }
}
