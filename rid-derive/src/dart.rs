use std::convert::TryFrom;

pub(crate) enum DartType {
    Int32,
    Int64,
    Bool,
    String,
}

impl DartType {
    pub(crate) fn getter_body(&self, ffi_method: &syn::Ident) -> String {
        match self {
            DartType::Int32 | DartType::Int64 => format!("rid_ffi.{}(this);", ffi_method),
            DartType::Bool => format!("rid_ffi.{}(this) != 0;", ffi_method),
            DartType::String => format!(
                r###"{{
    ///   int len = rid_ffi.{}_len(this);
    ///   return rid_ffi.{}(this).toDartString(len); 
    /// }}"###,
                ffi_method, ffi_method
            ),
        }
    }
    pub(crate) fn return_type(&self) -> String {
        // import 'dart:ffi' as ffi;
        match self {
            DartType::Int32 => "@ffi.Int32() int",
            DartType::Int64 => "@ffi.Int64() int",
            DartType::Bool => "int",
            DartType::String => "String",
        }
        .to_string()
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
