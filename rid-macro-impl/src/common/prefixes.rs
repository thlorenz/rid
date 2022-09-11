use heck::ToLowerCamelCase;
use quote::format_ident;
use syn::Ident;

pub fn reply_class_name_for_enum(enum_name: &str) -> String {
    format!("Posted{}", enum_name)
}

pub fn store_field_ident(store_ident: &Ident) -> Ident {
    format_ident!("_{}", store_ident.to_string().to_lower_camel_case())
}

pub fn store_state_class_ident(store_ident: &Ident) -> Ident {
    format_ident!("{}State", store_ident)
}
