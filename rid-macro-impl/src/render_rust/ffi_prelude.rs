use proc_macro2::TokenStream;
use quote::quote;

pub fn ffi_prelude() -> TokenStream {
    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C"
    }
}
