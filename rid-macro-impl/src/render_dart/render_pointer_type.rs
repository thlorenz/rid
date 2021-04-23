use quote::{format_ident, quote_spanned};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

use super::vec::*;
use crate::{
    attrs::Category,
    common::{abort, state::get_state},
    parse::{
        rust_type::{Composite, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_rust::TypeAlias,
};

impl RustType {
    pub fn render_dart_pointer_type(&self) -> String {
        use TypeKind as K;
        match &self.kind {
            K::Primitive(_) => self.render_dart_type(false),
            K::Unit => abort!(
                self.ident,
                "Should not export rust method that returns nothing"
            ),

            K::Value(val) => val.render_dart_pointer_type(),
            K::Composite(Composite::Vec, inner_type) => match inner_type {
                Some(ty) => {
                    let item_type = ty.ident.to_string();

                    let pointer = format!(
                        "{ffigen_bind}.RidVec_{pointer_prefix}{ty}",
                        pointer_prefix = TypeAlias::POINTER_ALIAS_PREFIX,
                        ffigen_bind = FFI_GEN_BIND,
                        ty = &item_type
                    );
                    pointer
                }
                None => {
                    abort!(
                        self.ident,
                        "Rust composite should include inner type"
                    )
                }
            },
            K::Composite(_, _) => {
                abort!(
                    self.ident,
                    "TODO: RustType::render_dart_pointer_type K::Composite"
                )
            }
            K::Unknown => abort!(
                self.ident,
                "TODO: RustType::render_dart_pointer_type K::Unknown"
            ),
        }
    }
}

impl Value {
    fn render_dart_pointer_type(&self) -> String {
        use Category as C;
        use Value::*;
        match self {
            CString => todo!("Value::render_dart_pointer_type ::CString"),
            String => todo!("Value::render_dart_pointer_type ::String"),
            Str => todo!("Value::render_dart_pointer_type ::Str"),
            Custom(type_info, type_name) => match type_info.cat {
                C::Enum => format!(
                    "{dart_ffi}.Pointer<{dart_ffi}.Int32>",
                    dart_ffi = DART_FFI
                ),
                C::Struct => format!(
                    "{dart_ffi}.Pointer<{ffigen_bind}.{type_name}>",
                    dart_ffi = DART_FFI,
                    ffigen_bind = FFI_GEN_BIND,
                    type_name = type_name,
                ),
                C::Prim => format!(
                    "{dart_ffi}.Pointer<{ffigen_bind}.{type_name}>",
                    dart_ffi = DART_FFI,
                    ffigen_bind = FFI_GEN_BIND,
                    type_name = type_name,
                ),
            },
        }
    }
}
