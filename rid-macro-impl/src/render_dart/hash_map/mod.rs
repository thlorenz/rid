use rid_common::{DART_COLLECTION, RID_FFI};

use crate::{accesses::HashMapAccess, attrs::TypeInfoMap};

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
        let key_type = self.key_type.render_dart_field_return_type();
        let key_ffi_arg = self
            .key_type
            .render_dart_resolved_ffi_var(type_infos, "key");
        let val_return_type = self.val_type.render_dart_field_return_type();
        let val_type = self.val_type.render_dart_field_return_type();

        // NOTE: this works as long as key/val types only have alpha-numeric chars
        let hash_map_type = format!(
            "HashMap_{key}__{val}",
            key = self.key_type.rust_ident(),
            val = self.val_type.rust_ident()
        );

        TEMPLATE_FIELD_ACCESS
            .replace("/// ", comment)
            .replace("{hash_map_type}", &hash_map_type)
            .replace("{pointer_hash_map_type}", &pointer_hash_map_type)
            .replace("{key_type}", &key_type)
            .replace("{key_ffi_arg}", &key_ffi_arg)
            .replace("{val_type}", &val_type)
            .replace("{val_return_type}", &val_return_type)
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace(
                "{fn_contains_key_ident}",
                &self.fn_contains_key_ident.to_string(),
            )
            .replace("{fn_keys_ident}", &self.fn_keys_ident.to_string())
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
