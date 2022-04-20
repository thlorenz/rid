use assert_matches::assert_matches;

use crate::attrs::{parse_rid_attrs, Category, FunctionConfig, TypeInfo};
use quote::quote;

use super::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedReference,
};

fn parse(input: proc_macro2::TokenStream) -> Option<RustType> {
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
            let arg = sig.inputs.iter().next().unwrap();

            match arg {
                syn::FnArg::Typed(syn::PatType { ty, .. }) => {
                    RustType::from_boxed_type(ty.clone(), &config.type_infos)
                }
                _ => panic!("Unexpected item, we're trying to parse simple function args here"),
            }
        }
        _ => {
            panic!("Unexpected item, we're trying to parse function args here")
        }
    }
}

// -----------------
// Primitive
// -----------------
mod primitive {
    use super::*;

    #[test]
    fn u8() {
        let res = parse(quote! { fn f(x: u8) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "u8", "ident");
        assert_eq!(ty.rust_ident().to_string(), "u8", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);
        assert_matches!(ty.kind, TypeKind::Primitive(Primitive::U8));
    }

    #[test]
    fn ref_u8() {
        let res = parse(quote! { fn f(x: &u8) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "u8", "ident");
        assert_eq!(ty.rust_ident().to_string(), "u8", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Ref(None));
        assert_matches!(ty.kind, TypeKind::Primitive(Primitive::U8));
    }

    #[test]
    fn ref_mut_u8() {
        let res = parse(quote! { fn f(x: &mut u8) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "u8", "ident");
        assert_eq!(ty.rust_ident().to_string(), "u8", "rust ident");
        assert_matches!(ty.reference, ParsedReference::RefMut(None));
        assert_matches!(ty.kind, TypeKind::Primitive(Primitive::U8));
    }

    #[test]
    fn ref_i64() {
        let res = parse(quote! { fn f(x: &i64) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "i64", "ident");
        assert_eq!(ty.rust_ident().to_string(), "i64", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Ref(None));
        assert_matches!(ty.kind, TypeKind::Primitive(Primitive::I64));
    }
}

// -----------------
// Strings
// -----------------
mod strings {
    use super::*;

    #[test]
    fn string() {
        let res = parse(quote! { fn f(x: String) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "String", "ident");
        assert_eq!(ty.rust_ident().to_string(), "String", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);
        assert_matches!(ty.kind, TypeKind::Value(Value::String));
    }

    #[test]
    fn ref_str() {
        let res = parse(quote! { fn f(x: &str) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "str", "ident");
        assert_eq!(ty.rust_ident().to_string(), "str", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Ref(None));
        assert_matches!(ty.kind, TypeKind::Value(Value::Str));
    }

    #[test]
    fn ref_mut_cstring() {
        let res = parse(quote! { fn f(x: &mut CString) {} });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "CString", "ident");
        assert_eq!(ty.rust_ident().to_string(), "CString", "rust ident");
        assert_matches!(ty.reference, ParsedReference::RefMut(None));
        assert_matches!(ty.kind, TypeKind::Value(Value::CString));
    }
}

// -----------------
// Custom Value Types
// -----------------
mod custom {
    use super::*;

    #[test]
    fn model() {
        let model_str = "Model".to_string();
        let res = parse(quote! {
            #[rid::structs(Model)]
            fn f(x: Model) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "RawModel", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Model", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);
        assert_matches!(
            ty.kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct,
                    typedef,
                },
                model_str
            ))
        )
    }
    #[test]
    fn model_missing_rid_type_info() {
        let res = parse(quote! {
            fn f(x: Model) {}
        });
        let ty = res.expect("extracts rust type");

        // Doesn't alias to RawModel since it doesn't know if it is a struct or not
        assert_eq!(ty.dart_wrapper_rust_string(), "Model", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Model", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);
        assert_matches!(ty.kind, TypeKind::Unknown)
    }
    #[test]
    fn ref_model() {
        let model_str = "Model".to_string();
        let res = parse(quote! {
            #[rid::structs(Model)]
            fn f(x: &Model) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "RawModel", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Model", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Ref(None));
        assert_matches!(
            ty.kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct,
                    typedef,
                },
                model_str
            ))
        )
    }
}

// --------------
// Vec
// --------------
mod vec {
    use super::*;

    #[test]
    fn vec_u8() {
        let res = parse(quote! {
            fn f(x: Vec<u8>) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "RawVec", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Vec", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            assert_matches!(composite, Composite::Vec);
            let inner_ty = inner.expect("has inner rust type");

            assert_eq!(
                inner_ty.dart_wrapper_rust_string(),
                "u8",
                "inner ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "u8",
                "inner rust ident"
            );
            assert_matches!(inner_ty.reference, ParsedReference::Owned);
            assert_matches!(inner_ty.kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }

    #[test]
    fn vec_ref_u8() {
        let res = parse(quote! {
            fn f(x: Vec<&u8>) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "RawVec", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Vec", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            assert_matches!(composite, Composite::Vec);
            let inner_ty = inner.expect("has inner rust type");

            assert_eq!(
                inner_ty.dart_wrapper_rust_string(),
                "u8",
                "inner ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "u8",
                "inner rust ident"
            );
            assert_matches!(inner_ty.reference, ParsedReference::Ref(None));
            assert_matches!(inner_ty.kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }

    #[test]
    fn ref_mut_vec_ref_todo() {
        let res = parse(quote! {
            #[rid::structs(Todo)]
            fn f(x: &mut Vec<&Todo>) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "RawVec", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Vec", "rust ident");
        assert_matches!(ty.reference, ParsedReference::RefMut(None));

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            let todo_str = "Todo".to_string();
            assert_matches!(composite, Composite::Vec);
            let inner_ty = inner.expect("has inner rust type");

            assert_eq!(
                inner_ty.dart_wrapper_rust_string(),
                "RawTodo",
                "inner ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "Todo",
                "inner rust ident"
            );
            assert_matches!(inner_ty.reference, ParsedReference::Ref(None));
            assert_matches!(
                inner_ty.kind,
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

    #[test]
    fn ref_vec_ref_todo_missing_type_annotation() {
        let res = parse(quote! {
            fn f(x: &Vec<&Todo>) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "RawVec", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Vec", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Ref(None));

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            let todo_str = "Todo".to_string();
            assert_matches!(composite, Composite::Vec);
            let inner_ty = inner.expect("has inner rust type");

            // TODO: not yet properly type aliasing argument types
            // see: src/render_rust/render_function_export.rs:95
            // assert_eq!(inner_ty.ident().to_string(), "RawTodo", "inner ident");
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "Todo",
                "inner rust ident"
            );
            assert_matches!(inner_ty.reference, ParsedReference::Ref(None));
            assert_matches!(inner_ty.kind, TypeKind::Unknown);
        } else {
            panic!("expected composite")
        };
    }
}

// -----------------
// Custom Composites
// -----------------

mod custom_composites {
    use super::*;

    #[test]
    fn ref_cont_u8() {
        let cont_str = "Cont".to_string();
        let res = parse(quote! {
            #[rid::structs(Cont)]
            fn f(x: &Cont<u8>) {}
        });
        let ty = res.expect("extracts rust type");

        // TODO(thlorenz): we don't properly alias arg types to Raw* yet,
        // see: src/render_rust/render_function_export.rs:93
        assert_eq!(ty.dart_wrapper_rust_string(), "Cont", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Cont", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Ref(None));

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            assert_matches!(
                composite,
                Composite::Custom(
                    TypeInfo {
                        key,
                        cat: Category::Struct,
                        typedef,
                    },
                    cont_str
                )
            );
            let inner_ty = inner.expect("has inner rust type");

            assert_eq!(
                inner_ty.dart_wrapper_rust_string(),
                "u8",
                "inner ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "u8",
                "inner rust ident"
            );
            assert_matches!(inner_ty.reference, ParsedReference::Owned);
            assert_matches!(inner_ty.kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }
}

// -----------------
// Composite Option
// -----------------
mod composite_option {
    use super::*;

    #[test]
    fn arg_option_ref_todo() {
        let model_str = "Todo".to_string();
        let res = parse(quote! {
            #[rid::structs(Todo)]
            fn f(x: Option<&Todo>) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "Option", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Option", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            assert_matches!(composite, Composite::Option);
            let inner_ty = inner.expect("has inner rust type");

            assert_eq!(
                inner_ty.dart_wrapper_rust_string(),
                "RawTodo",
                "inner ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "Todo",
                "inner rust ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "Todo",
                "inner rust ident"
            );
            assert_matches!(inner_ty.reference, ParsedReference::Ref(_));
            assert_matches!(
                inner_ty.kind,
                TypeKind::Value(Value::Custom(
                    TypeInfo {
                        key,
                        cat: Category::Struct,
                        typedef,
                    },
                    model_str
                ))
            )
        } else {
            panic!("expected composite")
        };
    }

    #[test]
    fn arg_option_u8() {
        let model_str = "Todo".to_string();
        let res = parse(quote! {
            fn f(x: Option<u8>) {}
        });
        let ty = res.expect("extracts rust type");

        assert_eq!(ty.dart_wrapper_rust_string(), "Option", "ident");
        assert_eq!(ty.rust_ident().to_string(), "Option", "rust ident");
        assert_matches!(ty.reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner, _) = ty.kind {
            assert_matches!(composite, Composite::Option);
            let inner_ty = inner.expect("has inner rust type");

            assert_eq!(
                inner_ty.dart_wrapper_rust_string(),
                "u8",
                "inner ident"
            );
            assert_eq!(
                inner_ty.rust_ident().to_string(),
                "u8",
                "inner rust ident"
            );
            assert_eq!(inner_ty.reference, ParsedReference::Owned, "reference");
            assert_matches!(inner_ty.kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }
}
