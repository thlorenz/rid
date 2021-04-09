use super::rust_type::{Primitive, RustType, TypeKind, Value};
use crate::attrs;
use assert_matches::assert_matches;
use attrs::{Category, TypeInfo};

use super::*;
use quote::quote;

fn parse(input: proc_macro2::TokenStream) -> ParsedImplBlock {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Impl(item) => {
            let attrs = attrs::parse_rid_attrs(&item.attrs, None);
            ParsedImplBlock::new(item, &attrs)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

mod self_aliasing {
    use super::*;
    #[test]
    fn impl_block_with_new_returning_self() {
        let mystruct_str = "MyStruct".to_string();

        let ParsedImplBlock {
            ty:
                RustType {
                    ident,
                    kind,
                    reference,
                },
            methods,
        } = parse(quote! {
            #[rid(export)]
            #[rid(types = { MyStruct: Struct })]
            impl MyStruct {
                #[rid(export)]
                pub fn new(id: u8) -> Self {
                    Self { id }
                }
            }
        });
        assert_eq!(ident.to_string(), "MyStruct", "return ident");
        assert_eq!(reference, ParsedReference::Owned, "return reference");
        assert_matches!(
            kind,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct
                },
                mystruct_str
            ))
        );

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
        } = &methods[0];

        assert_eq!(fn_ident.to_string(), "new", "function name");
        assert_eq!(*receiver, None, "no receiver");
        assert_eq!(args.len(), 1, "one");

        assert_matches!(
            &args[0],
            RustType {
                ident,
                kind: TypeKind::Primitive(Primitive::U8),
                reference: ParsedReference::Owned,
            }
        );

        assert_matches!(
            &ret_ty ,
            TypeKind::Value(Value::Custom(TypeInfo { key: _, cat }, name)) => {
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
    fn impl_block_with_four_methods_three_with_rid_export_attr() {
        let mystruct_str = "MyStruct".to_string();

        let ParsedImplBlock {
            ty:
                RustType {
                    ident,
                    kind: owner_ty,
                    reference,
                },
            methods,
        } = parse(quote! {
            #[rid(export)]
            #[rid(types = { MyStruct: Struct })]
            impl MyStruct {
                #[rid(export)]
                pub fn new(id: u8) -> Self {
                    Self { id }
                }
                #[rid(export)]
                pub fn get_id(&self) -> u8 {
                    self.id
                }
                #[rid(export)]
                pub fn set_id(&mut self, id: u8) {
                    self.id = id;
                }
                pub fn inc_id(&mut self, id: u8) {
                    self.id += 1;
                }
            }
        });

        assert_eq!(ident.to_string(), "MyStruct", "return ident");
        assert_eq!(reference, ParsedReference::Owned, "return reference");
        assert_matches!(
            &owner_ty,
            TypeKind::Value(Value::Custom(
                TypeInfo {
                    key,
                    cat: Category::Struct
                },
                mystruct_str
            ))
        );
        assert_eq!(methods.len(), 3, "exports 3 methods");

        // First Method: pub fn new(id: u8) -> Self
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
        } = &methods[0];

        assert_eq!(fn_ident.to_string(), "new", "function ident");
        assert_eq!(
            ret_ty, &owner_ty,
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
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
        } = &methods[1];

        assert_eq!(fn_ident.to_string(), "get_id", "function ident");
        assert_eq!(args.len(), 0, "no arg");
        assert_eq!(
            *receiver,
            Some(ParsedReceiver {
                reference: ParsedReference::Ref(None),
            }),
            "receiver is ref"
        );
        assert_eq!(*ret_ty, TypeKind::Primitive(Primitive::U8), "returns u8");

        // Third Method: pub fn set_id(&mut self, id: u8)
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustType { kind: ret_ty, .. },
        } = &methods[2];
        assert_eq!(fn_ident.to_string(), "set_id", "function ident");
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].kind,
            TypeKind::Primitive(Primitive::U8),
            "first arg u8"
        );
        assert_eq!(
            receiver,
            &Some(ParsedReceiver {
                reference: ParsedReference::RefMut(None),
            }),
            "receiver is ref mut"
        );
        assert_eq!(ret_ty, &TypeKind::Unit, "returns ()");
    }
}
