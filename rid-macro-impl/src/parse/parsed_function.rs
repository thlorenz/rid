use super::{
    rust_type::{RustType, TypeKind},
    ParsedReceiver, ParsedReference,
};
use crate::{
    attrs::{Category, FunctionConfig, RidAttr, TypeInfo, TypeInfoMap},
    common::abort,
};

#[derive(Debug)]
pub struct ParsedFunction {
    pub fn_ident: syn::Ident,
    pub receiver: Option<ParsedReceiver>,
    pub args: Vec<RustType>,
    pub return_arg: RustType,
}

impl ParsedFunction {
    pub fn new(
        sig: syn::Signature,
        config: &FunctionConfig,
        owner: Option<(&syn::Ident, &TypeInfoMap)>,
    ) -> ParsedFunction {
        use syn::*;

        let Signature {
            constness: _,   // Option<Token![const]>,
            asyncness: _,   // Option<Token![async]>,
            unsafety: _,    // Option<Token![unsafe]>,
            abi: _,         // Option<Abi>,
            fn_token: _,    // Token![fn],
            ident,          // Ident,
            generics: _,    // Generics,
            paren_token: _, // token::Paren,
            variadic: _,    // Option<Variadic>,
            inputs,         // Punctuated<FnArg, Token![,]>,
            output,         // ReturnType,
        } = sig;

        let mut receiver = None;
        let mut args: Vec<RustType> = vec![];
        for arg in inputs {
            match arg {
                FnArg::Receiver(rec) => {
                    let owner_info = owner
                        .and_then(|x| config.type_infos.get(&x.0.to_string()));
                    if let Some(owner_info) = owner_info {
                        receiver =
                            Some(ParsedReceiver::new(&rec, owner_info.clone()))
                    } else {
                        abort!(ident, "Missing owner info for this function with Self receiver.")
                    }
                }
                FnArg::Typed(PatType {
                    attrs: _,       // Vec<Attribute>,
                    pat: _,         // Box<Pat>,
                    colon_token: _, // Token![:],
                    ty,             // Box<Type>,
                }) => {
                    match RustType::from_boxed_type(
                        ty.clone(),
                        &config.type_infos,
                    ) {
                        Some(rust_type) => args.push(rust_type),
                        None => abort!(
                        ty,
                        "[rid] Type not supported for exported functions {:#?}",
                        *ty
                    ),
                    }
                }
            };
        }

        let return_arg = match output {
            ReturnType::Default => RustType::new(
                ident.clone(),
                TypeKind::Unit,
                ParsedReference::Owned,
            ),
            ReturnType::Type(_, ty) => {
                match RustType::from_boxed_type(ty.clone(), &config.type_infos)
                {
                    Some(rust_type) => rust_type,
                    None => abort!(
                        ty,
                        "[rid] Type not supported for exported functions {:#?}",
                        *ty
                    ),
                }
            }
        };

        let return_arg = match owner {
            Some((ident, _)) => return_arg.self_unaliased(ident.to_string()),
            None => return_arg,
        };

        Self {
            fn_ident: ident,
            receiver,
            args,
            return_arg,
        }
    }
}
