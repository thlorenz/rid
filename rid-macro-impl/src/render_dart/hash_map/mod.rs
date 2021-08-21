use crate::{attrs::TypeInfoMap, render_common::HashMapAccess};

impl HashMapAccess {
    pub fn render_dart_for_field_reference(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        // TODO(thlorenz): HashMap
        "".to_string()
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
