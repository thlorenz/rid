use proc_macro2::TokenStream;
use syn::{Item, NestedMeta};

use super::{
    parsed_message_enum::ParsedMessageEnum,
    render_message_enum::MessageRenderConfig, MessageEnumConfig,
};
use crate::{
    attrs::{self, parse_rid_args},
    common::{abort, callsite_error},
};

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
            if rid_args.len() == 2 {
                let enum_config = MessageEnumConfig::new(
                    &rid_attrs,
                    &rid_args[0],
                    &rid_args[1],
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
                Please specify exactly one store struct which this message\n\
                updates and a reply enum with which it responds.\n\
                Example: #[rid::message(Store, Reply)]"
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
