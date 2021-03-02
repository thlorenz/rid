use std::fmt::Display;

use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

pub(crate) enum DartType {
    Int32,
    Int64,
    Bool,
    String,
    Vec(String),
}

pub(crate) enum GetterBody {
    Expression(String),
    Statement(String),
}

use crate::rust::RustType;

impl DartType {
    pub(crate) fn getter_body(&self, ffi_method: &syn::Ident) -> GetterBody {
        use GetterBody::*;

        match self {
            DartType::Int32 | DartType::Int64 => {
                Expression(format!("{0}.{1}(this);", RID_FFI, ffi_method))
            }
            DartType::Bool => Expression(format!("{0}.{1}(this) != 0;", RID_FFI, ffi_method)),
            DartType::String => Statement(format!(
                r###"{{
    ///   int len = {0}.{1}_len(this);
    ///   return {0}.{1}(this).toDartString(len); 
    /// }}"###,
                RID_FFI, ffi_method
            )),
            DartType::Vec(_) => Expression(format!("{0}.{1}(this);", RID_FFI, ffi_method)),
        }
    }

    pub(crate) fn return_type(&self) -> String {
        match self {
            DartType::Int32 | DartType::Int64 => "int".to_string(),
            DartType::Bool => "bool".to_string(),
            DartType::String => "String".to_string(),
            // dart_ffi.Pointer<ffigen_bind.Vec_u8>
            DartType::Vec(inner) => format!(
                "{dart_ffi}.Pointer<{ffigen_bind}.Vec_{ty}>",
                dart_ffi = DART_FFI,
                ffigen_bind = FFI_GEN_BIND,
                ty = inner
            ),
        }
    }

    pub(crate) fn type_attribute(&self) -> Option<String> {
        match self {
            DartType::Int32 => Some(format!("@{dart_ffi}.Int32()", dart_ffi = DART_FFI)),
            DartType::Int64 => Some(format!("@{dart_ffi}.Int64()", dart_ffi = DART_FFI)),
            _ => None,
        }
    }

    pub(crate) fn try_from(rust_ty: &RustType, ident: &syn::Ident) -> Result<Self, String> {
        use crate::rust::{PrimitiveType::*, ValueType::*};

        match &rust_ty {
            RustType::Primitive(p) => match p {
                U8 | I8 | U16 | I16 | U32 | I32 => Ok(DartType::Int32),
                U64 | I64 => Ok(DartType::Int64),
                USize => Ok(DartType::Int64), // assuming 64-bit target
                Bool => Ok(DartType::Bool),
            },
            RustType::Value(v) => match v {
                // For now only supporting unnested Vecs
                RVec((_, vec_indent)) => Ok(DartType::Vec(vec_indent.to_string())),
                CString | RString => Ok(DartType::String),
            },
            _ => Err(format!(
                "Rust type '{}' cannot be converted to a Dart type",
                &ident
            )),
        }
    }
}

impl Display for DartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DartType::Int32 | DartType::Int64 => "int".to_string(),
            DartType::Bool => "bool".to_string(),
            DartType::String => "String".to_string(),
            DartType::Vec(inner) => format!("List<{}>", inner),
        };
        write!(f, "{}", s)
    }
}
