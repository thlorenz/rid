use crate::{
    attrs::{TypeInfo, TypeInfoMap},
    common::abort,
    parse::rust_type::RustType,
};
use rid_common::{DART_FFI, STRING_TO_NATIVE_INT8};

/// The Dart type representing a given Rust Type.
/// The first field indicates if the Rust Type is `Option<T>` and thus the Dart type is nullable.
#[derive(Debug, PartialEq)]
pub enum DartType {
    Int32(bool),
    Int64(bool),
    Bool(bool),
    String(bool),
    Custom(bool, TypeInfo, String),
    Vec(bool, Box<DartType>),
    Unit,
}

impl DartType {
    pub fn from(rust_type: &RustType, type_infos: &TypeInfoMap) -> Self {
        DartType::from_with_nullable(rust_type, type_infos, false)
    }

    pub fn from_with_nullable(
        rust_type: &RustType,
        type_infos: &TypeInfoMap,
        nullable: bool,
    ) -> Self {
        use crate::parse::rust_type::{
            Composite as C, Primitive as P, TypeKind::*, Value as V,
        };
        match &rust_type.kind {
            Primitive(p) => {
                use crate::parse::rust_type::Primitive::*;
                match p {
                    U8 | I8 | U16 | I16 | U32 | I32 => {
                        DartType::Int32(nullable)
                    }
                    U64 | I64 => DartType::Int64(nullable),
                    USize => DartType::Int64(nullable),
                    // assuming 64-bit target
                    Bool => DartType::Bool(nullable),
                }
            }
            Value(v) => {
                use crate::parse::rust_type::Value::*;
                match v {
                    CString | String | Str => DartType::String(nullable),
                    Custom(info, ty) => {
                        DartType::Custom(nullable, info.clone(), ty.to_string())
                    }
                }
            }
            Unit => DartType::Unit,
            Composite(composite, ty) => match composite {
                C::Vec => {
                    let inner = DartType::from_with_nullable(
                        ty.as_ref()
                            .expect("Vec Composite should have inner type")
                            .as_ref(),
                        type_infos,
                        false,
                    );
                    DartType::Vec(nullable, Box::new(inner))
                }
                C::Option => DartType::from_with_nullable(
                    ty.as_ref().unwrap(),
                    type_infos,
                    true,
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
