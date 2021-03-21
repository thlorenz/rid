use syn::{
    AngleBracketedGenericArguments, GenericArgument, Ident, Path,
    PathArguments, PathSegment, Type, TypePath,
};

use std::fmt::Debug;

use crate::attrs::{TypeInfo, TypeInfoMap};

use super::ParsedReference;

#[derive(Debug, PartialEq)]
pub struct RustType {
    pub ident: Ident,
    pub kind: TypeKind,
    pub reference: ParsedReference,
}

impl RustType {
    pub fn new(
        ident: Ident,
        kind: TypeKind,
        reference: ParsedReference,
    ) -> Self {
        Self {
            ident,
            kind,
            reference,
        }
    }

    pub fn self_unaliased(self, owner_name: String) -> Self {
        RustType {
            ident: self.ident,
            kind: self.kind.self_unaliased(owner_name),
            reference: self.reference,
        }
    }
}

// --------------
// TypeKind
// --------------
pub enum TypeKind {
    Primitive(Primitive),
    Value(Value),
    Composite(Composite, Option<Box<RustType>>),
    Unit,
    Unknown,
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Primitive(prim1), TypeKind::Primitive(prim2)) => {
                prim1 == prim2
            }
            (TypeKind::Value(val1), TypeKind::Value(val2)) => val1 == val2,
            (
                TypeKind::Composite(com1, ty1),
                TypeKind::Composite(com2, ty2),
            ) => com1 == com2 && ty1 == ty2,
            (TypeKind::Unit, TypeKind::Unit) => true,
            (TypeKind::Unknown, TypeKind::Unknown) => true,
            _ => false,
        }
    }
}

impl Debug for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            TypeKind::Primitive(p) => format!("TypeKind::Primitive({:?})", p),
            TypeKind::Value(val) => format!("TypeKind::Value({:?})", val),
            TypeKind::Composite(com, inner) => {
                format!("TypeKind::Composite({:?}, {:?})", com, inner)
            }
            TypeKind::Unit => "TypeKind::Unit".to_string(),
            TypeKind::Unknown => "TypeKind::Unknown".to_string(),
        };
        write!(f, "{}", kind)
    }
}

impl TypeKind {
    fn self_unaliased(self, owner_name: String) -> Self {
        match self {
            TypeKind::Value(val) => Self::Value(val.self_unaliased(owner_name)),
            _ => self,
        }
    }
}

// --------------
// Primitive
// --------------
#[derive(PartialEq)]
pub enum Primitive {
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

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            Primitive::U8 => "Primitive::U8",
            Primitive::I8 => "Primitive::I8",
            Primitive::U16 => "Primitive::U16",
            Primitive::I16 => "Primitive::I16",
            Primitive::U32 => "Primitive::U32",
            Primitive::I32 => "Primitive::I32",
            Primitive::U64 => "Primitive::U64",
            Primitive::I64 => "Primitive::I64",
            Primitive::USize => "Primitive::Usize",
            Primitive::Bool => "Primitive::Bool",
        };
        write!(f, "{}", ty)
    }
}

// --------------
// Value
// --------------
#[derive(PartialEq)]
pub enum Value {
    CString,
    String,
    Str,
    Custom(TypeInfo, String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::CString => write!(f, "Value::CString"),
            Value::Str => write!(f, "Value::Str"),
            Value::String => write!(f, "Value::String"),
            Value::Custom(type_info, name) => {
                write!(f, "Value::Custom({:?}, \"{}\")", type_info, name)
            }
        }
    }
}

impl Value {
    fn self_unaliased(self, owner_name: String) -> Self {
        match self {
            Value::Custom(type_info, name) if name == "Self" => {
                Self::Custom(type_info, owner_name)
            }
            _ => self,
        }
    }
}

