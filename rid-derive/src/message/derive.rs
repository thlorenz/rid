use super::parsed_enum::ParsedEnum;
use crate::common::errors::derive_error;
use syn::{self, Data, DataEnum, DeriveInput};

// https://stackoverflow.com/a/65182902/97443
pub fn rid_ffi_message_impl(ast: DeriveInput) -> proc_macro2::TokenStream {
    let enum_ident = ast.ident;
    match ast.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let parsed_enum = ParsedEnum::new(enum_ident, variants);
            parsed_enum.derive_code()
        }
        _ => derive_error(&enum_ident, "Message can only be derived for enums"),
    }
}
