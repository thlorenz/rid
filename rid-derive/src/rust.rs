#![allow(dead_code)]

pub(crate) enum ValueType {
    CString,
    RString,
    RVec((Box<RustType>, syn::Ident)),
}

pub(crate) enum PrimitiveType {
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

pub(crate) enum RustType {
    Value(ValueType),
    Primitive(PrimitiveType),
    Unknown,
}

use PrimitiveType::*;
use RustType::*;
use ValueType::*;

fn extract_path_segment(path: &syn::Path) -> (&syn::Ident, RustType) {
    let syn::PathSegment {
        ident, arguments, ..
    } = path.segments.last().unwrap();
    let rust_ty = match ident.to_string().as_str() {
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
                    let (ident, vec_type) = extract_path_segment(path);
                    Value(RVec((Box::new(vec_type), ident.clone())))
                }
                _ => Unknown,
            },
            _ => Unknown,
        },
        _ => Unknown,
    };

    (ident, rust_ty)
}

impl RustType {
    pub(crate) fn try_from(ty: &syn::Type) -> Result<(&syn::Ident, RustType), String> {
        match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => Ok(extract_path_segment(path)),
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
}