// --------------
// Composite
// --------------
#[derive(PartialEq)]
pub enum Composite {
    Vec,
    Custom(TypeInfo, String),
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Composite::Vec => write!(f, "Composite::Vec"),
            Composite::Custom(type_info, name) => {
                write!(f, "Composite::Custom({:?}, \"{}\")", type_info, name)
            }
        }
    }
}

// --------------
// RustType Impl
// --------------
impl RustType {
    pub fn from_boxed_type(
        ty: Box<Type>,
        type_infos: &TypeInfoMap,
    ) -> Option<RustType> {
        resolve_rust_ty(ty.as_ref(), type_infos)
    }

    pub fn from_type(ty: &Type, type_infos: &TypeInfoMap) -> Option<RustType> {
        resolve_rust_ty(ty, type_infos)
    }
}

fn resolve_rust_ty(ty: &Type, type_infos: &TypeInfoMap) -> Option<RustType> {
    let (ty, reference) = match ty {
        Type::Reference(r) => {
            let pr = ParsedReference::from(r);
            (r.elem.as_ref(), pr)
        }
        Type::Path(_) => (ty, ParsedReference::Owned),
        _ => return None,
    };

    let (ident, kind) = match ty {
        Type::Path(TypePath { path, .. }) => {
            let PathSegment {
                ident, arguments, ..
            } = path.segments.first().unwrap();
            (ident, ident_to_kind(ident, arguments, type_infos))
        }
        _ => return None,
    };

    Some(RustType {
        ident: ident.clone(),
        kind,
        reference,
    })
}

fn ident_to_kind(
    ident: &Ident,
    arguments: &PathArguments,
    type_infos: &TypeInfoMap,
) -> TypeKind {
    let ident_str = ident.to_string();

    match arguments {
        // Non Composite Types
        PathArguments::None => {
            // primitives
            match ident_str.as_str() {
                "u8" => return TypeKind::Primitive(Primitive::U8),
                "i8" => return TypeKind::Primitive(Primitive::I8),
                "u16" => return TypeKind::Primitive(Primitive::U16),
                "i16" => return TypeKind::Primitive(Primitive::I16),
                "u32" => return TypeKind::Primitive(Primitive::U32),
                "i32" => return TypeKind::Primitive(Primitive::I32),
                "u64" => return TypeKind::Primitive(Primitive::U64),
                "i64" => return TypeKind::Primitive(Primitive::I64),
                "usize" => return TypeKind::Primitive(Primitive::USize),
                "bool" => return TypeKind::Primitive(Primitive::Bool),
                _ => {}
            };

            // known value types
            match ident_str.as_str() {
                "String" => return TypeKind::Value(Value::String),
                "CString" => return TypeKind::Value(Value::CString),
                "str" => return TypeKind::Value(Value::Str),
                _ => {}
            }

            // custom value types
            if let Some(type_info) = type_infos.get(&ident_str) {
                return TypeKind::Value(Value::Custom(
                    type_info.clone(),
                    ident_str.clone(),
                ));
            }

            TypeKind::Unknown
        }

        // Composite Types
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            args,
            ..
        }) => {
            // For now assuming one arg
            match &args[0] {
                GenericArgument::Type(ty) => {
                    let inner =
                        resolve_rust_ty(ty, type_infos).map(|x| Box::new(x));
                    match ident_str.as_str() {
                        "Vec" => TypeKind::Composite(Composite::Vec, inner),
                        _ => {
                            if let Some(type_info) = type_infos.get(&ident_str)
                            {
                                TypeKind::Composite(
                                    Composite::Custom(
                                        type_info.clone(),
                                        ident_str.clone(),
                                    ),
                                    inner,
                                )
                            } else {
                                TypeKind::Unknown
                            }
                        }
                    }
                }
                _ => TypeKind::Unknown,
            }
        }
        PathArguments::Parenthesized(args) => {
            todo!(
                "rust_type::ident_to_kind PathArguments::Parenthesized {:#?}",
                args
            )
        }
    }
}
