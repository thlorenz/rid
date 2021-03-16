use super::{ParsedFunction, ParsedImplBlock};
use crate::attrs;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::quote;

pub fn rid_export_impl(item: syn::Item, args: syn::AttributeArgs) -> TokenStream {
    match item {
        syn::Item::Impl(item) => {
            let attrs = attrs::parse_rid_attrs(&item.attrs);

            // TODO: ignore if no #[rid(export)] attr present
            let parsed = ParsedImplBlock::new(item, &attrs);
            eprintln!("impl: {:#?}", parsed);
            TokenStream::new()
        }
        syn::Item::Fn(syn::ItemFn {
            attrs, // Vec<Attribute>,
            vis,   // Visibility,
            sig,   // Signature,
            block, // Box<Block>,
        }) => {
            let attrs = attrs::parse_rid_attrs(&attrs);
            let parsed = ParsedFunction::new(sig, attrs);
            eprintln!("impl: {:#?}", parsed);
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

    #[test]
    fn struct_impl() {
        let attrs = TokenStream::new();
        let input: TokenStream = quote! {
          impl MyStruct {
              pub fn new(id: u8, title: String) -> Self {
                  Self { id, title }
              }

              pub fn dispose(msg: String) {}
          }
        }
        .into();

        let item = syn::parse2::<syn::Item>(input).unwrap();
        // let args = syn::parse2::<syn::AttributeArgs>(attrs).unwrap();
        let args = syn::AttributeArgs::new();

        let res = rid_export_impl(item, args);

        eprintln!("{}", res);
    }
}
