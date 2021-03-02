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
            DartType::Custom(_) => Expression(format!("{0}.{1}(this);", RID_FFI, ffi_method)),
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
            DartType::Custom(ty) => format!(
                "{dart_ffi}.Pointer<{ffigen_bind}.{ty}>",
                dart_ffi = DART_FFI,
                ffigen_bind = FFI_GEN_BIND,
                ty = ty
            ),
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
