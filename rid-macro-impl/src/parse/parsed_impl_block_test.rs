use super::rust_type::{Primitive, RustType, TypeKind, Value};
use crate::attrs::{self, ImplBlockConfig};
use assert_matches::assert_matches;
use attrs::{parse_rid_attrs, Category, TypeInfo};

use super::*;
use quote::quote;

fn parse(input: proc_macro2::TokenStream) -> ParsedImplBlock {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Impl(item) => {
            let config = ImplBlockConfig::from(&item);
            ParsedImplBlock::new(item, config)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

mod self_aliasing {
    use super::*;
    #[test]
    fn impl_block_with_new_returning_self() {
        let mystruct_str = "MyStruct".to_string();

        let ParsedImplBlock { ty, methods, .. } = parse(quote! {
            #[rid::export]
            #[rid::structs(MyStruct)]
            impl MyStruct {
                #[rid::export]
                pub fn new(id: u8) -> Self {
                    Self { id }
                }
            }
        });
        assert_eq!(ty.ident().to_string(), "RawMyStruct", "return ident");
        assert_eq!(
            ty.rust_ident().to_string(),
            "MyStruct",
            "return rust ident"
        );
        assert_eq!(ty.reference, ParsedReference::Owned, "return reference");
        assert_matches!(
            ty.kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct,
                    typedef,
                },
                mystruct_str
            ))
        );

        let ParsedFunction {
            fn_ident,
            fn_ident_alias,
            receiver,
            args,
            return_arg:
                RustType {
                    kind: ret_ty,
                    reference,
                    ..
                },
            config: _,
            ..
        } = &methods[0];

        assert_eq!(fn_ident.to_string(), "new", "function name");
        assert_eq!(fn_ident_alias, &None, "no export ident");
        assert_eq!(*receiver, None, "no receiver");
        assert_eq!(args.len(), 1, "one");

        assert_matches!(
            &args[0],
            RustType {
                kind: TypeKind::Primitive(Primitive::U8),
                reference: ParsedReference::Owned,
                ..
            }
        );

        assert_matches!(
            &ret_ty ,
            TypeKind::Value(Value::Custom(TypeInfo { key: _, cat, .. }, name)) => {
                assert_eq!(
                    (cat, name.as_str()),
                    (&attrs::Category::Struct, "MyStruct"),
                    "custom return type"
                );
            }
        );
    }
}

mod method_exports {
    use super::*;

    #[test]
    fn impl_block_with_four_methods_three_with_rid_export_attr_new_aliased_to_init_model(
    ) {
        let mystruct_str = "MyStruct".to_string();

        let ParsedImplBlock {
            ty: owner_ty,
            methods,
            ..
        } = parse(quote! {
            #[rid::export]
            impl MyStruct {
                #[rid::export(initModel)]
                pub fn new(id: u8) -> Self {
                    Self { id }
                }
                #[rid::export]
                pub fn get_id(&self) -> u8 {
                    self.id
                }
                #[rid::export]
                pub fn set_id(&mut self, id: u8) {
                    self.id = id;
                }
                pub fn inc_id(&mut self, id: u8) {
                    self.id += 1;
                }
            }
        });

        assert_eq!(owner_ty.ident().to_string(), "RawMyStruct", "return ident");
        assert_eq!(
            owner_ty.rust_ident().to_string(),
            "MyStruct",
            "return rust ident"
        );
        assert_eq!(
            owner_ty.reference,
            ParsedReference::Owned,
            "return reference"
        );
        assert_matches!(
            &owner_ty.kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct,
                    typedef,
                },
                mystruct_str
            ))
        );
        assert_eq!(methods.len(), 3, "exports 3 methods");

        // First Method: pub fn new(id: u8) -> Self
        let ParsedFunction {
            fn_ident,
            fn_ident_alias,
            receiver,
            args,
            return_arg: RustType { kind: ret_kind, .. },
            config: _,
            ..
        } = &methods[0];

        assert_eq!(fn_ident.to_string(), "new", "function ident");
        assert_eq!(
            fn_ident_alias.as_ref().unwrap().to_string(),
            "initModel",
            "'initModel' export ident"
        );
        assert_eq!(
            ret_kind, &owner_ty.kind,
            "new() -> Self return type is owning struct"
        );
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].kind,
            TypeKind::Primitive(Primitive::U8),
            "first arg u8"
        );
        assert_eq!(receiver, &None, "no receiver");

        // Second Method: pub fn get_id(&self) -> u8
        let ParsedFunction {
            fn_ident,
            fn_ident_alias,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            config: _,
            ..
        } = &methods[1];

        assert_eq!(fn_ident.to_string(), "get_id", "function ident");
        assert_eq!(fn_ident_alias, &None, "no export ident");
        assert_eq!(args.len(), 0, "no arg");
        assert_matches!(
            receiver,
            Some(ParsedReceiver {
                reference: ParsedReference::Ref(None),
                info: _,
            }),
            "ref receiver"
        );
        assert_eq!(*ret_ty, TypeKind::Primitive(Primitive::U8), "returns u8");

        // Third Method: pub fn set_id(&mut self, id: u8)
        let ParsedFunction {
            fn_ident,
            fn_ident_alias,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
            config: _,
            ..
        } = &methods[2];
        assert_eq!(fn_ident.to_string(), "set_id", "function ident");
        assert_eq!(fn_ident_alias, &None, "no export ident");
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].kind,
            TypeKind::Primitive(Primitive::U8),
            "first arg u8"
        );
        assert_matches!(
            receiver,
            Some(ParsedReceiver {
                reference: ParsedReference::RefMut(None),
                info: _,
            }),
            "ref mut receiver"
        );
        assert_eq!(ret_ty, &TypeKind::Unit, "returns ()");
    }
}
