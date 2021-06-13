use super::{rid_dart_object_impl, DartObjectImplConfig};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

fn render(input: proc_macro2::TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::DeriveInput>(input).unwrap();
    rid_dart_object_impl(&item, DartObjectImplConfig::for_tests())
}

// TODO: write tests
