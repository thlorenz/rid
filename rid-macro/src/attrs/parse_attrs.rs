use std::collections::{HashMap, HashSet};

use proc_macro2::Ident;
use quote::__private::ext::RepToTokensExt;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Expr, ExprArray, ExprBlock, ExprCall, ExprGroup, ExprStruct, ExprTuple, ExprType,
    LitStr, Token,
};

use super::{type_category::ExprTypeCategory, ValidatedTypeCategoryItem};
use proc_macro_error::{abort, ResultExt};

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

    Types(syn::Ident, HashMap<String, ValidatedTypeCategoryItem>),

    Wip,
}

impl Parse for RidAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use self::RidAttr::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        eprintln!("{:#?}", input);
        if name_str == "types" {
            if input.peek(Token![=]) {
                let assign_token = input.parse::<Token![=]>()?;
                let res = input.parse::<ExprTypeCategory>()?;
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
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                // TODO: more details about specific case
                abort!(name, "don't use quotes in assignments")
            } else {
                match &*name_str {
                    "types" => match input.parse::<ExprTuple>() {
                        Ok(tpl) => {
                            eprintln!("tuple {:#?}", tpl);
                            Ok(RidAttr::Wip)
                        }
                        Err(_) => abort!(name, "key: {} needs to be tuple", name_str),
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

pub fn parse_rid_attrs(all_attrs: &[Attribute]) -> Vec<RidAttr> {
    all_attrs
        .iter()
        .filter(|attr| attr.path.is_ident("rid"))
        .flat_map(|attr| {
            attr.parse_args_with(
                syn::punctuated::Punctuated::<RidAttr, syn::Token![,]>::parse_terminated,
            )
            .unwrap_or_abort()
        })
        .collect()
}
