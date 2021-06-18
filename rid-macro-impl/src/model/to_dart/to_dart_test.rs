use crate::attrs::StructConfig;

use super::{render_to_dart, DartRenderImplConfig};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

fn render(input: proc_macro2::TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::Item>(input).unwrap();

    match item {
        Item::Struct(struct_item) => {
            let struct_config = StructConfig::from(&struct_item);
            render_to_dart(
                &struct_item,
                struct_config,
                DartRenderImplConfig::for_tests(),
            )
        }
        _ => panic!("Trying to parse a struct here"),
    }
}

// TODO: write tests
