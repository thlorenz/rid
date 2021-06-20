use super::to_dart::render_to_dart;
use crate::{
    attrs,
    attrs::{parse_rid_args, EnumConfig, StructConfig},
    common::abort,
    model::{parsed_struct::ParsedStruct, store::render_store_module},
    parse::{self, ParsedEnum},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::{Fields, Item};

pub fn rid_ffi_model_impl(item: &Item, is_store: bool) -> TokenStream {
    match item {
        Item::Struct(struct_item) => {
            let rid_attrs = attrs::parse_rid_attrs(&struct_item.attrs);
            let struct_config = StructConfig::from(&struct_item);
            let parsed_struct = parse::ParsedStruct::new(
                &struct_item,
                &struct_item.ident,
                struct_config.clone(),
            );
            let dart_class_tokens =
                render_to_dart(&parsed_struct, is_store, Default::default());

            let store_module = if is_store {
                render_store_module(&struct_item.ident)
            } else {
                TokenStream::new()
            };

            // TODO(thlorenz): This needs to go in its own file
            let store_wrapper_tokens = if is_store {
                let comment = "///";
                let field_wrappers =
                    parsed_struct.render_field_wrappers(comment);
                let field_wrapper_tokens: TokenStream = format!(
                    r###"
{comment} ```dart
{comment} /// Wrappers to access fields with the higher level API which is memory safe.
{comment} extension FieldAccessWrappersOn_{Store} on {Store} {{
{field_wrappers}
{comment} }}
{comment} ```"###,
                    Store = parsed_struct.ident,
                    field_wrappers = field_wrappers.join("\n"),
                    comment = comment,
                )
                .parse()
                .unwrap();

                let mod_ident =
                    format_ident!("{}_field_wrappers", parsed_struct.ident);
                let fn_ident = format_ident!(
                    "_include_{}_field_wrappers",
                    parsed_struct.ident
                );
                quote_spanned! { struct_item.ident.span() =>
                    #[allow(non_snake_case)]
                    mod #mod_ident {
                        #field_wrapper_tokens
                        #[no_mangle]
                        pub extern "C" fn #fn_ident() {}
                    }
                }
            } else {
                TokenStream::new()
            };

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
                #store_module
                #exports
                #dart_class_tokens
                #store_wrapper_tokens
            }
        }
        Item::Enum(enum_item) => {
            let enum_config = EnumConfig::from(enum_item);
            let parsed_enum = ParsedEnum::from(enum_item, enum_config);
            let dart_enum: TokenStream =
                parsed_enum.render_dart("///").parse().unwrap();

            let export_enum_tokens: TokenStream = format!(
                "#[no_mangle] pub extern \"C\" fn _export_dart_enum_{} () {{}}",
                enum_item.ident
            )
            .parse()
            .unwrap();

            let resolution_impl = parsed_enum.render_enum_resolution_impl();
            quote_spanned! { enum_item.ident.span() =>
                #[repr(C)]
                #item
                #dart_enum
                #export_enum_tokens
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
