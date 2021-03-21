use super::parsed_enum::ParsedEnum;
use crate::{
    attrs::{parse_rid_attrs, EnumConfig},
    common::callsite_error,
};

// https://stackoverflow.com/a/65182902/97443
pub fn rid_ffi_message_impl(
    input: syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let enum_ident = input.ident;
    let rid_attrs = parse_rid_attrs(&input.attrs);
    let enum_config = EnumConfig::new(&enum_ident, &rid_attrs);

    match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let parsed_enum =
                ParsedEnum::new(enum_ident, variants.clone(), enum_config);
            parsed_enum.tokens()
        }
        _ => callsite_error("message can only be attached to enums"),
    }
}
