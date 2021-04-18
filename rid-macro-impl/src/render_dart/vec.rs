use rid_common::{DART_COLLECTION, DART_FFI, FFI_GEN_BIND, RID_FFI};

use crate::{
    common::DartType, parse::rust_type::RustType, render_common::VecAccess,
};

const TEMPLATE_OLD: &str = std::include_str!("./vec_old.dart");
const TEMPLATE: &str = std::include_str!("./vec.dart");

pub(crate) struct ImplementVecOld {
    pub(crate) vec_type: String,
    pub(crate) dart_item_type: DartType,
    pub(crate) fn_len_ident: String,
    pub(crate) fn_get_ident: String,
}

pub(crate) fn render(vec: &ImplementVecOld) -> String {
    let iterated_item_type = if vec.dart_item_type.is_primitive() {
        vec.dart_item_type.to_string()
    } else {
        format!(
            "{dart_ffi}.Pointer<{ffigen_bind}.{dart_item_type}>",
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            dart_item_type = &vec.dart_item_type
        )
    };
    TEMPLATE_OLD
        .replace("{vec_type}", &vec.vec_type)
        .replace("{dart_item_type}", &vec.dart_item_type.to_string())
        .replace("{iterated_item_type}", &iterated_item_type)
        .replace("{fn_len_ident}", &vec.fn_len_ident)
        .replace("{fn_get_ident}", &vec.fn_get_ident)
        .replace("{ffigen_bind}", FFI_GEN_BIND)
        .replace("{dart_ffi}", DART_FFI)
        .replace("{rid_ffi}", RID_FFI)
        .replace("{dart_collection}", DART_COLLECTION)
}

impl VecAccess {
    pub fn render_dart(&self, comment: &str) -> String {
        // TODO: once we have recursive vecs, we need to pass the nested vecs to implement along
        // TODO: this is not working for primitive types since they use RidVecs.
        //  - we can try to reuse the template, but then we cannot include `Pointer<...>` as return
        //  type in it, or we use a different template, i.e. `rid_vec.dart` for those
        let dart_item_type = &self.item_type.render_dart_pointer_type();
        TEMPLATE
            .replace("///", comment)
            .replace("{vec_type}", &self.vec_type_dart)
            .replace("{dart_item_type}", &dart_item_type)
            .replace("{fn_len_ident}", &self.fn_len_ident.to_string())
            .replace("{fn_get_ident}", &self.fn_get_ident.to_string())
            .replace("{fn_free_ident}", &self.fn_free_ident.to_string())
            .replace("{ffigen_bind}", FFI_GEN_BIND)
            .replace("{dart_ffi}", DART_FFI)
            .replace("{rid_ffi}", RID_FFI)
            .replace("{dart_collection}", DART_COLLECTION)
    }
}