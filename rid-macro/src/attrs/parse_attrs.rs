use proc_macro2::Ident;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Attribute, Expr, LitStr, Token,
};

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
}

impl Parse for RidAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        use self::RidAttr::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // key = value
            let assign_token = input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                // TODO: more details about specific case
                abort!(name, "don't use quotes in assignments")
            } else {
                match input.parse::<Expr>() {
                    Ok(expr) => match &*name_str {
                        "to" => Ok(Model(name, expr)),
                        _ => abort!(name, "key: {} is not supported", name_str),
                    },
                    Err(_) => abort! {
                        assign_token,
                        "expected `expression` after `=`"
                    },
                }
            }
        } else if input.peek(syn::token::Paren) {
            // value()
            let nested;
            parenthesized!(nested in input);

            match &*name_str {
                "tuple" => print!("nested: {:#?}", nested),
                "to" => abort!(
                    name,
                    "'{0}' needs to be assigned via '=', i.e. #[rid({0} = value)]",
                    name_str
                ),
                _ => abort!(name, "unexpected parenthesized attribute: {}", name_str),
            }

            Ok(Debug(name))
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
