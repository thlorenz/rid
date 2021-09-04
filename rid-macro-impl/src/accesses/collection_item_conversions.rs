use crate::{
    attrs::TypeInfoMap,
    parse::{dart_type::DartType, rust_type::RustType},
};

pub fn resolved_dart_item_type_string(
    item_type: &RustType,
    type_infos: &TypeInfoMap,
) -> String {
    if item_type.is_struct() {
        item_type.rust_ident().to_string()
    } else if item_type.is_string_like() {
        "String".to_string()
    } else {
        DartType::from(&item_type, type_infos).render_type(false)
    }
}

pub fn map_to_dart_string(
    item_type: &RustType,
    dart_item_type: &str,
) -> String {
    if item_type.is_struct() {
        format!(".map((raw) => raw.toDart())")
    } else if item_type.is_enum() {
        format!(
            ".map((x) => {enum_type}.values[x])",
            enum_type = dart_item_type
        )
    } else {
        "".to_string()
    }
}
