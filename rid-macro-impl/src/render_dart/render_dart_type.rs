use rid_common::{DART_FFI, STRING_TO_NATIVE_INT8};
use syn::Ident;

use crate::{
    attrs::{Category, TypeInfoMap},
    common::abort,
    parse::{
        dart_type::DartType,
        rust_type::{self, RustType, TypeKind},
    },
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
            DartType::Vec(inner) => {
                format!("List<{inner}>", inner = inner.render_type())
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
            Bool => format!("arg{} ? 1 : 0", slot),
            String => format!(
                "arg{slot}.{toNativeInt8}()",
                slot = slot,
                toNativeInt8 = STRING_TO_NATIVE_INT8
            ),
            Int32 | Int64 => format!("arg{}", slot),
            Custom(_, _) => format!("arg{}", slot),
            Vec(_) => format!("arg{}", slot),
            Unit => todo!("render_resolved_ffi_arg"),
        }
    }

    pub fn render_to_dart_for_arg(&self, arg_ident: &Ident) -> String {
        use DartType::*;
        match self {
            Int32 | Int64 | Bool => arg_ident.to_string(),
            // NOTE: Raw Strings are already converted to Dart Strings
            String => arg_ident.to_string(),
            Custom(info, _) => {
                if info.is_struct() {
                    // TODO(thlorenz): not 100% sure about this one
                    format!("{arg_ident}.toDart()", arg_ident = arg_ident)
                } else {
                    arg_ident.to_string()
                }
            }
            // NOTE: All vecs are expected have a `toDart` extension method implemented
            // which maps all it's items `toDart` before converting it `toList`
            Vec(_) => format!("{arg_ident}.toDart()", arg_ident = arg_ident),
            Unit => abort!(
                arg_ident,
                "render_to_dart_for_arg makes no sense for Unit types"
            ),
        }
    }
}

impl RustType {
    pub fn render_dart_type(
        &self,
        type_infos: &TypeInfoMap,
        include_type_attribute: bool,
    ) -> String {
        let dart_type = DartType::from(&self, type_infos);
        if include_type_attribute {
            match dart_type.render_type_attribute() {
                Some(attr) => format!("{} {}", attr, dart_type.render_type()),
                None => dart_type.render_type(),
            }
        } else {
            dart_type.render_type()
        }
    }

    pub fn render_dart_and_ffi_type(
        &self,
        type_infos: &TypeInfoMap,
    ) -> (String, Option<String>) {
        let dart_type = DartType::from(&self, type_infos);
        (dart_type.render_type(), dart_type.render_type_attribute())
    }

    pub fn render_to_dart_for_arg(
        &self,
        type_infos: &TypeInfoMap,
        arg_ident: &Ident,
    ) -> String {
        DartType::from(&self, type_infos).render_to_dart_for_arg(arg_ident)
    }
}
