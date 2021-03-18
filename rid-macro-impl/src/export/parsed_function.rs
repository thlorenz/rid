use proc_macro_error::abort;

use crate::{
    attrs::{Category, RidAttr, TypeInfo, TypeInfoMap},
    common::{
        extract_path_segment, ParsedReceiver, ParsedReference, PrimitiveType, RustArg, RustType,
        ValueType,
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
        owner: Option<&syn::Ident>,
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

        let mut receiver = None;
        let mut args: Vec<RustArg> = vec![];
        for arg in inputs {
            match arg {
                FnArg::Receiver(rec) => receiver = Some(ParsedReceiver::new(&rec)),
                FnArg::Typed(PatType {
                    attrs,       // Vec<Attribute>,
                    pat,         // Box<Pat>,
                    colon_token, // Token![:],
                    ty,          // Box<Type>,
                }) => {
                    let (ty, parsed_ref) = match *ty {
                        Type::Reference(r) => {
                            let pr = Some(ParsedReference::from(&r));
                            (r.elem, pr)
                        }
                        Type::Path(_) => (ty, None),
                        _ => {
                            eprintln!("{:#?}", &*ty);
                            abort!(
                                ty,
                                "[rid] Type not supported for exported functions {:#?}",
                                *ty
                            );
                        }
                    };
                    let ty_tpl = if let Type::Path(TypePath { ref path, .. }) = *ty {
                        let (ident, ty) = extract_path_segment(path, Some(&type_infos));
                        args.push(RustArg::new(ident, ty, parsed_ref));
                    } else {
                        eprintln!("{:#?}", &*ty);
                        abort!(
                            ty,
                            "[rid] Type not supported for exported functions {:#?}",
                            *ty
                        );
                    };
                }
            };
        }

        let (ret_ident, ret_ty) = match output {
            ReturnType::Default => (ident.clone(), RustType::Unit),
            ReturnType::Type(_, ty) => {
                if let Type::Path(TypePath { ref path, .. }) = &*ty {
                    let (ident, ty) = extract_path_segment(path, None);
                    if let Some(owner) = owner {
                        (ident, ty.with_replaced_self(owner))
                    } else {
                        (ident, ty)
                    }
                } else {
                    abort!(
                        ty,
                        "[rid] Type not supported for exported functions {:#?}",
                        *ty
                    );
                }
            }
        };
        // TODO: where do we get return arg reference from?
        let return_arg = RustArg::new(ret_ident, ret_ty, None);

        Self {
            fn_ident: ident,
            receiver,
            args,
            return_arg,
        }
    }
}

fn get_type_infos(fn_attrs: &[RidAttr], owner: Option<&syn::Ident>) -> TypeInfoMap {
    // TODO: merge owner RidAttrs here in order to avoid having to specify types of
    // an impl block multiple times.
    let mut type_infos: TypeInfoMap = fn_attrs.into();
    if let Some(ident) = owner {
        // NOTE: assuming that the owner is always a Struct as the only way that
        // this function must be an impl method (we don't allow exporting Enum impl
        // methods at this point)
        type_infos.insert(
            ident.to_string(),
            TypeInfo {
                key: ident.clone(),
                cat: Category::Struct,
            },
        );
    };
    type_infos
}

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
    fn self_function_one_arg() {
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustArg { ty: ret_ty, .. },
        } = parse(quote! {
            fn new(id: u8) -> Self {}
        });

        assert_eq!(fn_ident.to_string(), "new", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].ty,
            RustType::Primitive(PrimitiveType::U8),
            "first arg u8"
        );
        // NOTE: that since no owner was provided the Self is not converted to the owner type
        // for a more realistic case see ./parsed_impl_block.rs
        if let RustType::Value(ValueType::RCustom(ty, name)) = ret_ty {
            assert!(ty.is_self(), "returns Self");
            assert_eq!(name, "Self", "named Self")
        } else {
            panic!("Expected Self return");
        }
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
                reference: Some(ParsedReference::Ref(None,)),
                ty: RustType::Value(ValueType::RCustom(
                    TypeInfo {
                        cat: Category::Struct,
                        ..
                    },
                    name
                )),
            } if name == "ItemStruct" => {}
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

        match &ret_ty {
            RustType::Value(ValueType::RCustom(TypeInfo { key, cat }, name)) => {
                assert_eq!(
                    (cat, name.as_str()),
                    (&attrs::Category::Struct, "Todo"),
                    "custom return type"
                );
            }
            _ => panic!("did not match return type"),
        };

        eprintln!("return: {:#?}", ret_ty);
    }

    // fn filtered_todos(&self) -> Vec<&Todo> {
}
