use crate::{attrs::TypeInfo, common::abort, parse::rust_type::RustType};
use rid_common::{DART_FFI, STRING_TO_NATIVE_INT8};

#[derive(Debug)]
pub enum DartType {
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
                abort!(rust_type.rust_ident(), "No simple conversion from composite rust type to dart type exists")
            }
            Unknown => {
                abort!(
                    rust_type.rust_ident(),
                    "Cannot convert unknown rust type to dart type"
                )
            }
        }
    }
}
