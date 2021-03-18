use crate::{
    attrs::{merge_type_infos, Category, RidAttr, TypeInfo, TypeInfoMap},
    common::{
        abort, extract_path_segment, ParsedReceiver, ParsedReference, PrimitiveType, RustArg,
        RustType, ValueType,
    },
};
use std::{any::Any, collections::HashMap};

#[derive(Debug)]
pub struct ParsedFunction {
    pub fn_ident: syn::Ident,
    pub receiver: Option<ParsedReceiver>,
    pub args: Vec<RustArg>,
    pub return_arg: RustArg,
}

impl ParsedFunction {
    pub fn new(
        sig: syn::Signature,
        fn_attrs: &[RidAttr],
        owner: Option<(&syn::Ident, &TypeInfoMap)>,
    ) -> ParsedFunction {
        use syn::*;

        let Signature {
            constness,   // Option<Token![const]>,
            asyncness,   // Option<Token![async]>,
            unsafety,    // Option<Token![unsafe]>,
            abi,         // Option<Abi>,
            fn_token,    // Token![fn],
            ident,       // Ident,
            generics,    // Generics,
            paren_token, // token::Paren,
            variadic,    // Option<Variadic>,
            inputs,      // Punctuated<FnArg, Token![,]>,
            output,      // ReturnType,
        } = sig;

        let type_infos = get_type_infos(fn_attrs, owner);
        let type_infos_string = format!("{:#?}", type_infos);
        eprintln!("{}", type_infos_string);

        let mut receiver = None;
        let mut args: Vec<RustArg> = vec![];
        for arg in inputs {
            let rust_arg = match arg {
                FnArg::Receiver(rec) => receiver = Some(ParsedReceiver::new(&rec)),
                FnArg::Typed(PatType {
                    attrs,       // Vec<Attribute>,
                    pat,         // Box<Pat>,
                    colon_token, // Token![:],
                    ty,          // Box<Type>,
                }) => match RustArg::from_ty(ty.clone(), Some(&type_infos), None) {
                    Some(rust_arg) => args.push(rust_arg),
                    None => abort!(
                        ty,
                        "[rid] Type not supported for exported functions {:#?}",
                        *ty
                    ),
                },
            };
        }

        let return_arg = match output {
            ReturnType::Default => RustArg::new(ident.clone(), RustType::Unit, None),
            ReturnType::Type(_, ty) => {
                match RustArg::from_ty(ty.clone(), Some(&type_infos), owner.map(|(idnt, _)| idnt)) {
                    Some(rust_arg) => rust_arg,
                    None => abort!(
                        ty,
                        "[rid] Type not supported for exported functions {:#?}",
                        *ty
                    ),
                }
            }
        };
        Self {
            fn_ident: ident,
            receiver,
            args,
            return_arg,
        }
    }
}

fn get_type_infos(fn_attrs: &[RidAttr], owner: Option<(&syn::Ident, &TypeInfoMap)>) -> TypeInfoMap {
    let mut type_infos: TypeInfoMap = fn_attrs.into();

    if let Some((ident, owner_type_infos)) = owner {
        merge_type_infos(&mut type_infos, &owner_type_infos);
        // NOTE: assuming that the owner is a Struct unless otherwise specified
        let key = ident.to_string();
        if !owner_type_infos.contains_key(&key) {
            type_infos.insert(
                key.clone(),
                TypeInfo {
                    key: ident.clone(),
                    cat: Category::Struct,
                },
            );
        }
        // Other parts of the type resolution process, i.e. RustType shouldn't need to
        // know about the special case of 'Self' therefore we alias it to the owner type here
        if let Some(type_info) = type_infos.get(&key) {
            let type_info = type_info.clone();
            type_infos.insert("Self".to_string(), type_info);
        }
    };
    type_infos
}

/**********************************************
 **************      Tests       **************
**********************************************/

#[cfg(test)]
mod tests {
    use crate::{attrs, common::ParsedReference};

    use assert_matches::assert_matches;

    use super::*;
    use quote::quote;

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
}
