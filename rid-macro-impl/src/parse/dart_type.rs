use crate::{
    attrs::{TypeInfo, TypeInfoMap},
    common::abort,
    parse::rust_type::RustType,
};
use rid_common::{DART_FFI, STRING_TO_NATIVE_INT8};

#[derive(Debug, PartialEq)]
pub enum DartType {
    Int32,
    Int64,
    Bool,
    String,
    Custom(TypeInfo, String),
    Vec(Box<DartType>),
    Unit,
}

impl DartType {
    pub fn from(rust_type: &RustType, type_infos: &TypeInfoMap) -> Self {
        use crate::parse::rust_type::{
            Composite as C, Primitive as P, TypeKind::*, Value as V,
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
            Composite(composite, ty) => match composite {
                C::Vec => {
                    let inner = DartType::from(
                        ty.as_ref()
                            .expect("Vec Composite should have inner type")
                            .as_ref(),
                        type_infos,
                    );
                    DartType::Vec(Box::new(inner))
                }
                C::Option => abort!(
                    rust_type.rust_ident(),
                    "TODO: convert option composite rust type to dart type"
                ),
                C::Custom(_, _) => abort!(
                    rust_type.rust_ident(),
                    "TODO: convert custom composite rust type to dart type"
                ),
            },
            Unknown => {
                abort!(
                    rust_type.rust_ident(),
                    "Cannot convert unknown rust type to dart type"
                )
            }
        }
    }
}
