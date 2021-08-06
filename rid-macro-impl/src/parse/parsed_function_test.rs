use assert_matches::assert_matches;
use attrs::{parse_rid_attrs, FunctionConfig};

use super::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedFunction, ParsedReceiver, ParsedReference,
};
use crate::{
    attrs,
    attrs::{Category, TypeInfo, TypeInfoMap},
};
use quote::quote;

fn parse(input: proc_macro2::TokenStream) -> ParsedFunction {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Fn(syn::ItemFn {
            attrs,    // Vec<Attribute>,
            vis: _,   // Visibility,
            sig,      // Signature,
            block: _, // Box<Block>,
        }) => {
            let rid_attrs = parse_rid_attrs(&attrs);
            let config = FunctionConfig::new(&rid_attrs, None);
            ParsedFunction::new(sig, config, None)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

mod base_case {
    use super::*;
    #[test]
    fn void_function_no_args() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            ..
        } = parse(quote! {
            fn me() {}
        });

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 0, "empty args");
        assert_eq!(ret_ty, TypeKind::Unit, "returns ()");
    }
}

mod return_arg {
    use super::*;

    #[test]
    fn u8_function_no_args() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            ..
        } = parse(quote! {
            fn me() -> u8 {}
        });

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 0, "empty args");
        assert_eq!(ret_ty, TypeKind::Primitive(Primitive::U8), "returns u8");
    }

    #[test]
    fn custom_return_type_ref() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg:
                RustType {
                    kind: ret_ty,
                    reference,
                    ..
                },
            ..
        } = parse(quote! {
            #[rid::structs(Todo)]
            fn get_todo() -> &Todo {}
        });

        assert_eq!(fn_ident.to_string(), "get_todo", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 0, "no arg");

        assert_matches!(
            &ret_ty ,
            TypeKind::Value(Value::Custom(TypeInfo { key: _, cat, .. }, name)) => {
                assert_eq!(
                    (cat, name.as_str()),
                    (&attrs::Category::Struct, "Todo"),
                    "custom return type"
                );
            }
        );
        assert_matches!(reference, ParsedReference::Ref(None));
    }

    #[test]
    fn custom_return_type_ref_with_lifetime() {
        let ParsedFunction {
            return_arg: RustType { reference, .. },
            ..
        } = parse(quote! {
            #[rid(types = { Todo: Struct })]
            fn get_todo() -> &'a Todo {}
        });

        assert_matches!(reference, ParsedReference::Ref(Some(ident)) => {
            assert_eq!(ident.to_string(), "a");
        });
    }

    #[test]
    fn vec_custom_type_ref_return() {
        let ParsedFunction { return_arg, .. } = parse(quote! {
            #[rid::structs(Todo)]
            fn filtered_todos() -> Vec<&Todo> {}
        });

        assert_eq!(
            return_arg.dart_wrapper_rust_ident().to_string(),
            "RawVec",
            "ident"
        );
        assert_eq!(return_arg.rust_ident().to_string(), "Vec", "rust ident");
        assert_matches!(return_arg.reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner, _) = return_arg.kind {
            let todo_str = "Todo".to_string();
            assert_matches!(composite, Composite::Vec);
            let inner = inner.expect("has inner rust type");

            assert_eq!(
                inner.dart_wrapper_rust_ident().to_string(),
                "RawTodo",
                "ident"
            );
            assert_eq!(inner.rust_ident().to_string(), "Todo", "rust ident");
            assert_matches!(inner.reference, ParsedReference::Ref(None));
            assert_matches!(
                inner.kind,
                TypeKind::Value(Value::Custom(
                    TypeInfo {
                        key,
                        cat: Category::Struct,
                        typedef,
                    },
                    todo_str
                ))
            )
        } else {
            panic!("expected composite")
        };
    }
}

mod multiple_args {
    use super::*;

    #[test]
    fn string_function_i32_and_string_arg() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            ..
        } = parse(quote! {
            fn me(id: i32, s: String) -> String {}
        });

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 2, "two args");
        assert_eq!(
            args[0].kind,
            TypeKind::Primitive(Primitive::I32),
            "first arg i32"
        );
        assert_eq!(
            args[1].kind,
            TypeKind::Value(Value::String),
            "second arg String"
        );
        assert_eq!(ret_ty, TypeKind::Value(Value::String), "returns String");
    }
}

mod receiver {
    use std::collections::HashMap;

    use super::*;

    fn parse(input: proc_macro2::TokenStream, owner: &str) -> ParsedFunction {
        let type_info = TypeInfo::from((owner, attrs::Category::Struct));
        let mut map = HashMap::new();
        map.insert(owner.to_string(), type_info.clone());
        let type_info_map = TypeInfoMap(map);
        let owner = Some((&type_info.key, &type_info_map));

        let item = syn::parse2::<syn::Item>(input).unwrap();
        match item {
            syn::Item::Fn(syn::ItemFn {
                attrs,    // Vec<Attribute>,
                vis: _,   // Visibility,
                sig,      // Signature,
                block: _, // Box<Block>,
            }) => {
                let rid_attrs = parse_rid_attrs(&attrs);
                let owner = Some((&type_info.key, &type_info_map));
                let config = FunctionConfig::new(&rid_attrs, owner);
                ParsedFunction::new(sig, config, owner)
            }
            _ => {
                panic!("Unexpected item, we're trying to parse functions here")
            }
        }
    }

    #[test]
    fn void_function_no_args_ref_self() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            ..
        } = parse(
            quote! {
                fn me(&self) {}
            },
            "MyStruct",
        );

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_matches!(
            receiver,
            Some(ParsedReceiver {
                reference: ParsedReference::Ref(None),
                info: _,
            }),
            "no ref receiver"
        );
        assert_eq!(args.len(), 0, "empty args");
        assert_eq!(ret_ty, TypeKind::Unit, "returns ()");
    }

    #[test]
    fn void_function_one_arg_ref_mut_self() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            ..
        } = parse(
            quote! {
                fn me(&mut self, id: usize) {}
            },
            "MyStruct",
        );

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_matches!(
            receiver,
            Some(ParsedReceiver {
                reference: ParsedReference::RefMut(None),
                info: _,
            }),
            "ref mut receiver"
        );
        assert_eq!(args.len(), 1, "empty args");
        assert_eq!(
            args[0].kind,
            TypeKind::Primitive(Primitive::USize),
            "first arg of type usize"
        );
        assert_eq!(ret_ty, TypeKind::Unit, "returns ()");
    }

    #[test]
    fn void_function_no_args_owned_self() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            ..
        } = parse(
            quote! {
                fn me(self) {}
            },
            "MyStruct",
        );

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_matches!(
            receiver,
            Some(ParsedReceiver {
                reference: ParsedReference::Owned,
                info: _,
            }),
            "owned receiver"
        );
        assert_eq!(args.len(), 0, "empty args");
        assert_eq!(ret_ty, TypeKind::Unit, "returns ()");
    }
}
