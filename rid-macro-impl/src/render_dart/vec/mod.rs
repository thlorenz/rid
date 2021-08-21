use heck::{CamelCase, SnakeCase};
use rid_common::{
    DART_COLLECTION, DART_FFI, FFI_GEN_BIND, RID_FFI, STRING_REF_ACCESS,
};

use crate::{
    attrs::TypeInfoMap,
    parse::{dart_type::DartType, rust_type::RustType},
    render_common::{AccessKind, RenderableAccess, VecAccess},
};

use super::RenderDartTypeOpts;

const TEMPLATE_FIELD_ACCESS: &str =
    std::include_str!("./vec_field_access.dart");
const TEMPLATE: &str = std::include_str!("./rid_vec.dart");

impl VecAccess {
    pub fn render_dart_for_field_reference(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        let pointer_vec_type = self.vec_type.render_dart_field_return_type();
        let vec_type = self.key().to_camel_case();
        let iterated_item_type = self.item_type.render_dart_field_return_type();

        let resolved_dart_item_type =
            self.resolved_dart_item_type_string(type_infos);

        let map_to_dart = self.map_to_dart_string(&resolved_dart_item_type);

        let item_to_dart = if self.item_type.is_string_like() {
            ".toDartString()".to_string()
        } else {
            "".to_string()
        };

        TEMPLATE_FIELD_ACCESS
            .replace("/// ", comment)
            .replace("{vec_type}", &vec_type.to_string())
            .replace("{pointer_vec_type}", &pointer_vec_type)
            .replace("{resolved_dart_item_type}", &resolved_dart_item_type)
            .replace("{iterated_item_type}", &iterated_item_type)
            .replace("{item_to_dart}", &item_to_dart)
            .replace("{map_to_dart}", &map_to_dart)
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace("{ffigen_bind}", FFI_GEN_BIND)
            .replace("{dart_ffi}", DART_FFI)
            .replace("{rid_ffi}", RID_FFI)
            .replace("{dart_collection}", DART_COLLECTION)
    }

    pub fn render_dart_return_from_method(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        let resolved_dart_item_type =
            self.resolved_dart_item_type_string(type_infos);
        let map_to_dart = self.map_to_dart_string(&resolved_dart_item_type);

        let access_item_return = if self.item_type.is_string_like()
            && !self.item_type.reference.is_owned()
        {
            "return raw.toDartString();".to_string()
        } else {
            "return raw;".to_string()
        };
        let dart_raw_item_type = &self.item_type.render_dart_pointer_type();
        TEMPLATE
            .replace("///", comment)
            .replace("{vec_type}", &self.vec_type_dart)
            .replace("{dart_raw_item_type}", &dart_raw_item_type)
            .replace("{resolved_dart_item_type}", &resolved_dart_item_type)
            .replace("{map_to_dart}", &map_to_dart)
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace("{fn_free_ident}", &self.fn_free_ident.to_string())
            .replace("{access_item_return}", &access_item_return)
            .replace("{ffigen_bind}", FFI_GEN_BIND)
            .replace("{dart_ffi}", DART_FFI)
            .replace("{rid_ffi}", RID_FFI)
            .replace("{dart_collection}", DART_COLLECTION)
    }

    // -----------------
    // Common
    // -----------------
    fn resolved_dart_item_type_string(
        &self,
        type_infos: &TypeInfoMap,
    ) -> String {
        if self.item_type.is_struct() {
            self.item_type.rust_ident().to_string()
        } else if self.item_type.is_string_like() {
            "String".to_string()
        } else {
            DartType::from(&self.item_type, type_infos).render_type(false)
        }
    }

    fn map_to_dart_string(&self, dart_item_type: &str) -> String {
        if self.item_type.is_struct() {
            format!(".map((raw) => raw.toDart())")
        } else if self.item_type.is_enum() {
            format!(
                ".map((x) => {enum_type}.values[x])",
                enum_type = dart_item_type
            )
        } else {
            "".to_string()
        }
    }
}
