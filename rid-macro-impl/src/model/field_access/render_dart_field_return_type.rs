use rid_common::{DART_FFI, FFI_GEN_BIND};

use crate::{
    attrs::{Category, TypeInfoMap},
    common::abort,
    parse::{
        rust_type::{Composite, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_dart::RenderDartTypeOpts,
};

impl RustType {
    /// Very similar to [RustType::render_dart_pointer_type] but handles some types like Vec
    /// differently.
    /// For others it forwards to [RustType::render_dart_pointer_type].
    pub fn render_dart_field_return_type(&self) -> String {
        use TypeKind as K;
        match &self.kind {
            // -----------------
            // Primitives
            // -----------------
            K::Primitive(_) => self.render_dart_pointer_type(),
            // -----------------
            // Strings
            // -----------------
            K::Value(val) => match val {
                Value::CString | Value::String | Value::Str => {
                    "String".to_string()
                }
                Value::Custom(type_info, _) => match &type_info.cat {
                    Category::Enum => "int".to_string(),
                    Category::Struct => self.render_dart_pointer_type(),
                    Category::Prim => abort!(
                        self.rust_ident(),
                        "RustType::render_dart_field_return_type:K::Value:Custom:Prim"
                    ),
                },
            },
            // -----------------
            // Collection Types
            // -----------------
            K::Composite(Composite::Vec, inner_type, _) => match inner_type {
                Some(ty) => {
                    let item_type = ty.rust_ident();
                    format!(
                        "{dart_ffi}.Pointer<{ffigen_bind}.Vec_{ty}>",
                        dart_ffi = DART_FFI,
                        ffigen_bind = FFI_GEN_BIND,
                        ty = item_type
                    )
                }
                None => {
                    abort!(
                        self.rust_ident(),
                        "Rust Vec composite should include inner type"
                    )
                }
            },
            K::Composite(Composite::HashMap, key_type, val_type) => match (key_type, val_type) {
                (Some(key), Some(val)) => {
                    let key_type = key.rust_ident();
                    let val_type = val.rust_ident();
                    format!(
                        "{dart_ffi}.Pointer<{ffigen_bind}.HashMap_{key}__{val}>",
                        dart_ffi = DART_FFI,
                        ffigen_bind = FFI_GEN_BIND,
                        key = key_type,
                        val  = val_type
                    )
                }
                _ => {
                    abort!(
                        self.rust_ident(),
                        "Rust HashMap composite should include key and val type"
                    )
                }
            },
            // -----------------
            // Option
            // -----------------
            K::Composite(Composite::Option, inner_type, _) => {
                self.render_dart_pointer_type()
            }
            // -----------------
            // Not yet supported
            // -----------------
            K::Composite(kind, _, _) => {
                abort!(
                    self.rust_ident(),
                    "TODO: RustType::render_dart_field_return_type K::Composite({:?})",
                    kind
                )
            }
            // -----------------
            // Invalid
            // -----------------
            K::Unit => {
                abort!(self.rust_ident(), "Should not include unit field type")
            }
            K::Unknown => abort!(
                self.rust_ident(),
                "Cannot RustType::render_dart_field_return_type K::Unknown"
            ),
        }
    }
}
