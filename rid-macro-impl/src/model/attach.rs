use crate::{
    attrs, attrs::StructConfig, common::abort,
    model::parsed_struct::ParsedStruct, parse::ParsedEnum,
};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Fields, Item};

pub fn rid_ffi_model_impl(item: &Item) -> TokenStream {
    match item {
        Item::Struct(struct_item) => {
            let rid_attrs = attrs::parse_rid_attrs(&struct_item.attrs);
            let struct_config = StructConfig::new(&rid_attrs);
            let exports = match &struct_item.fields {
                Fields::Named(fields) => {
                    let parsed_struct = ParsedStruct::new(
                        struct_item.ident.clone(),
                        &fields.named,
                        struct_config,
                    );
                    parsed_struct.tokens()
                }
                Fields::Unnamed(_) => abort!(
                    struct_item.ident,
                    "not yet supporting structs with unnamed fields"
                ),
                Fields::Unit => abort!(
                    struct_item.ident,
                    "structs without fields cannot be a rid::model"
                ),
            };
            quote_spanned! { struct_item.ident.span() =>
                #item
                #exports
            }
        }
        Item::Enum(enum_item) => {
            let parsed_enum = ParsedEnum::from(enum_item);
            let resolution_impl = parsed_enum.render_enum_resolution_impl();
            quote_spanned! { enum_item.ident.span() =>
                #[repr(C)]
                #item
                #resolution_impl
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
        | Item::Trait(_)
        | Item::TraitAlias(_)
        | Item::Type(_)
        | Item::Union(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | Item::__TestExhaustive(_) => {
            abort!(item, "rid::model attribute can only be applied to structs and c-style enums");
        }
    }
}
