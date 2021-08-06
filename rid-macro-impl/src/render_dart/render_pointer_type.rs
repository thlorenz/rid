use quote::{format_ident, quote_spanned};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};
use syn::Ident;

use super::vec::*;
use crate::{
    attrs::{Category, TypeInfoMap},
    common::{abort, state::get_state},
    parse::{
        rust_type::{Composite, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_common::PointerTypeAlias,
    render_dart::RenderDartTypeOpts,
};

impl RustType {
    pub fn render_dart_pointer_type(&self) -> String {
        use TypeKind as K;
        match &self.kind {
            K::Primitive(_) => self.render_dart_type(
                &TypeInfoMap::default(),
                RenderDartTypeOpts::raw(),
            ),
            K::Unit => abort!(
                self.rust_ident(),
                "Should not export rust method that returns nothing"
            ),

            K::Value(val) => {
                val.render_dart_pointer_type(self.dart_wrapper_rust_ident())
            }
            K::Composite(Composite::Vec, inner_type, _) => match inner_type {
                Some(ty) => {
                    let item_type = ty.rust_ident();
                    let pointer = format!(
                        "{ffigen_bind}.RidVec_{pointer_prefix}{ty}",
                        pointer_prefix = PointerTypeAlias::POINTER_ALIAS_PREFIX,
                        ffigen_bind = FFI_GEN_BIND,
                        ty = item_type
                    );
                    pointer
                }
                None => {
                    abort!(
                        self.rust_ident(),
                        "Rust Vec composite should include inner type"
                    )
                }
            },
            K::Composite(Composite::Option, inner_type, _) => {
                match inner_type {
                    Some(ty) => {
                        let pointer = format!(
                            "{dart_ffi}.Pointer<{ffigen_bind}.{ty}>?",
                            dart_ffi = DART_FFI,
                            ffigen_bind = FFI_GEN_BIND,
                            ty = inner_type
                                .as_ref()
                                .unwrap()
                                .dart_wrapper_rust_ident(),
                        );
                        pointer
                    }
                    None => {
                        abort!(
                            self.rust_ident(),
                            "Rust Option composite should include inner type"
                        )
                    }
                }
            }
            K::Composite(Composite::HashMap, key_type, val_type) => {
                // TODO(thlorenz): HashMap
                abort!(
                    self.rust_ident(),
                    "TODO: RustType::render_dart_pointer_type K::Composite::HashMap<{:?}, {:?}>",
                    key_type, val_type
                )
            }
            K::Composite(kind, _, _) => {
                abort!(
                    self.rust_ident(),
                    "TODO: RustType::render_dart_pointer_type K::Composite({:?})",
                    kind
                )
            }
            K::Unknown => abort!(
                self.rust_ident(),
                "TODO: RustType::render_dart_pointer_type K::Unknown"
            ),
        }
    }
}

impl Value {
    fn render_dart_pointer_type(&self, ident: &Ident) -> String {
        use Category as C;
        use Value::*;
        match self {
            // TODO(thlorenz): implement to_return_type for dart with the string conversion
            CString => "String".to_string(),
            String => "String".to_string(),
            Str => todo!("Value::render_dart_pointer_type ::Str"),
            Custom(type_info, _) => match type_info.cat {
                // NOTE: assumes that enums are `repr(C)`.
                // If they are `repr(u8)` this would have to be a Uint8
                C::Enum => format!(
                    "{dart_ffi}.Pointer<{dart_ffi}.Int32>",
                    dart_ffi = DART_FFI
                ),
                C::Struct => format!(
                    "{dart_ffi}.Pointer<{ffigen_bind}.{type_name}>",
                    dart_ffi = DART_FFI,
                    ffigen_bind = FFI_GEN_BIND,
                    type_name = ident,
                ),
                C::Prim => format!(
                    "{dart_ffi}.Pointer<{ffigen_bind}.{type_name}>",
                    dart_ffi = DART_FFI,
                    ffigen_bind = FFI_GEN_BIND,
                    type_name = ident,
                ),
            },
        }
    }
}
