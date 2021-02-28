#![allow(dead_code)]
use std::convert::TryFrom;

pub(crate) enum ValueType {
    CString,
    RString,
}

pub(crate) enum PrimitiveType {
    Int,
    Bool,
    Unknown,
}

pub(crate) enum RustType {
    Value(ValueType),
    Primitive(PrimitiveType),
}

use PrimitiveType::*;
use RustType::*;
use ValueType::*;

impl TryFrom<&syn::Type> for RustType {
    type Error = String;
    fn try_from(ty: &syn::Type) -> Result<Self, Self::Error> {
        // TODO: cannot really detect if a type is primitive (just guess)
        // do we need to have an attribute for this?
        Ok(match ty {
            syn::Type::Array(ty) => {
                println!("Array: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::BareFn(ty) => {
                println!("BareFn: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Group(ty) => {
                println!("Group: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::ImplTrait(ty) => {
                println!("ImplTrait: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Infer(ty) => {
                println!("Infer: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Macro(ty) => {
                println!("Macro: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Never(ty) => {
                println!("Never: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Paren(ty) => {
                println!("Paren: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let syn::PathSegment { ident, .. } = path.segments.last().unwrap();
                match ident.to_string().as_str() {
                    "CString" => Value(CString),
                    "String" => Value(RString),
                    "u8" | "i8" | "u16" | "i16" | "u32" | "i32" => Primitive(Int),
                    "bool" => Primitive(Bool),
                    _ => Primitive(Unknown),
                }
            }
            syn::Type::Ptr(ty) => {
                println!("Ptr: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Reference(ty) => {
                println!("Reference: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Slice(ty) => {
                println!("Slice: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::TraitObject(ty) => {
                println!("TraitObject: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Tuple(ty) => {
                println!("Tuple: {:#?}", &ty);
                Primitive(Unknown)
            }
            syn::Type::Verbatim(ty) => {
                println!("Verbatim: {:#?}", &ty);
                Primitive(Unknown)
            }
            _ => Primitive(Unknown),
        })
    }
}
