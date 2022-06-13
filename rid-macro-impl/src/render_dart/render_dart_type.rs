use quote::format_ident;
use rid_common::{DART_COLLECTION, DART_FFI, STRING_TO_NATIVE_INT8};
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
    /// Renders this type to a valid Dart representation.
    /// Depending on `raw` certain types are represented differently.
    ///
    /// ### Non-Raw Specifics
    ///
    /// Enum: is passed just by it's named type, i.e. 'Filter' normally
    ///
    /// ### Raw Specifics
    ///
    /// Enum: is passed as 'int'
    pub fn render_type(&self, raw: bool) -> String {
        use DartType::*;
        match self {
            // -----------------
            // Primitives
            // -----------------
            Int32(nullable) | Int64(nullable) if *nullable => {
                "int?".to_string()
            }
            Int32(_) | Int64(_) => "int".to_string(),
            Bool(nullable) if *nullable => "bool?".to_string(),
            Bool(_) => "bool".to_string(),
            // -----------------
            // Double
            // -----------------
            Double(nullable) if *nullable => "double?".to_string(),
            Double(_) => "double".to_string(),
            // -----------------
            // Strings
            // -----------------
            String(nullable) if *nullable => "String?".to_string(),
            String(_) => "String".to_string(),
            // -----------------
            // Custom
            // -----------------
            Custom(nullable, info, ty) => {
                use Category::*;
                match info.cat {
                    Enum if *nullable => {
                        if raw {
                            "int?".to_string()
                        } else {
                            format!("{ty}?", ty = ty)
                        }
                    }
                    Enum => {
                        if raw {
                            "int".to_string()
                        } else {
                            ty.to_string()
                        }
                    }
                    Struct | Prim if *nullable => format!("{}?", ty),
                    Struct | Prim => ty.to_string(),
                }
            }
            // -----------------
            // Collection Types
            // -----------------
            Vec(nullable, inner) if *nullable => {
                format!("List<{inner}>?", inner = inner.render_type(raw))
            }
            Vec(_, inner) => {
                format!("List<{inner}>", inner = inner.render_type(raw))
            }
            HashMap(nullable, key, val) if *nullable => {
                format!(
                    "{dart_collection}.HashMap<{key}, {val}>?",
                    dart_collection = DART_COLLECTION,
                    key = key.render_type(raw),
                    val = val.render_type(raw)
                )
            }
            HashMap(_, key, val) => {
                format!(
                    "{dart_collection}.HashMap<{key}, {val}>",
                    dart_collection = DART_COLLECTION,
                    key = key.render_type(raw),
                    val = val.render_type(raw)
                )
            }
            // -----------------
            // Invalid
            // -----------------
            DartType::Unit => "".to_string(),
        }
    }

    pub fn render_type_attribute(&self) -> Option<String> {
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

    fn render_resolved_ffi_var(&self, var: &str) -> String {
        use DartType::*;
        match self {
            // -----------------
            // Primitives
            // -----------------
            // TODO(thlorenz): All the below also would resolve to a different type when nullable
            Int32(nullable) | Int64(nullable) | Double(nullable) if *nullable => {
                format!("{}", var)
            }
            Int32(_) | Int64(_) | Double(_) => format!("{}", var),
            Bool(nullable) if *nullable => {
                format!("{var} == null ? 0 : {var} ? 1 : 0", var = var)
            }
            Bool(_) => format!("{} ? 1 : 0", var),
            // -----------------
            // Strings
            // -----------------
            // TODO(thlorenz): I doubt this is correct
            String(nullable) if *nullable => format!(
                "{var}?.{toNativeInt8}()",
                var = var,
                toNativeInt8 = STRING_TO_NATIVE_INT8
            ),
            String(_) => format!(
                "{var}.{toNativeInt8}()",
                var = var,
                toNativeInt8 = STRING_TO_NATIVE_INT8
            ),

            // -----------------
            // Custom
            // -----------------
            Custom(_, _, _) => format!("{var}", var = var),

            // -----------------
            // Collection Types
            // -----------------
            Vec(_, _) => format!("{var}", var = var),
            HashMap(_, _, _) => format!("{var}", var = var),

            // -----------------
            // Invalid
            // -----------------
            Unit => todo!("render_resolved_ffi_var"),
        }
    }

    pub fn render_resolved_ffi_arg(&self, slot: usize) -> String {
        let arg_name = format!("arg{}", slot);
        self.render_resolved_ffi_var(&arg_name)
    }

    pub fn render_to_dart_for_snippet(&self, snip: &str) -> String {
        use DartType::*;
        match self {
            // -----------------
            // Primitives
            // -----------------
            Int32(nullable) | Int64(nullable) | Bool(nullable) | Double(nullable) if *nullable => {
                format!("{}?", snip)
            }
            Int32(_) | Int64(_) | Bool(_) | Double(_) => snip.to_string(),

            // -----------------
            // Strings
            // -----------------
            // NOTE: Raw Strings are already converted to Dart Strings
            String(nullable) if *nullable => {
                format!("{snip}?", snip = snip)
            }
            String(_) => snip.to_string(),

            // -----------------
            // Custom
            // -----------------
            Custom(nullable, info, type_name) if *nullable => {
                use Category::*;
                match info.cat {
                    // i.e. () { final x = store.filter; return x != null ? Filter.values[x] : null; }()
                    Enum => format!(
                        "() {{ final x = {snip}; return x != null ? {type_name}.values[x] : null; }}()",
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
                        "{type_name}.values[{snip}]",
                        type_name = type_name,
                        snip = snip
                    ),
                    Struct => {
                        format!("{snip}.toDart()", snip = snip)
                    }
                    Prim => snip.to_string(),
                }
            }

            // -----------------
            // Collection Types
            // -----------------

            // NOTE: All vecs are expected have a `toDart` extension method implemented
            // which maps all it's items `toDart` before converting it `toList`
            Vec(nullable, _) if *nullable => {
                format!("{snip}?.toDart()", snip = snip)
            }
            Vec(_, _) => format!("{snip}.toDart()", snip = snip),

            // NOTE: All hashmaps are expected have a `toDart` extension method implemented
            // which maps all it's keys/values `toDart` before converting it `toHashMap`
            HashMap(nullable, _, _) if *nullable => {
                format!("{snip}?.toDart()", snip = snip)
            }
            HashMap(_, _, _) => format!("{snip}.toDart()", snip = snip),

            // -----------------
            // Invalid
            // -----------------
            Unit => {
                abort!(
                    format_ident!("()"),
                    "render_to_dart makes no sense for Unit types"
                )
            }
        }
    }
}

