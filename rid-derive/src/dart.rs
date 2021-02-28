use std::convert::TryFrom;

use rid_common::{DART_FFI, RID_FFI};

pub(crate) enum DartType {
    Int32,
    Int64,
    Bool,
    String,
}

pub(crate) enum GetterBody {
    Expression(String),
    Statement(String),
}
use GetterBody::*;

impl DartType {
    pub(crate) fn getter_body(&self, ffi_method: &syn::Ident) -> GetterBody {
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
        }
    }
    pub(crate) fn return_type(&self) -> String {
        match self {
            DartType::Int32 | DartType::Int64 => "int",
            DartType::Bool => "bool",
            DartType::String => "String",
        }
        .to_string()
    }

    pub(crate) fn type_attribute(&self) -> Option<String> {
        match self {
            DartType::Int32 => Some(format!("@{dart_ffi}.Int32()", dart_ffi = DART_FFI)),
            DartType::Int64 => Some(format!("@{dart_ffi}.Int64()", dart_ffi = DART_FFI)),
            _ => None,
        }
    }
}

impl TryFrom<&syn::Type> for DartType {
    type Error = String;
    fn try_from(ty: &syn::Type) -> Result<Self, Self::Error> {
        match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let syn::PathSegment { ident, .. } = path.segments.last().unwrap();
                match ident.to_string().as_str() {
                    "CString" | "String" => Ok(DartType::String),
                    "u8" | "i8" | "u16" | "i16" | "u32" | "i32" => Ok(DartType::Int32),
                    "u64" | "i64" => Ok(DartType::Int64),
                    "usize" => Ok(DartType::Int64), // assuming 64-bit target
                    "u128" | "i128" => Err(format!(
                        "Rust type {} cannot be represented in Dart.",
                        &ident
                    )),
                    "bool" => Ok(DartType::Bool),
                    _ => Err(format!(
                        "Rust type '{}' cannot be converted to a Dart type",
                        &ident
                    )),
                }
            }
            _ => Err(format!(
                "Rust {:?} type cannot be converted to a Dart type",
                &ty
            )),
        }
    }
}
