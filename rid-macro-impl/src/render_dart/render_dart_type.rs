use rid_common::{DART_FFI, STRING_TO_NATIVE_INT8};

use crate::{
    attrs::Category,
    parse::{dart_type::DartType, rust_type::RustType},
};

impl DartType {
    fn render_type(&self) -> String {
        match self {
            DartType::Int32 | DartType::Int64 => "int".to_string(),
            DartType::Bool => "bool".to_string(),
            DartType::String => "String".to_string(),
            DartType::Custom(info, ty) => {
                use Category::*;
                match info.cat {
                    Enum => "int".to_string(),
                    Struct | Prim => ty.to_string(),
                }
            }
            DartType::Unit => "".to_string(),
        }
    }

    fn render_type_attribute(&self) -> Option<String> {
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

    pub fn render_resolved_ffi_arg(&self, slot: usize) -> String {
        use DartType::*;
        match self {
            Bool => format!("arg{} == 0 ? false : true", slot),
            String => format!(
                "arg{slot}.{toNativeInt8}()",
                slot = slot,
                toNativeInt8 = STRING_TO_NATIVE_INT8
            ),
            Int32 | Int64 => format!("arg{}", slot),
            Custom(_, _) => format!("arg{}", slot),
            Unit => todo!("render_resolved_ffi_arg"),
        }
    }
}

impl RustType {
    pub fn render_dart_type(&self, include_type_attribute: bool) -> String {
        let dart_type: DartType = self.into();
        if include_type_attribute {
            match dart_type.render_type_attribute() {
                Some(attr) => format!("{} {}", attr, dart_type.render_type()),
                None => dart_type.render_type(),
            }
        } else {
            dart_type.render_type()
        }
    }
}
