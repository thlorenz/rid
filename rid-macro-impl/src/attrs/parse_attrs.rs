use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Expr, ExprTuple, LitStr, Meta, MetaList, NestedMeta,
    Path, PathSegment, Token, Type,
};

use super::{type_category::ExprTypeInfo, TypeInfoMap};

use crate::common::abort;
use proc_macro_error::ResultExt;

#[derive(Debug)]
pub enum RidAttrOld {
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

impl RidAttrOld {
    pub fn is_export(&self) -> bool {
        match self {
            RidAttrOld::Export(_) => true,
            _ => false,
        }
    }
}

impl Parse for RidAttrOld {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use self::RidAttrOld::*;

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
                        Ok(_tpl) => Ok(RidAttrOld::Wip),
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

pub fn parse_rid_attrs_old(
    all_attrs: &[Attribute],
    ident: Option<&Ident>,
) -> Vec<RidAttrOld> {
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
                                RidAttrOld,
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
                    "UnknownIdent",
                    proc_macro2::Span::call_site(),
                );
                abort!(ident, "need ident for method exports")
            }
        };
        Some(RidAttrOld::Export(ident.clone()))
    });

    punctuated.chain(non_punctuated).collect()
}

#[derive(Debug)]
pub enum RidAttr {
    Structs(Ident, Vec<syn::Ident>),
    Enums(Ident, Vec<syn::Ident>),
    Message(Ident, syn::Ident),
    Export(Ident),
}

#[derive(Debug)]
struct AttrTuple {
    paren_token: token::Paren,
    fields: Punctuated<Type, Token![,]>,
}

impl Parse for AttrTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(AttrTuple {
            paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(Type::parse)?,
        })
    }
}

pub fn parse_rid_attrs(all_attrs: &[Attribute]) -> Vec<RidAttr> {
    all_attrs.iter().flat_map(parse_rid_attr).collect()
}

fn validate_non_empty(attr_ident: &Ident, idents: &[Ident], error_msg: &str) {
    if idents.is_empty() {
        abort!(attr_ident.span(), error_msg)
    }
}

fn validate_single(attr_ident: &Ident, idents: &[Ident], error_msg: &str) {
    if idents.len() != 1 {
        abort!(attr_ident.span(), error_msg)
    }
}

fn validate_empty(attr_ident: &Ident, idents: &[Ident], error_msg: &str) {
    if !idents.is_empty() {
        abort!(attr_ident.span(), error_msg)
    }
}

fn parse_segments(
    segments: &Punctuated<PathSegment, Token![::]>,
    nested: Option<&Punctuated<NestedMeta, Token![,]>>,
) -> Option<RidAttr> {
    if segments.len() >= 2 {
        let (
            syn::PathSegment { ident: first, .. },
            syn::PathSegment { ident: second, .. },
        ) = (&segments[0], &segments[1]);
        if first.to_string() != "rid" {
            None
        } else {
            let idents: Vec<Ident> = match nested {
                Some(nested) => nested
                    .into_iter()
                    .flat_map(|meta| match meta {
                        NestedMeta::Meta(m) => {
                            m.path().get_ident().map(|x| x.clone())
                        }
                        NestedMeta::Lit(_) => None,
                    })
                    .collect(),
                None => Vec::new(),
            };
            match second.to_string().as_str() {
                "structs" => {
                    validate_non_empty(&second, &idents, "rid::structs attributes need at least one type, i.e. #[rid::structs(MyStruct)]");
                    Some(RidAttr::Structs(second.clone(), idents))
                }
                "enums" => {
                    validate_non_empty(&second, &idents, "rid::enums attributes need at least one type, i.e. #[rid::enums(MyEnum)]");
                    Some(RidAttr::Enums(second.clone(), idents))
                }
                "message" => {
                    validate_single(&second, &idents, "rid::message attributes need to specify one recipient struct, i.e. #[rid::message(Model)]");
                    Some(RidAttr::Message(second.clone(), idents[0].clone()))
                }
                "export" => {
                    validate_empty(&second, &idents, "rid::export attributes don't support arguments and should always be just #[rid::export]");
                    Some(RidAttr::Export(second.clone()))
                }
                _ => None,
            }
        }
    } else {
        None
    }
}

fn parse_rid_attr(attr: &Attribute) -> Option<RidAttr> {
    match attr.parse_meta() {
        Ok(Meta::List(MetaList {
            path: Path { segments, .. },
            nested,
            ..
        })) => parse_segments(&segments, Some(&nested)),
        Ok(Meta::Path(Path { segments, .. })) => {
            parse_segments(&segments, None)
        }

        _ => None,
    }
}
