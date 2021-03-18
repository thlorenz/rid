#![allow(dead_code)]
use super::abort;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(PartialEq)]
pub enum ValueType {
    CString,
    RString,
    RVec((Box<RustType>, syn::Ident)),
    RCustom(TypeInfo, String),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            CString => "CString".to_string(),
            RString => "String".to_string(),
            RVec((rust_ty, ident)) => format!("Vec<{}|{}>", rust_ty, ident),
            RCustom(info, s) => format!("ValueType::RCustom({:?}, {})", info, s),
        };
        write!(f, "{}", ty)
    }
}

#[derive(PartialEq)]
pub enum PrimitiveType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    USize,
    Bool,
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            U8 => "u8",
            I8 => "i8",
            U16 => "u16",
            I16 => "i16",
            U32 => "u32",
            I32 => "i32",
            U64 => "u64",
            I64 => "i64",
            USize => "usize",
            Bool => "bool",
        };
        write!(f, "{}", ty)
    }
}

#[derive(PartialEq)]
pub enum RustType {
    Value(ValueType),
    Primitive(PrimitiveType),
    Unit,
    Unknown,
}

impl Debug for RustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for RustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            Value(x) => format!("RustType::Value({})", x),
            Primitive(x) => format!("RustType::Primitive(({})", x),
            Unit => "RustType::Unit".to_string(),
            Unknown => "RustType::Unknown".to_string(),
        };
        write!(f, "{}", ty)
    }
}

use PrimitiveType::*;
use RustType::*;
use ValueType::*;

use crate::attrs::{self, TypeInfo, TypeInfoMap};

pub fn extract_path_segment(
    path: &syn::Path,
    types: Option<&TypeInfoMap>,
) -> (syn::Ident, RustType) {
    let syn::PathSegment {
        ident, arguments, ..
    } = path.segments.last().unwrap();
    let ident_str = ident.to_string();
    // TODO: consider  path.type_id() for built in types here instead
    let rust_ty = match ident_str.as_str() {
        "CString" => Value(CString),
        "String" => Value(RString),
        "u8" => Primitive(U8),
        "i8" => Primitive(I8),
        "u16" => Primitive(U16),
        "i16" => Primitive(I16),
        "u32" => Primitive(U32),
        "i32" => Primitive(I32),
        "usize" => Primitive(USize),
        "bool" => Primitive(Bool),
        "Vec" => match arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => match &args[0] {
                syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) => {
                    let (ident, vec_type) = extract_path_segment(path, types);
                    Value(RVec((Box::new(vec_type), ident.clone())))
                }
                _ => Unknown,
            },
            _ => Unknown,
        },
        // TODO: is there a way to check this or do we require an attribute?
        // Or is it fine to just blindly assume we're dealing with a Value type
        // for which the same generic code we generate works?
        // However since for built in rust types we won't have an opaque structs,
        // the access methods will be missing, so at least that we need to consider somwehow.
        _ => {
            if let Some(types) = types {
                // TODO: We don't currently verify that all types are being used. That would require
                // recording this across all variant fields.
                match types.get(&ident_str) {
                    Some(ty) => Value(RCustom(ty.clone(), ident_str)),
                    None => abort!(
                        ident,
                        // TODO: Include info regarding which custom types are viable, link to URL?
                        "[rid] Missing info for custom type {0}. \
                    Specify via '#[rid(types = {{ {0}: Enum }})]' or similar.",
                        ident_str
                    ),
                }
            } else {
                // TODO: For now we always assume struct since this branch only hits of struct
                // impls ATM
                Value(RCustom(
                    TypeInfo {
                        key: ident.clone(),
                        cat: attrs::Category::Struct,
                    },
                    ident_str,
                ))
            }
        }
    };

    (ident.clone(), rust_ty)
}

impl RustType {
    pub fn try_from(ty: &syn::Type, types: &TypeInfoMap) -> Result<(syn::Ident, RustType), String> {
        match &ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                Ok(extract_path_segment(path, Some(types)))
            }
            syn::Type::Array(ty) => Err(format!("Array: {:#?}", &ty)),
            syn::Type::BareFn(ty) => Err(format!("BareFn: {:#?}", &ty)),
            syn::Type::Group(ty) => Err(format!("Group: {:#?}", &ty)),
            syn::Type::ImplTrait(ty) => Err(format!("ImplTrait: {:#?}", &ty)),
            syn::Type::Infer(ty) => Err(format!("Infer: {:#?}", &ty)),
            syn::Type::Macro(ty) => Err(format!("Macro: {:#?}", &ty)),
            syn::Type::Never(ty) => Err(format!("Never: {:#?}", &ty)),
            syn::Type::Paren(ty) => Err(format!("Paren: {:#?}", &ty)),
            syn::Type::Ptr(ty) => Err(format!("Ptr: {:#?}", &ty)),
            syn::Type::Reference(ty) => Err(format!("Reference: {:#?}", &ty)),
            syn::Type::Slice(ty) => Err(format!("Slice: {:#?}", &ty)),
            syn::Type::TraitObject(ty) => Err(format!("TraitObject: {:#?}", &ty)),
            syn::Type::Tuple(ty) => Err(format!("Tuple: {:#?}", &ty)),
            syn::Type::Verbatim(ty) => Err(format!("Verbatim: {:#?}", &ty)),
            _ => Err(format!("Unexpected: {:#?}", &ty)),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Primitive(_) => true,
            _ => false,
        }
    }

    pub fn with_replaced_self(self, owner: &syn::Ident) -> RustType {
        match self {
            RustType::Value(ValueType::RCustom(
                TypeInfo {
                    key: _,
                    cat: attrs::Category::Struct,
                },
                name,
            )) if name == "Self" => RustType::Value(ValueType::RCustom(
                TypeInfo {
                    key: owner.clone(),
                    cat: attrs::Category::Struct,
                },
                owner.to_string(),
            )),
            _ => self,
        }
    }
}
