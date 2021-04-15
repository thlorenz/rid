use crate::{
    attrs, attrs::StructConfig, common::abort,
    model::parsed_struct::ParsedStruct,
};
use proc_macro2::TokenStream;
use syn::{Fields, Item};

pub fn rid_ffi_model_impl(item: &Item) -> TokenStream {
    match item {
        Item::Struct(item) => {
            let rid_attrs = attrs::parse_rid_attrs(&item.attrs);
            let struct_config = StructConfig::new(&rid_attrs);
            match &item.fields {
                Fields::Named(fields) => {
                    let parsed_struct = ParsedStruct::new(
                        item.ident.clone(),
                        &fields.named,
                        struct_config,
                    );
                    parsed_struct.tokens()
                }
                Fields::Unnamed(_) => abort!(
                    item.ident,
                    "not yet supporting structs with unnamed fields"
                ),
                Fields::Unit => abort!(
                    item.ident,
                    "structs without fields cannot be a rid::model"
                ),
            }
        }
        Item::Const(_)
        | Item::Enum(_)
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
            abort!(item, "rid::model attribute can only be applied to structs");
        }
    }
}