pub struct RenderDartTypeOpts {
    pub raw: bool,
    pub include_type_attribute: bool,
}

impl RenderDartTypeOpts {
    pub fn attr_raw() -> Self {
        Self {
            raw: true,
            include_type_attribute: true,
        }
    }
    pub fn raw() -> Self {
        Self {
            raw: true,
            include_type_attribute: false,
        }
    }
    pub fn attr() -> Self {
        Self {
            raw: false,
            include_type_attribute: true,
        }
    }
    pub fn plain() -> Self {
        Self {
            raw: false,
            include_type_attribute: false,
        }
    }
}

impl RustType {
    pub fn render_dart_type(
        &self,
        type_infos: &TypeInfoMap,
        opts: RenderDartTypeOpts,
    ) -> String {
        let dart_type = DartType::from(&self, type_infos);
        let RenderDartTypeOpts {
            raw,
            include_type_attribute,
        } = opts;

        if include_type_attribute {
            match dart_type.render_type_attribute() {
                Some(attr) => {
                    format!("{} {}", attr, dart_type.render_type(raw))
                }
                None => dart_type.render_type(raw),
            }
        } else {
            dart_type.render_type(raw)
        }
    }

    pub fn render_dart_and_ffi_type(
        &self,
        type_infos: &TypeInfoMap,
        raw: bool,
    ) -> (String, Option<String>) {
        let dart_type = DartType::from(&self, type_infos);
        (
            dart_type.render_type(raw),
            dart_type.render_type_attribute(),
        )
    }

    pub fn render_to_dart_for_arg(
        &self,
        type_infos: &TypeInfoMap,
        snip: &str,
    ) -> String {
        DartType::from(&self, type_infos).render_to_dart_for_snippet(snip)
    }

    /// Renders the provided variable which is assumed to have a plain Dart type including
    /// conversion to FFI Dart type, i.e. `var.toNative()` for [Strings].
    /// For all other cases it just renders the name of the `var`.
    pub fn render_dart_resolved_ffi_var(
        &self,
        type_infos: &TypeInfoMap,
        var: &str,
    ) -> String {
        DartType::from(&self, type_infos).render_resolved_ffi_var(var)
    }
}
