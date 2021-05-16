use std::collections::HashMap;

use quote::quote;
use syn::Item;

use crate::{
    attrs::{self, TypeInfo, TypeInfoMap},
    parse::ParsedFunction,
};

use super::{Derive, RidAttr};

fn parse(input: proc_macro2::TokenStream) -> Derive {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        Item::Struct(struct_item) => {
            attrs::parse_derive_attrs(&struct_item.attrs)
        }
        _ => panic!(
            "Unexpected item, we're trying to parse attributes of structs here"
        ),
    }
}

mod derive_present {
    use super::*;

    #[test]
    fn struct_deriving_debug_only() {
        let derive = parse(quote! {
            #[derive(Debug)]
            struct Model {}
        });
        assert_eq!(
            derive,
            Derive {
                debug: true,
                clone: false
            },
            "debug"
        )
    }

    #[test]
    fn struct_deriving_clone_only() {
        let derive = parse(quote! {
            #[derive(Clone)]
            struct Model {}
        });
        assert_eq!(
            derive,
            Derive {
                debug: false,
                clone: true
            },
            "clone"
        )
    }

    #[test]
    fn struct_deriving_debug_and_clone() {
        //  single derive
        {
            let derive = parse(quote! {
                #[derive(Debug, Clone)]
                struct Model {}
            });
            assert_eq!(
                derive,
                Derive {
                    debug: true,
                    clone: true
                },
                "debug+clone"
            )
        }
        //  separate derives
        {
            let derive = parse(quote! {
                #[derive(Debug)]
                #[derive(Clone)]
                struct Model {}
            });
            assert_eq!(
                derive,
                Derive {
                    debug: true,
                    clone: true
                },
                "debug+clone"
            )
        }
    }
}
