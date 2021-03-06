use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

use crate::common::DartType;

pub enum GetterBody {
    Expression(String),
    Statement(String),
}

impl DartType {
    pub fn getter_body(&self, ffi_method: &syn::Ident) -> GetterBody {
        use GetterBody::*;

        match self {
            DartType::String => Statement(format!(
                r###"{{
                ///   int len = {rid_ffi}.{ffi_method}_len(this);
                ///   {dart_ffi}.Pointer<{dart_ffi}.Int8> ptr = {rid_ffi}.{ffi_method}(this);
                ///   String s = ptr.toDartString(len); 
                ///   ptr.free();
                ///   return s;
                /// }}"###,
                rid_ffi = RID_FFI,
                ffi_method = ffi_method,
                dart_ffi = DART_FFI
            )),
            DartType::Int32 | DartType::Int64 => Expression(format!(
                "{rid_ffi}.{ffi_method}(this);",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method
            )),
            DartType::Bool => Expression(format!(
                "{rid_ffi}.{ffi_method}(this) != 0;",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method
            )),
            DartType::Vec(_) => Expression(format!(
                "{rid_ffi}.{ffi_method}(this);",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method
            )),
            DartType::Custom(_) => Expression(format!(
                "{rid_ffi}.{ffi_method}(this);",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method
            )),
        }
    }

    pub fn return_type(&self) -> String {
        match self {
            DartType::Int32 | DartType::Int64 => "int".to_string(),
            DartType::Bool => "bool".to_string(),
            DartType::String => "String".to_string(),
            DartType::Vec(inner) => format!(
                "{dart_ffi}.Pointer<{ffigen_bind}.Vec_{ty}>",
                dart_ffi = DART_FFI,
                ffigen_bind = FFI_GEN_BIND,
                ty = inner
            ),
            // TODO: only works for Enums
            DartType::Custom(_ty) => {
                format!("{dart_ffi}.Pointer<{dart_ffi}.Int32>", dart_ffi = DART_FFI,)
            }
        }
    }

    pub fn type_attribute(&self) -> Option<String> {
        match self {
            DartType::Int32 => Some(format!("@{dart_ffi}.Int32()", dart_ffi = DART_FFI)),
            DartType::Int64 => Some(format!("@{dart_ffi}.Int64()", dart_ffi = DART_FFI)),
            _ => None,
        }
    }
}
