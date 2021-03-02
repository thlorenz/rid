use rid_common::{DART_COLLECTION, DART_FFI, FFI_GEN_BIND, RID_FFI};

use crate::dart::DartType;

const TEMPLATE: &str = std::include_str!("./vec.dart");

pub(crate) struct ImplementVec {
    pub(crate) vec_type: String,
    pub(crate) dart_item_type: DartType,
    pub(crate) fn_len_ident: String,
    pub(crate) fn_get_ident: String,
}

pub(crate) fn render(vec: &ImplementVec) -> String {
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
    TEMPLATE
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
