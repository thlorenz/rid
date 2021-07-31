use heck::{CamelCase, SnakeCase};
use rid_common::{DART_COLLECTION, DART_FFI, FFI_GEN_BIND, RID_FFI};

use crate::{
    attrs::TypeInfoMap,
    parse::rust_type::RustType,
    render_common::{VecAccess, VecKind},
};

use super::RenderDartTypeOpts;

const TEMPLATE_FIELD_ACCESS: &str =
    std::include_str!("./vec_field_access.dart");
const TEMPLATE: &str = std::include_str!("./vec.dart");

impl VecAccess {
    pub fn render_dart(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        match self.kind {
            VecKind::FieldReference => {
                self.render_dart_for_field_reference(comment)
            }
            VecKind::MethodReturn => {
                self.render_dart_return_from_method(type_infos, comment)
            }
        }
    }

    fn render_dart_for_field_reference(&self, comment: &str) -> String {
        let pointer_vec_type = self.vec_type.render_dart_field_return_type();
        let vec_type = self.key().to_camel_case();
        let iterated_item_type = self.item_type.render_dart_field_return_type();
        let resolved_dart_item_type = self.item_type.rust_ident();
        let map_to_dart = if self.item_type.is_struct() {
            format!(".map((raw) => raw.toDart())")
        } else {
            "".to_string()
        };
        TEMPLATE_FIELD_ACCESS
            .replace("/// ", comment)
            .replace("{vec_type}", &vec_type.to_string())
            .replace("{pointer_vec_type}", &pointer_vec_type)
            .replace(
                "{resolved_dart_item_type}",
                &resolved_dart_item_type.to_string(),
            )
            .replace("{iterated_item_type}", &iterated_item_type)
            .replace("{map_to_dart}", &map_to_dart)
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace("{ffigen_bind}", FFI_GEN_BIND)
            .replace("{dart_ffi}", DART_FFI)
            .replace("{rid_ffi}", RID_FFI)
            .replace("{dart_collection}", DART_COLLECTION)
    }

    fn render_dart_return_from_method(
        &self,
        type_infos: &TypeInfoMap,
        comment: &str,
    ) -> String {
        // TODO: once we have recursive vecs, we need to pass the nested vecs to implement along
        // TODO: this is not working for primitive types since they use RidVecs.
        //  - we can try to reuse the template, but then we cannot include `Pointer<...>` as return
        //  type in it, or we use a different template, i.e. `rid_vec.dart` for those
        let dart_item_type = &self
            .item_type
            .render_dart_type(type_infos, RenderDartTypeOpts::raw());

        let map_to_dart = if self.item_type.is_struct() {
            format!(".map((raw) => raw.toDart())")
        } else {
            "".to_string()
        };
        let dart_raw_item_type = &self.item_type.render_dart_pointer_type();
        TEMPLATE
            .replace("///", comment)
            .replace("{vec_type}", &self.vec_type_dart)
            .replace("{dart_item_type}", &dart_item_type)
            .replace("{dart_raw_item_type}", &dart_raw_item_type)
            .replace("{map_to_dart}", &map_to_dart)
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace("{fn_free_ident}", &self.fn_free_ident.to_string())
            .replace("{ffigen_bind}", FFI_GEN_BIND)
            .replace("{dart_ffi}", DART_FFI)
            .replace("{rid_ffi}", RID_FFI)
            .replace("{dart_collection}", DART_COLLECTION)
    }
}
