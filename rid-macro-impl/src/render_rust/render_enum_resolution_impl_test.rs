use assert_matches::assert_matches;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Item};

use crate::{common::dump_tokens, parse::ParsedEnum};

fn render(input: TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        Item::Enum(item) => {
            ParsedEnum::from(&item).render_enum_resolution_impl()
        }
        _ => panic!("Unexpected item, we're trying to parse enums here"),
    }
}

mod c_style_enums {
    use super::*;

    #[test]
    fn render_c_style_enum_two_variants() {
        let ident_one: Ident = format_ident!("One");
        let ident_two: Ident = format_ident!("Two");
        let res = render(quote! {
            enum Count { One, Two }
        });
        let expected = quote! {
            impl Count {
                pub fn _rid_from_discriminant<T>(discriminant: T) -> Self
                where
                    T: Into<i32> + Sized,
                {
                    match discriminant.into() {
                        0 => Self::One,
                        1 => Self::Two,
                        n => panic!("enum Count does not include discriminant {}", n),
                    }
                }
                pub fn _rid_into_discriminant(&self) -> i32 {
                    match self {
                        Self::One => 0,
                        Self::Two => 1,
                    }
                }
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
    }
}
