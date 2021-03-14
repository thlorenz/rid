use proc_macro2::TokenStream;
use quote::quote;

pub fn rid_export_impl(item: syn::Item, args: syn::AttributeArgs) -> TokenStream {
    (quote! { fn some_func() {} }).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn struct_impl() {
        let attrs = TokenStream::new();
        let input: TokenStream = quote! {
          impl MyStruct {}
        }
        .into();

        let item = syn::parse2::<syn::Item>(input).unwrap();
        // let args = syn::parse2::<syn::AttributeArgs>(attrs).unwrap();
        let args = syn::AttributeArgs::new();

        let res = rid_export_impl(item, args);

        eprintln!("{}", res);
    }
}
