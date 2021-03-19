use assert_matches::assert_matches;

use crate::{
    attrs,
    attrs::{merge_type_infos, Category, RidAttr, TypeInfo, TypeInfoMap},
    common::{
        abort, extract_path_segment, ParsedReceiver, ParsedReference, PrimitiveType, RustArg,
        RustType, ValueType,
    },
};
use quote::quote;
use std::{any::Any, collections::HashMap};

use super::ParsedFunction;

fn parse(input: proc_macro2::TokenStream) -> ParsedFunction {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    let args = syn::AttributeArgs::new();
    match item {
        syn::Item::Fn(syn::ItemFn {
            attrs, // Vec<Attribute>,
            vis,   // Visibility,
            sig,   // Signature,
            block, // Box<Block>,
        }) => {
            let attrs = attrs::parse_rid_attrs(&attrs);
            ParsedFunction::new(sig, &attrs, None)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

#[test]
fn void_function_no_args() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me() {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(receiver, None, "no receiver");
    assert_eq!(args.len(), 0, "empty args");
    assert_eq!(ret_ty, RustType::Unit, "returns ()");
}

#[test]
fn u8_function_no_args() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me() -> u8 {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(receiver, None, "no receiver");
    assert_eq!(args.len(), 0, "empty args");
    assert_eq!(ret_ty, RustType::Primitive(PrimitiveType::U8), "returns u8");
}

#[test]
fn u8_function_i32_arg() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me(id: i32) -> u8 {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(receiver, None, "no receiver");
    assert_eq!(args.len(), 1, "one arg");
    assert_eq!(
        args[0].ty,
        RustType::Primitive(PrimitiveType::I32),
        "first arg i32"
    );
    assert_eq!(ret_ty, RustType::Primitive(PrimitiveType::U8), "returns u8");
}

#[test]
fn string_function_i32_and_string_arg() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me(id: i32, s: String) -> String {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(receiver, None, "no receiver");
    assert_eq!(args.len(), 2, "two args");
    assert_eq!(
        args[0].ty,
        RustType::Primitive(PrimitiveType::I32),
        "first arg i32"
    );
    assert_eq!(
        args[1].ty,
        RustType::Value(ValueType::RString),
        "second arg String"
    );
    assert_eq!(
        ret_ty,
        RustType::Value(ValueType::RString),
        "returns String"
    );
}

#[test]
fn void_function_no_args_ref_self() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me(&self) {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(
        receiver,
        Some(ParsedReceiver {
            reference: ParsedReference::Ref(None)
        }),
        "no receiver"
    );
    assert_eq!(args.len(), 0, "empty args");
    assert_eq!(ret_ty, RustType::Unit, "returns ()");
}

#[test]
fn void_function_one_arg_ref_mut_self() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me(&mut self, id: usize) {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(
        receiver,
        Some(ParsedReceiver {
            reference: ParsedReference::RefMut(None)
        }),
        "no receiver"
    );
    assert_eq!(args.len(), 1, "empty args");
    assert_eq!(
        args[0].ty,
        RustType::Primitive(PrimitiveType::USize),
        "first arg of type usize"
    );
    assert_eq!(ret_ty, RustType::Unit, "returns ()");
}

#[test]
fn void_function_no_args_owned_self() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        fn me(self) {}
    });

    assert_eq!(fn_ident.to_string(), "me", "function name");
    assert_eq!(
        receiver,
        Some(ParsedReceiver {
            reference: ParsedReference::Owned
        }),
        "owned receiver"
    );
    assert_eq!(args.len(), 0, "empty args");
    assert_eq!(ret_ty, RustType::Unit, "returns ()");
}

#[test]
fn u8_function_custom_struct_arg() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        #[rid(types = { ItemStruct: Struct })]
        fn id(item: &ItemStruct) -> u8 {}
    });

    assert_eq!(fn_ident.to_string(), "id", "function name");
    assert_eq!(receiver, None, "no receiver");
    assert_eq!(args.len(), 1, "one arg");

    assert_matches!(
        &args[0],
        RustArg {
            ident: _,
            reference: Some(ParsedReference::Ref(None)),
            ty: RustType::Value(ValueType::RCustom(
                TypeInfo {
                    cat: Category::Struct,
                    ..
                },
                name
            )),
        }  => {
            assert_eq!(name, "ItemStruct");
        }
    );

    assert_eq!(ret_ty, RustType::Primitive(PrimitiveType::U8), "returns u8");
}

#[test]
fn custom_return_type() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg: RustArg { ty: ret_ty, .. },
    } = parse(quote! {
        #[rid(types = { Todo: Struct })]
        fn get_todo(&self) -> Todo {}
    });

    assert_eq!(fn_ident.to_string(), "get_todo", "function name");
    assert_eq!(
        receiver,
        Some(ParsedReceiver {
            reference: ParsedReference::Ref(None)
        }),
        "ref receiver"
    );
    assert_eq!(args.len(), 0, "no arg");

    assert_matches!(
        &ret_ty ,
        RustType::Value(ValueType::RCustom(TypeInfo { key, cat }, name)) => {
            assert_eq!(
                (cat, name.as_str()),
                (&attrs::Category::Struct, "Todo"),
                "custom return type"
            );
        }
    );
}

#[test]
fn custom_return_type_ref() {
    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg:
            RustArg {
                ty: ret_ty,
                reference,
                ..
            },
    } = parse(quote! {
        #[rid(types = { Todo: Struct })]
        fn get_todo() -> &Todo {}
    });

    assert_eq!(fn_ident.to_string(), "get_todo", "function name");
    assert_eq!(receiver, None, "no receiver");
    assert_eq!(args.len(), 0, "no arg");

    assert_matches!(
        &ret_ty ,
        RustType::Value(ValueType::RCustom(TypeInfo { key, cat }, name)) => {
            assert_eq!(
                (cat, name.as_str()),
                (&attrs::Category::Struct, "Todo"),
                "custom return type"
            );
        }
    );
    assert_matches!(&reference, Some(ParsedReference::Ref(None)));
}

#[test]
fn custom_return_type_ref_with_lifetime() {
    let ParsedFunction {
        return_arg: RustArg { reference, .. },
        ..
    } = parse(quote! {
        #[rid(types = { Todo: Struct })]
        fn get_todo() -> &'a Todo {}
    });

    assert_matches!(&reference, Some(ParsedReference::Ref(Some(ident))) => {
        assert_eq!(ident.to_string(), "a");
    });
}

// fn filtered_todos(&self) -> Vec<&Todo> {
