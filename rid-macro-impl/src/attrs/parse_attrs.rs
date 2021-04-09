use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Expr, ExprTuple, LitStr, Token,
};

use super::{type_category::ExprTypeInfo, TypeInfoMap};

use crate::common::abort;
use proc_macro_error::ResultExt;

#[derive(Debug)]
pub enum RidAttr {
    // single-identifier attributes
    Debug(syn::Ident),

    // ident [= "string literal"]
    // About(syn::Ident, Option<LitStr>),

    // ident = "string literal"
    // X(syn::Ident, syn::LitStr),

    // parse(parser_kind [= parser_func])
    // Parse(syn::Ident, ParserSpec),

    // ident [= arbitrary_expr]
    // Xs(syn::Ident, Option<syn::Expr>),

    // ident = arbitrary_expr
    Model(syn::Ident, Expr),

    Types(syn::Ident, TypeInfoMap),

    Export(syn::Ident),

    Wip,
}

impl RidAttr {
    pub fn is_export(&self) -> bool {
        match self {
            RidAttr::Export(_) => true,
            _ => false,
        }
    }
}

impl Parse for RidAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use self::RidAttr::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if name_str == "types" {
            if input.peek(Token![=]) {
                let _ = input.parse::<Token![=]>()?;
                let res = input.parse::<ExprTypeInfo>()?;
                return match res.into_validated() {
                    Ok(hash) => Ok(Types(name, hash)),
                    Err(err) => abort!(name, err),
                };
            } else {
                abort!(
                    name,
                    "'types' need to be of the form #[rid(types = { Ty: Enum }] or similar"
                )
            }
        }
        if input.peek(Token![=]) {
            // key = value
            let _ = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                // TODO: more details about specific case
                abort!(name, "don't use quotes in assignments")
            } else {
                match &*name_str {
                    "types" => match input.parse::<ExprTuple>() {
                        Ok(_tpl) => Ok(RidAttr::Wip),
                        Err(_) => {
                            abort!(name, "key: {} needs to be tuple", name_str)
                        }
                    },
                    "to" => match input.parse::<Expr>() {
                        Ok(expr) => Ok(Model(name, expr)),
                        _ => abort!(name, "key: {} is not supported", name_str),
                    },
                    _ => abort!(name, "key: {} is not supported", name_str),
                }
            }
        } else if input.peek(syn::token::Paren) {
            match &*name_str {
                "to" => abort!(
                    name,
                    "'{0}' needs to be assigned via '=', i.e. #[rid({0} = value)]",
                    name_str
                ),
                _ => abort!(name, "unexpected parenthesized attribute: {}", name_str),
            }
        } else {
            match &*name_str {
                "debug" => Ok(Debug(name)),
                "export" => Ok(Export(name)),
                "to" => abort!(
                    name,
                    "'{0}' needs to be assigned via '=', i.e. #[rid({0} = value)]",
                    name_str
                ),
                _ => abort!(name, "unexpected attribute: {}", name_str),
            }
        }
    }
}

pub fn parse_rid_attrs(
    all_attrs: &[Attribute],
    ident: Option<&Ident>,
) -> Vec<RidAttr> {
    let is_rid_ident: Vec<&Attribute> = all_attrs
        .iter()
        .filter(|attr| attr.path.is_ident("rid"))
        .collect();

    let segmented_rid_ident = all_attrs.iter().filter(|attr| {
        if is_rid_ident.iter().any(|x| x == attr) {
            false
        } else {
            match &attr.path {
                syn::Path { segments, .. } => {
                    segments.first().map_or(false, |s| s.ident == "rid")
                }
            }
        }
    });

    let punctuated = is_rid_ident.iter().flat_map(|attr| {
        attr
                        .parse_args_with(
                            syn::punctuated::Punctuated::<
                                RidAttr,
                                syn::Token![,],
                            >::parse_terminated,
                        )
                        .unwrap_or_abort()
    });

    let non_punctuated = segmented_rid_ident.flat_map(|x| {
        let ident = match ident {
            Some(ident) => ident,
            None => {
                let ident = syn::Ident::new(
                    "<unknown>",
                    proc_macro2::Span::call_site(),
                );
                abort!(ident, "need ident for method exports")
            }
        };
        Some(RidAttr::Export(ident.clone()))
    });

    punctuated.chain(non_punctuated).collect()
}
