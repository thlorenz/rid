use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Expr, ExprTuple, Item, LitStr, Meta, MetaList,
    NestedMeta, Path, PathSegment, Token, Type,
};

use crate::common::abort;

#[derive(Debug)]
pub enum RidAttr {
    // rid specific
    Structs(Ident, Vec<syn::Ident>),
    Enums(Ident, Vec<syn::Ident>),
    Message(Ident, syn::Ident),
    Export(Ident, Option<Ident>),

    // derives
    DeriveDebug(Ident),
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

pub fn parse_rid_args(all_args: &[NestedMeta]) -> Vec<Ident> {
    all_args.iter().flat_map(parse_rid_arg).collect()
}

fn parse_rid_arg(arg: &NestedMeta) -> Option<Ident> {
    match arg {
        // NOTE: for now only supporting args with one segment, otherwise get_ident won't work
        NestedMeta::Meta(m) => m.path().get_ident().map(|x| x.clone()),
        NestedMeta::Lit(_) => None,
    }
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

fn validate_two(attr_ident: &Ident, idents: &[Ident], error_msg: &str) {
    if idents.len() != 2 {
        abort!(attr_ident.span(), error_msg)
    }
}

fn validate_empty(attr_ident: &Ident, idents: &[Ident], error_msg: &str) {
    if !idents.is_empty() {
        abort!(attr_ident.span(), error_msg)
    }
}

fn idents_from_nested(
    nested: Option<&Punctuated<NestedMeta, Token![,]>>,
) -> Vec<Ident> {
    match nested {
        Some(nested) => nested
            .into_iter()
            .flat_map(|meta| match meta {
                NestedMeta::Meta(m) => m.path().get_ident().map(|x| x.clone()),
                NestedMeta::Lit(_) => None,
            })
            .collect(),
        None => Vec::new(),
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
        match first.to_string().as_str() {
            "rid" => {
                let idents = idents_from_nested(nested);
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
                        validate_two(&second, &idents, "rid::message attributes need to specify a recipient struct and a reply enum, i.e. #[rid::message(Store, Reply)]");
                        Some(RidAttr::Message(
                            second.clone(),
                            idents[0].clone(),
                        ))
                    }
                    "export" => {
                        if idents.is_empty() {
                            Some(RidAttr::Export(second.clone(), None))
                        } else {
                            validate_single(&second, &idents, "Empty 'rid::exports' calls take exactly one argument, use either '#[rid::exports]' or #[rid::exports(fn_name)]");
                            Some(RidAttr::Export(
                                second.clone(),
                                Some(idents[0].clone()),
                            ))
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    } else if segments.len() == 1 {
        let syn::PathSegment { ident: first, .. } = &segments[0];
        match first.to_string().as_str() {
            "derive" => {
                let idents = idents_from_nested(nested);
                // For now we only extract Debug, but if we want to extract more then this would
                // have to return more than one RidAttr.
                let derives: Vec<&Ident> = idents
                    .iter()
                    .flat_map(|x| {
                        if x.to_string() == "Debug" {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .collect();
                if derives.len() > 0 {
                    Some(RidAttr::DeriveDebug(derives[0].clone()))
                } else {
                    None
                }
            }
            _ => None,
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
