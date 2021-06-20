use quote::format_ident;
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
        use DartType::*;
        match self {
            Int32(nullable) | Int64(nullable) if *nullable => {
                "int?".to_string()
            }
            Int32(_) | Int64(_) => "int".to_string(),
            Bool(nullable) if *nullable => "bool?".to_string(),
            Bool(_) => "bool".to_string(),
            String(nullable) if *nullable => "String?".to_string(),
            String(_) => "String".to_string(),
            Custom(nullable, info, ty) => {
                use Category::*;
                match info.cat {
                    Enum if *nullable => "int?".to_string(),
                    Enum => "int".to_string(),
                    Struct | Prim if *nullable => format!("{}?", ty),
                    Struct | Prim => ty.to_string(),
                }
            }
            Vec(nullable, inner) if *nullable => {
                format!("List<{inner}>?", inner = inner.render_type())
            }
            Vec(_, inner) => {
                format!("List<{inner}>", inner = inner.render_type())
            }
            DartType::Unit => "".to_string(),
        }
    }

    fn render_type_attribute(&self) -> Option<String> {
        match self {
            DartType::Int32(_) => {
                Some(format!("@{dart_ffi}.Int32()", dart_ffi = DART_FFI))
            }
            DartType::Int64(_) => {
                Some(format!("@{dart_ffi}.Int64()", dart_ffi = DART_FFI))
            }
            _ => None,
        }
    }

    pub fn render_resolved_ffi_arg(&self, slot: usize) -> String {
        use DartType::*;
        match self {
            Bool(nullable) if *nullable => {
                format!(
                    "arg{slot} == null ? 0 : arg{slot} ? 1 : 0",
                    slot = slot
                )
            }
            Bool(_) => format!("arg{} ? 1 : 0", slot),
            // TODO(thlorenz): I doubt his is correct
            String(nullable) if *nullable => format!(
                "arg{slot}?.{toNativeInt8}()",
                slot = slot,
                toNativeInt8 = STRING_TO_NATIVE_INT8
            ),
            String(_) => format!(
                "arg{slot}.{toNativeInt8}()",
                slot = slot,
                toNativeInt8 = STRING_TO_NATIVE_INT8
            ),
            // TODO(thlorenz): All the below also would resolve to a different type when nullable
            Int32(nullable) | Int64(nullable) if *nullable => {
                format!("arg{}", slot)
            }
            Int32(_) | Int64(_) => format!("arg{}", slot),
            Custom(_, _, _) => format!("arg{}", slot),
            Vec(_, _) => format!("arg{}", slot),
            Unit => todo!("render_resolved_ffi_arg"),
        }
    }

    pub fn render_to_dart_for_snippet(&self, snip: &str) -> String {
        use DartType::*;
        match self {
            Int32(nullable) | Int64(nullable) | Bool(nullable) if *nullable => {
                format!("{snip}?", snip = snip)
            }
            Int32(_) | Int64(_) | Bool(_) => snip.to_string(),
            // NOTE: Raw Strings are already converted to Dart Strings
            String(nullable) if *nullable => {
                format!("{snip}?", snip = snip)
            }
            String(_) => snip.to_string(),
            Custom(nullable, info, type_name) if *nullable => {
                use Category::*;
                match info.cat {
                    // i.e. () { final x = store.filter; return x != null ? Filter.values[x] : null; }()
                    Enum => format!(
                        "() {{ final x = {snip}; return x != null ? {type_name}.value[x] : null; }}()",
                        type_name = type_name,
                        snip = snip
                    ),
                    Struct => {
                        format!("{snip}?.toDart()", snip = snip)
                    }
                    Prim => format!("{snip}?", snip = snip),
                }
            }
            Custom(_, info, type_name) => {
                use Category::*;
                match info.cat {
                    // i.e. Filter.values[store.filter]
                    Enum => format!(
                        "{type_name}.value[{snip}?]",
                        type_name = type_name,
                        snip = snip
                    ),
                    Struct => {
                        format!("{snip}.toDart()", snip = snip)
                    }
                    Prim => snip.to_string(),
                }
            }
            // NOTE: All vecs are expected have a `toDart` extension method implemented
            // which maps all it's items `toDart` before converting it `toList`
            Vec(nullable, _) if *nullable => {
                format!("{snip}?.toDart()", snip = snip)
            }
            Vec(_, _) => format!("{snip}.toDart()", snip = snip),
            Unit => {
                abort!(
                    format_ident!("()"),
                    "render_to_dart makes no sense for Unit types"
                )
            }
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
        DartType::from(&self, type_infos)
            .render_to_dart_for_snippet(arg_ident.to_string().as_str())
    }
}
