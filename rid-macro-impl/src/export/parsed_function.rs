use proc_macro_error::abort;

use crate::{
    attrs::{self, TypeInfo},
    common::{extract_path_segment, RustType},
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
