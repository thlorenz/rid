use std::collections::HashMap;

use quote::quote;

use crate::{
    attrs::{self, TypeInfo, TypeInfoMap},
    parse::ParsedFunction,
};

use super::RidAttr;

fn parse(input: proc_macro2::TokenStream) -> Vec<RidAttr> {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Fn(syn::ItemFn { attrs, sig, .. }) => {
            attrs::parse_rid_attrs(&attrs)
        }
        _ => panic!("Unexpected item, we're trying to parse attributes of functions here"),
    }
}

mod type_annotations {
    use super::*;

    #[test]
    fn structs_one_item_and_enums_two_items() {
        let attrs = parse(quote! {
            #[rid::structs(Item)]
            #[rid::enums(Enum1, Enum2)]
            fn noop(){}
        });
        let attrs = format!("{:?}", attrs);
        assert_eq!(
            attrs,
            "[Structs(Ident(structs), [Ident(Item)]), Enums(Ident(enums), [Ident(Enum1), Ident(Enum2)])]"
        );
    }
}

mod exports {
    use super::*;

    #[test]
    fn single_export_unaliased() {
        let attrs = parse(quote! {
            #[rid::export]
            fn noop(){}
        });
        let attrs = format!("{:?}", attrs);
        assert_eq!(attrs, "[Export(Ident(export), None)]");
    }

    #[test]
    fn single_export_aliased() {
        let attrs = parse(quote! {
            #[rid::export(noopAlias)]
            fn noop(){}
        });
        let attrs = format!("{:?}", attrs);
        assert_eq!(attrs, "[Export(Ident(export), Some(Ident(noopAlias)))]");
    }
}

mod message {
    use super::*;

    #[test]
    fn message_to_todo() {
        let attrs = parse(quote! {
            #[rid::message(Todo, Reply)]
            fn noop(){}
        });
        let attrs = format!("{:?}", attrs);
        assert_eq!(attrs, "[Message(Ident(message), Ident(Todo))]");
    }
}
