use crate::{attrs::EnumConfig, parse::ParsedEnum};
use proc_macro2::TokenStream;

use quote::quote_spanned;
use syn::ItemEnum;

pub fn render_enum(enum_item: &ItemEnum) -> TokenStream {
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
        #dart_enum
        #export_enum_tokens
        #resolution_impl
    }
}
