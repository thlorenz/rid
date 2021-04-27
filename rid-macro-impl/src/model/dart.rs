use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

use crate::{attrs, common::DartType};

pub enum GetterBody {
    Expression(String),
    Statement(String),
}

impl DartType {
    pub fn getter_body(
        &self,
        ffi_method: &syn::Ident,
        enum_name: &Option<String>,
    ) -> GetterBody {
        use GetterBody::*;

        // NOTE: quickly hacked together enum resolution support since this code is going to be
        // replaced anyways
        if enum_name.is_some() {
            let enum_name = enum_name.as_ref().unwrap();

            return Expression(format!(
                "{enum_name}.values[{rid_ffi}.{ffi_method}(this)];",
                enum_name = enum_name,
                rid_ffi = RID_FFI,
                ffi_method = ffi_method
            ));
        }

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
            DartType::Custom(_, _) => Expression(format!(
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
            DartType::Custom(info, ty) => {
                use attrs::Category::*;
                match info.cat {
                    // TODO: we are assuming each enum is #[repr(C)]
                    Enum => "int".to_string(),
                    Struct => format!(
                        "{dart_ffi}.Pointer<{ffigen_bind}.{ty}>",
                        dart_ffi = DART_FFI,
                        ffigen_bind = FFI_GEN_BIND,
                        ty = ty
                    ),
                    Prim => todo!("dart::return_type Prim"),
                }
            }
        }
    }

    pub fn type_attribute(&self) -> Option<String> {
        match self {
            DartType::Int32 => {
                Some(format!("@{dart_ffi}.Int32()", dart_ffi = DART_FFI))
            }
            DartType::Int64 => {
                Some(format!("@{dart_ffi}.Int64()", dart_ffi = DART_FFI))
            }
            _ => None,
        }
    }
}
