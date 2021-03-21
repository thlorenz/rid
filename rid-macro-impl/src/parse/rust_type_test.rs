use assert_matches::assert_matches;
use attrs::TypeInfoMap;

use crate::{
    attrs,
    attrs::{Category, TypeInfo},
};
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
            let attrs = attrs::parse_rid_attrs(&attrs);
            let type_infos = TypeInfoMap::from(attrs.as_slice());
            let arg = sig.inputs.iter().next().unwrap();

            match arg {
                syn::FnArg::Typed(syn::PatType { ty, .. }) => {
                    RustType::from_boxed_type(ty.clone(), &type_infos)
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
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "u8", "ident");
        assert_matches!(reference, ParsedReference::Owned);
        assert_matches!(kind, TypeKind::Primitive(Primitive::U8));
    }

    #[test]
    fn ref_u8() {
        let res = parse(quote! { fn f(x: &u8) {} });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "u8", "ident");
        assert_matches!(reference, ParsedReference::Ref(None));
        assert_matches!(kind, TypeKind::Primitive(Primitive::U8));
    }

    #[test]
    fn ref_mut_u8() {
        let res = parse(quote! { fn f(x: &mut u8) {} });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "u8", "ident");
        assert_matches!(reference, ParsedReference::RefMut(None));
        assert_matches!(kind, TypeKind::Primitive(Primitive::U8));
    }

    #[test]
    fn ref_i64() {
        let res = parse(quote! { fn f(x: &i64) {} });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "i64", "ident");
        assert_matches!(reference, ParsedReference::Ref(None));
        assert_matches!(kind, TypeKind::Primitive(Primitive::I64));
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
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "String", "ident");
        assert_matches!(reference, ParsedReference::Owned);
        assert_matches!(kind, TypeKind::Value(Value::String));
    }

    #[test]
    fn ref_str() {
        let res = parse(quote! { fn f(x: &str) {} });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "str", "ident");
        assert_matches!(reference, ParsedReference::Ref(None));
        assert_matches!(kind, TypeKind::Value(Value::Str));
    }

    #[test]
    fn ref_mut_cstring() {
        let res = parse(quote! { fn f(x: &mut CString) {} });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "CString", "ident");
        assert_matches!(reference, ParsedReference::RefMut(None));
        assert_matches!(kind, TypeKind::Value(Value::CString));
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
            #[rid(types = { Model: Struct })]
            fn f(x: Model) {}
        });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Model", "ident");
        assert_matches!(reference, ParsedReference::Owned);
        assert_matches!(
            kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct
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
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Model", "ident");
        assert_matches!(reference, ParsedReference::Owned);
        assert_matches!(kind, TypeKind::Unknown)
    }
    #[test]
    fn ref_model() {
        let model_str = "Model".to_string();
        let res = parse(quote! {
            #[rid(types = { Model: Struct })]
            fn f(x: &Model) {}
        });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Model", "ident");
        assert_matches!(reference, ParsedReference::Ref(None));
        assert_matches!(
            kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct
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
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Vec", "ident");
        assert_matches!(reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner) = kind {
            assert_matches!(composite, Composite::Vec);
            let RustType {
                ident,
                kind,
                reference,
            } = *inner.expect("has inner rust type");

            assert_eq!(ident.to_string(), "u8", "ident");
            assert_matches!(reference, ParsedReference::Owned);
            assert_matches!(kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }

    #[test]
    fn vec_ref_u8() {
        let res = parse(quote! {
            fn f(x: Vec<&u8>) {}
        });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Vec", "ident");
        assert_matches!(reference, ParsedReference::Owned);

        if let TypeKind::Composite(composite, inner) = kind {
            assert_matches!(composite, Composite::Vec);
            let RustType {
                ident,
                kind,
                reference,
            } = *inner.expect("has inner rust type");

            assert_eq!(ident.to_string(), "u8", "ident");
            assert_matches!(reference, ParsedReference::Ref(None));
            assert_matches!(kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }

    #[test]
    fn ref_mut_vec_ref_todo() {
        let res = parse(quote! {
            #[rid(types = { Todo: Struct })]
            fn f(x: &mut Vec<&Todo>) {}
        });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Vec", "ident");
        assert_matches!(reference, ParsedReference::RefMut(None));

        if let TypeKind::Composite(composite, inner) = kind {
            let todo_str = "Todo".to_string();
            assert_matches!(composite, Composite::Vec);
            let RustType {
                ident,
                kind,
                reference,
            } = *inner.expect("has inner rust type");

            assert_eq!(ident.to_string(), "Todo", "ident");
            assert_matches!(reference, ParsedReference::Ref(None));
            assert_matches!(
                kind,
                TypeKind::Value(Value::Custom(
                    TypeInfo {
                        key,
                        cat: Category::Struct
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
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Vec", "ident");
        assert_matches!(reference, ParsedReference::Ref(None));

        if let TypeKind::Composite(composite, inner) = kind {
            let todo_str = "Todo".to_string();
            assert_matches!(composite, Composite::Vec);
            let RustType {
                ident,
                kind,
                reference,
            } = *inner.expect("has inner rust type");

            assert_eq!(ident.to_string(), "Todo", "ident");
            assert_matches!(reference, ParsedReference::Ref(None));
            assert_matches!(kind, TypeKind::Unknown);
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
            #[rid(types = { Cont: Struct })]
            fn f(x: &Cont<u8>) {}
        });
        let RustType {
            ident,
            kind,
            reference,
        } = res.expect("extracts rust type");

        assert_eq!(ident.to_string(), "Cont", "ident");
        assert_matches!(reference, ParsedReference::Ref(None));

        if let TypeKind::Composite(composite, inner) = kind {
            assert_matches!(
                composite,
                Composite::Custom(
                    TypeInfo {
                        key,
                        cat: Category::Struct
                    },
                    cont_str
                )
            );
            let RustType {
                ident,
                kind,
                reference,
            } = *inner.expect("has inner rust type");

            assert_eq!(ident.to_string(), "u8", "ident");
            assert_matches!(reference, ParsedReference::Owned);
            assert_matches!(kind, TypeKind::Primitive(Primitive::U8));
        } else {
            panic!("expected composite")
        };
    }
}
