use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{Item, NestedMeta};

use super::{
    parsed_message_enum::ParsedMessageEnum,
    render_message_enum::MessageRenderConfig, MessageEnumConfig,
};
use crate::{
    attrs::{self, parse_rid_args},
    common::{abort, callsite_error},
};
use rid_common::STORE;

// https://stackoverflow.com/a/65182902/97443
pub fn rid_message_impl(
    item: &Item,
    args: &[NestedMeta],
    render_config: MessageRenderConfig,
) -> TokenStream {
    match item {
        Item::Enum(item) => {
            let rid_attrs = attrs::parse_rid_attrs(&item.attrs);
            let rid_args = parse_rid_args(args);
            if rid_args.len() == 1 {
                // NOTE: hardcode the store ident here instead of removing it everywhere in case we
                // ever want to not rely on it being name 'Store' for the message implementation.
                let enum_config = MessageEnumConfig::new(
                    &rid_attrs,
                    format_ident!("{}", STORE),
                    &rid_args[0],
                );
                let parsed_message_enum = ParsedMessageEnum::new(
                    &item.ident,
                    item.variants.clone(),
                    enum_config,
                );
                parsed_message_enum.render(&render_config).0
            } else {
                abort!(
                    item,
                    "\
                Please specify exactly one reply type which is used\nto respond to messages.\n\
                Example: #[rid::message(Reply)]"
                )
            }
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
