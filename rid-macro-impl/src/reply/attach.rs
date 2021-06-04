use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Item, NestedMeta};

use crate::{
    attrs::{self, parse_rid_args, EnumConfig},
    common::{abort, callsite_error},
};

use super::{
    render_reply_dart::render_reply_dart,
    render_reply_into_dart::render_reply_into_dart,
    reply_variant::ReplyVariant,
};

pub fn rid_ffi_reply_impl(item: &Item, _: &[NestedMeta]) -> TokenStream {
    match item {
        Item::Enum(item) => {
            let reply_variants: Vec<ReplyVariant> = item
                .variants
                .iter()
                .enumerate()
                .map(|(slot, x)| ReplyVariant::new(slot, x))
                .collect();

            let into_dart =
                render_reply_into_dart(&item.ident, &reply_variants);
            let reply_dart =
                render_reply_dart(&item.ident, &item.variants, "///");

            quote_spanned! { item.ident.span() =>
                mod __rid_reply_mod {
                    #reply_dart
                    #[no_mangle]
                    pub extern "C" fn include_reply() {}
                }
                #into_dart
            }
        }
        _ => {
            abort!(item, "rid::reply attribute can only be applied to enums");
        }
    }
}
