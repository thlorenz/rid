use super::{
    rust_type::{RustType, TypeKind},
    ParsedReceiver, ParsedReference,
};
use crate::{
    attrs::{merge_type_infos, Category, RidAttr, TypeInfo, TypeInfoMap},
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
        fn_attrs: &[RidAttr],
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

        let type_infos = get_type_infos(fn_attrs, owner);

        let mut receiver = None;
        let mut args: Vec<RustType> = vec![];
        for arg in inputs {
            match arg {
                FnArg::Receiver(rec) => {
                    receiver = Some(ParsedReceiver::new(&rec))
                }
                FnArg::Typed(PatType {
                    attrs: _,       // Vec<Attribute>,
                    pat: _,         // Box<Pat>,
                    colon_token: _, // Token![:],
                    ty,             // Box<Type>,
                }) => {
                    match RustType::from_boxed_type(ty.clone(), &type_infos) {
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
                match RustType::from_boxed_type(ty.clone(), &type_infos) {
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

fn get_type_infos(
    fn_attrs: &[RidAttr],
    owner: Option<(&syn::Ident, &TypeInfoMap)>,
) -> TypeInfoMap {
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
