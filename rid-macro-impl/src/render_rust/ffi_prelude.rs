use proc_macro2::TokenStream;
use quote::quote;

pub fn allow_prelude() -> TokenStream {
    quote! { #[allow(non_snake_case, non_camel_case_types, unused_imports)] }
}

pub fn ffi_prelude() -> TokenStream {
    let allow = allow_prelude();
    quote! {
        #[no_mangle]
        #allow
        pub extern "C"
    }
}
