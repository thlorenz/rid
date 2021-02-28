#![allow(dead_code)]
use std::convert::TryFrom;

pub(crate) enum ValueType {
    CString,
    RString,
    RVec((Box<RustType>, syn::Ident)),
}

pub(crate) enum PrimitiveType {
    Int,
    Bool,
}

pub(crate) enum RustType {
    Value(ValueType),
    Primitive(PrimitiveType),
    Unknown,
}

use PrimitiveType::*;
use RustType::*;
use ValueType::*;

fn extract_path_segment(path: &syn::Path) -> RustType {
    let syn::PathSegment {
        ident, arguments, ..
    } = path.segments.last().unwrap();
    match ident.to_string().as_str() {
        "CString" => Value(CString),
        "String" => Value(RString),
        "u8" | "i8" | "u16" | "i16" | "u32" | "i32" => Primitive(Int),
        "usize" => Primitive(Int),
        "bool" => Primitive(Bool),
        "Vec" => match arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => match &args[0] {
                syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) => {
                    let vec_type = extract_path_segment(path);
                    let syn::PathSegment { ident, .. } = path.segments.last().unwrap();
                    Value(RVec((Box::new(vec_type), ident.clone())))
                }
                _ => Unknown,
            },
            _ => Unknown,
        },
        _ => Unknown,
    }
}

impl TryFrom<&syn::Type> for RustType {
    type Error = String;
    fn try_from(ty: &syn::Type) -> Result<Self, Self::Error> {
        // TODO: cannot really detect if a type is primitive (just guess)
        // do we need to have an attribute for this?
        Ok(match ty {
            syn::Type::Array(ty) => {
                println!("Array: {:#?}", &ty);
                Unknown
            }
            syn::Type::BareFn(ty) => {
                println!("BareFn: {:#?}", &ty);
                Unknown
            }
            syn::Type::Group(ty) => {
                println!("Group: {:#?}", &ty);
                Unknown
            }
            syn::Type::ImplTrait(ty) => {
                println!("ImplTrait: {:#?}", &ty);
                Unknown
            }
            syn::Type::Infer(ty) => {
                println!("Infer: {:#?}", &ty);
                Unknown
            }
            syn::Type::Macro(ty) => {
                println!("Macro: {:#?}", &ty);
                Unknown
            }
            syn::Type::Never(ty) => {
                println!("Never: {:#?}", &ty);
                Unknown
            }
            syn::Type::Paren(ty) => {
                println!("Paren: {:#?}", &ty);
                Unknown
            }
            syn::Type::Path(syn::TypePath { path, .. }) => extract_path_segment(path),
            syn::Type::Ptr(ty) => {
                println!("Ptr: {:#?}", &ty);
                Unknown
            }
            syn::Type::Reference(ty) => {
                println!("Reference: {:#?}", &ty);
                Unknown
            }
            syn::Type::Slice(ty) => {
                println!("Slice: {:#?}", &ty);
                Unknown
            }
            syn::Type::TraitObject(ty) => {
                println!("TraitObject: {:#?}", &ty);
                Unknown
            }
            syn::Type::Tuple(ty) => {
                println!("Tuple: {:#?}", &ty);
                Unknown
            }
            syn::Type::Verbatim(ty) => {
                println!("Verbatim: {:#?}", &ty);
                Unknown
            }
            _ => Unknown,
        })
    }
}
