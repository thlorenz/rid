use proc_macro_error::abort;

use crate::{
    attrs::{self, TypeInfo},
    common::{extract_path_segment, PrimitiveType, RustType, ValueType},
};
use std::{any::Any, collections::HashMap};

#[derive(Debug)]
pub struct ParsedFunction {
    fn_ident: syn::Ident,
    receiver: Option<syn::Receiver>,
    args: Vec<(syn::Ident, RustType)>,
    return_ty: (syn::Ident, RustType),
}

impl ParsedFunction {
    pub fn new(sig: syn::Signature, _attrs: Vec<attrs::RidAttr>) -> ParsedFunction {
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

        let mut receiver = None;
        let mut args: Vec<(Ident, RustType)> = vec![];
        for arg in inputs {
            match arg {
                FnArg::Receiver(rec) => receiver = Some(rec),
                FnArg::Typed(PatType {
                    attrs,       // Vec<Attribute>,
                    pat,         // Box<Pat>,
                    colon_token, // Token![:],
                    ty,          // Box<Type>,
                }) => {
                    // TODO: For now we don't support passing custom types, but that should change
                    // same for return type
                    let ty_tpl = if let Type::Path(TypePath { ref path, .. }) = &*ty {
                        let arg_info = extract_path_segment(path, None);
                        args.push(arg_info);
                    } else {
                        abort!(
                            ty,
                            "[rid] Type not supported for exported functions {:#?}",
                            *ty
                        );
                    };
                }
            };
        }

        let return_ty = match output {
            ReturnType::Default => (ident.clone(), RustType::Unit),
            ReturnType::Type(_, ty) => {
                if let Type::Path(TypePath { ref path, .. }) = &*ty {
                    extract_path_segment(path, None)
                } else {
                    abort!(
                        ty,
                        "[rid] Type not supported for exported functions {:#?}",
                        *ty
                    );
                }
            }
        };

        Self {
            fn_ident: ident,
            receiver,
            args,
            return_ty,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

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
                ParsedFunction::new(sig, attrs)
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
            return_ty: (_, ret_ty),
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
            return_ty: (_, ret_ty),
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
            return_ty: (_, ret_ty),
        } = parse(quote! {
            fn me(id: i32) -> u8 {}
        });

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].1,
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
            return_ty: (_, ret_ty),
        } = parse(quote! {
            fn me(id: i32, s: String) -> String {}
        });

        assert_eq!(fn_ident.to_string(), "me", "function name");
        assert_eq!(receiver, None, "no receiver");
        assert_eq!(args.len(), 2, "two args");
        assert_eq!(
            args[0].1,
            RustType::Primitive(PrimitiveType::I32),
            "first arg i32"
        );
        assert_eq!(
            args[1].1,
            RustType::Value(ValueType::RString),
            "second arg String"
        );
        assert_eq!(
            ret_ty,
            RustType::Value(ValueType::RString),
            "returns String"
        );
    }
}
