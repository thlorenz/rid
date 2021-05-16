use crate::{
    attrs::{parse_derive_attrs, StructConfig},
    parse::ParsedStruct,
};

use super::{render_to_dart, DartRenderImplConfig};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

fn render(input: proc_macro2::TokenStream, is_store: bool) -> TokenStream {
    let item = syn::parse2::<syn::Item>(input).unwrap();

    match item {
        Item::Struct(struct_item) => {
            let struct_config = StructConfig::from(&struct_item);
            let derive = parse_derive_attrs(&struct_item.attrs);
            let parsed_struct = ParsedStruct::new(
                &struct_item,
                &struct_item.ident,
                struct_config,
            );
            render_to_dart(
                &parsed_struct,
                is_store,
                &derive,
                DartRenderImplConfig::for_tests(),
            )
        }
        _ => panic!("Trying to parse a struct here"),
    }
}

// TODO: write tests
