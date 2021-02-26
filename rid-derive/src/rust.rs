#![allow(dead_code)]
use std::convert::TryFrom;

pub(crate) enum ValueType {
    CString,
    String,
}

pub(crate) enum RustType {
    Value(ValueType),
    Primitive,
}

impl TryFrom<&syn::Type> for RustType {
    type Error = String;
    fn try_from(ty: &syn::Type) -> Result<Self, Self::Error> {
        // TODO: cannot really detect if a type is primitive (just guess)
        // do we need to have an attribute for this?
        Ok(match ty {
            syn::Type::Array(ty) => {
                println!("Array: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::BareFn(ty) => {
                println!("BareFn: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Group(ty) => {
                println!("Group: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::ImplTrait(ty) => {
                println!("ImplTrait: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Infer(ty) => {
                println!("Infer: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Macro(ty) => {
                println!("Macro: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Never(ty) => {
                println!("Never: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Paren(ty) => {
                println!("Paren: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let syn::PathSegment { ident, .. } = path.segments.last().unwrap();
                match ident.to_string().as_str() {
                    "CString" => RustType::Value(ValueType::CString),
                    "String" => RustType::Value(ValueType::String),
                    _ => RustType::Primitive,
                }
            }
            syn::Type::Ptr(ty) => {
                println!("Ptr: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Reference(ty) => {
                println!("Reference: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Slice(ty) => {
                println!("Slice: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::TraitObject(ty) => {
                println!("TraitObject: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Tuple(ty) => {
                println!("Tuple: {:#?}", &ty);
                RustType::Primitive
            }
            syn::Type::Verbatim(ty) => {
                println!("Verbatim: {:#?}", &ty);
                RustType::Primitive
            }
            _ => RustType::Primitive,
        })
    }
}
