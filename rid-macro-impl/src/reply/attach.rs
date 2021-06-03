use proc_macro2::TokenStream;
use syn::{Item, NestedMeta};

use crate::{
    attrs::{self, parse_rid_args, EnumConfig},
    common::{abort, callsite_error},
};

use super::{
    render_reply_into_dart::render_reply_into_dart, reply_variant::ReplyVariant,
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
            render_reply_into_dart(&item.ident, &reply_variants)
        }
        Item::Const(_)
        | Item::ExternCrate(_)
        | Item::Fn(_)
        | Item::ForeignMod(_)
        | Item::Impl(_)
        | Item::Macro(_)
        | Item::Macro2(_)
        | Item::Mod(_)
        | Item::Static(_)
        | Item::Struct(_)
        | Item::Trait(_)
        | Item::TraitAlias(_)
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | Item::__TestExhaustive(_) => {
            abort!(item, "rid::message attribute can only be applied to enums");
        }
    }
}
