use crate::{attrs::TypeInfo, common::abort, parse::rust_type::RustType};
use rid_common::DART_FFI;

enum DartType {
    Int32,
    Int64,
    Bool,
    String,
    Custom(TypeInfo, String),
    Unit,
}

impl From<&RustType> for DartType {
    fn from(rust_type: &RustType) -> Self {
        use crate::parse::rust_type::{
            Primitive as P, TypeKind::*, Value as V,
        };
        match &rust_type.kind {
            Primitive(p) => {
                use crate::parse::rust_type::Primitive::*;
                match p {
                    U8 | I8 | U16 | I16 | U32 | I32 => DartType::Int32,
                    U64 | I64 => DartType::Int64,
                    USize => DartType::Int64,
                    // assuming 64-bit target
                    Bool => DartType::Bool,
                }
            }
            Value(v) => {
                use crate::parse::rust_type::Value::*;
                match v {
                    CString | String | Str => DartType::String,
                    Custom(info, ty) => {
                        DartType::Custom(info.clone(), ty.to_string())
                    }
                }
            }
            Unit => DartType::Unit,
            Composite(_, _) => {
                abort!(rust_type.ident, "No simple conversion from composite rust type to dart type exists")
            }
            Unknown => {
                abort!(
                    rust_type.ident,
                    "Cannot convert unknown rust type to dart type"
                )
            }
        }
    }
}

impl DartType {
    fn render_type(&self) -> String {
        match self {
            DartType::Int32 | DartType::Int64 => "int".to_string(),
            DartType::Bool => "bool".to_string(),
            DartType::String => "String".to_string(),
            DartType::Custom(_, ty) => ty.to_string(),
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
