use std::fmt::{Debug, Display};

pub enum DartType {
    Int32,
    Int64,
    Bool,
    String,
    Vec(String),
    Custom(TypeInfo, String),
}

use crate::attrs::TypeInfo;

use super::RustType;

impl DartType {
    pub fn try_from(
        rust_ty: &RustType,
        ident: &syn::Ident,
    ) -> Result<Self, String> {
        use super::rust::{PrimitiveType::*, ValueType::*};

        match &rust_ty {
            RustType::Primitive(p) => match p {
                U8 | I8 | U16 | I16 | U32 | I32 => Ok(DartType::Int32),
                U64 | I64 => Ok(DartType::Int64),
                USize => Ok(DartType::Int64), // assuming 64-bit target
                Bool => Ok(DartType::Bool),
            },
            RustType::Value(v) => match v {
                // For now only supporting unnested Vecs
                RVec((_, vec_indent)) => {
                    Ok(DartType::Vec(vec_indent.to_string()))
                }
                CString | RString => Ok(DartType::String),
                RCustom(info, _) => {
                    let ty =
                        info.typedef.as_ref().unwrap_or(&info.key).to_string();
                    Ok(DartType::Custom(info.clone(), ty))
                }
            },
            _ => Err(format!(
                "Rust type '{}'/'{}' cannot be converted to a Dart type",
                &rust_ty, &ident
            )),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            DartType::Int32 | DartType::Int64 | DartType::Bool => true,
            DartType::String => false,
            DartType::Vec(_) => false,
            DartType::Custom(_, _) => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            DartType::Int32 | DartType::Int64 | DartType::Bool => false,
            DartType::String => false,
            DartType::Vec(_) => false,
            DartType::Custom(inner, _) => !inner.is_enum(),
        }
    }
}

impl Debug for DartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// WARN: this is used to determine dart return type.
// Should probably move into a separate method
impl Display for DartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DartType::Int32 | DartType::Int64 => "int".to_string(),
            DartType::Bool => "bool".to_string(),
            DartType::String => "String".to_string(),
            DartType::Vec(inner) => format!("List<{}>", inner),
            DartType::Custom(_, ty) => ty.to_string(),
        };
        write!(f, "{}", s)
    }
}
