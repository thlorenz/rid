use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Item, NestedMeta};

use crate::{
    attrs::{self, parse_rid_args, EnumConfig},
    common::{abort, callsite_error, utils_module_tokens},
    parse::ParsedEnum,
};

use super::{
    render_reply_dart::render_reply_dart,
    render_reply_into_dart::render_reply_into_dart,
    reply_variant::ReplyVariant,
};

pub fn rid_ffi_reply_impl(item: &Item, _: &[NestedMeta]) -> TokenStream {
    match item {
        Item::Enum(enum_item) => {
            let reply_variants: Vec<ReplyVariant> = enum_item
                .variants
                .iter()
                .enumerate()
                .map(|(slot, x)| ReplyVariant::new(slot, x))
                .collect();
            let into_dart =
                render_reply_into_dart(&enum_item.ident, &reply_variants);
            let enum_config = EnumConfig::from(&enum_item);
            let parsed_enum = ParsedEnum::from(&enum_item, enum_config);
            let reply_dart = render_reply_dart(&parsed_enum, "///");

            let utils_module = utils_module_tokens();
            quote_spanned! { enum_item.ident.span() =>
                mod __rid_reply_mod {
                    #reply_dart
                    #[no_mangle]
                    pub extern "C" fn include_reply() {}
                }
                #into_dart
                #utils_module
            }
        }
        _ => {
            abort!(item, "rid::reply attribute can only be applied to enums");
        }
    }
}
