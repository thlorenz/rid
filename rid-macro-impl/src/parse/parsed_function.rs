use quote::format_ident;

use super::{
    rust_type::{RustType, TypeKind},
    ParsedReceiver, ParsedReference,
};
use crate::{
    attrs::{Category, FunctionConfig, RidAttr, TypeInfo, TypeInfoMap},
    common::abort,
    parse::rust_type::RustTypeContext,
    render_dart::DartArg,
};

#[derive(Debug)]
pub struct ParsedFunction {
    /// Ident of original exported function
    pub fn_ident: syn::Ident,

    /// Ident to which the function wrapping the original one should be aliased
    pub fn_ident_alias: Option<syn::Ident>,

    /// Self of instance methods
    pub receiver: Option<ParsedReceiver>,

    /// Function args besides the receiver
    pub args: Vec<RustType>,

    /// The `args` converted into `DartArg` to use when rendering Dart code
    pub dart_args: Vec<DartArg>,

    /// The type of arg returned by the original function
    pub return_arg: RustType,

    /// Function config with extra information like type_infos [TypeInfoMap]
    pub config: FunctionConfig,
}

impl ParsedFunction {
    pub fn new(
        sig: syn::Signature,
        config: FunctionConfig,
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
                RustTypeContext::Default,
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

        let fn_ident_alias = config.fn_export_alias.clone();

        let dart_args: Vec<DartArg> = args
            .iter()
            .enumerate()
            .map(|(slot, arg)| DartArg::from(arg, &config.type_infos, slot))
            .collect();

        Self {
            fn_ident: ident,
            fn_ident_alias,
            receiver,
            args,
            return_arg,
            config,
            dart_args,
        }
    }

    /// Information about custom types specified on top of the functio's impl block or the function
    /// definition itself
    pub fn type_infos(&self) -> &TypeInfoMap {
        &self.config.type_infos
    }
}
