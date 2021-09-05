use rid_common::{DART_COLLECTION, RID_FFI};

use crate::{
    accesses::{
        map_to_dart_string, resolved_dart_item_type_string, HashMapAccess,
    },
    attrs::TypeInfoMap,
};

const TEMPLATE_FIELD_ACCESS: &str =
    std::include_str!("./hash_map_field_access.dart");

impl HashMapAccess {
    pub fn render_dart_for_field_reference(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        let pointer_hash_map_type =
            self.hash_map_type.render_dart_field_return_type();
        let key_ffi_arg = self
            .key_type
            .render_dart_resolved_ffi_var(type_infos, "key");

        // NOTE: this works as long as key/val types only have alpha-numeric chars
        let hash_map_type = format!(
            "HashMap_{key}__{val}",
            key = self.key_type.rust_ident(),
            val = self.val_type.rust_ident()
        );

        let resolved_dart_key_type =
            resolved_dart_item_type_string(&self.key_type, type_infos);
        let resolved_dart_val_type =
            resolved_dart_item_type_string(&self.val_type, type_infos);

        // TODO(thlorenz): does this `to_dart` logic already living somewhere else?
        // If not we should extract it as it is used similarly inside ./render_dart/vec/mod.rs
        let key_to_dart = if self.key_type.is_string_like() {
            ".toDartString()".to_string()
        } else {
            "".to_string()
        };

        let val_to_dart = if self.val_type.is_string_like() {
            ".toDartString()".to_string()
        } else if self.val_type.is_primitive() {
            ".value".to_string()
        } else {
            ".toDart()".to_string()
        };

        TEMPLATE_FIELD_ACCESS
            .replace("/// ", comment)
            .replace("{hash_map_type}", &hash_map_type)
            .replace("{pointer_hash_map_type}", &pointer_hash_map_type)
            // key
            .replace("{key_ffi_arg}", &key_ffi_arg)
            .replace("{resolved_dart_key_type}", &resolved_dart_key_type)
            .replace("{key_to_dart}", &key_to_dart)
            // val
            .replace("{resolved_dart_val_type}", &resolved_dart_val_type)
            .replace("{val_to_dart}", &val_to_dart)
            // fn idents
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace(
                "{fn_contains_key_ident}",
                &self.fn_contains_key_ident.to_string(),
            )
            .replace("{fn_keys_ident}", &self.fn_keys_ident.to_string())
            // namespaces
            .replace("{rid_ffi}", RID_FFI)
            .replace("{dart_collection}", DART_COLLECTION)
    }

    pub fn render_dart_return_from_method(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        // TODO(thlorenz): HashMap
        "".to_string()
    }
}
