use crate::{
    attrs::{parse_derive_attrs, EnumConfig},
    common::extract_variant_names,
    model::debug::render_debug,
    parse::{rust_type::RustType, ParsedEnum},
};
use proc_macro2::TokenStream;

use quote::quote_spanned;
use syn::ItemEnum;

pub fn render_enum(enum_item: &ItemEnum) -> TokenStream {
    let derive = parse_derive_attrs(&enum_item.attrs);
    let enum_config = EnumConfig::from(enum_item);
    let parsed_enum = ParsedEnum::from(enum_item, enum_config);

    // -----------------
    // Dart Enum
    // -----------------
    let dart_enum: TokenStream =
        parsed_enum.render_dart("///").parse().unwrap();

    // -----------------
    // derive(Debug)
    // -----------------
    let derive_debug_tokens = if derive.debug {
        let rust_type = RustType::from_owned_enum(&parsed_enum.ident);
        let variants = extract_variant_names(&enum_item.variants);
        render_debug(rust_type, &Some(variants), Default::default())
    } else {
        TokenStream::new()
    };

    // -----------------
    // rid::export
    // -----------------
    let export_enum_tokens: TokenStream = format!(
        "#[no_mangle] pub extern \"C\" fn _export_dart_enum_{} () {{}}",
        enum_item.ident
    )
    .parse()
    .unwrap();

    // -----------------
    // Resolve enum to int and vice versa
    // -----------------
    let resolution_impl = parsed_enum.render_enum_resolution_impl();

    // -----------------
    // Combine all the above
    // -----------------
    quote_spanned! { enum_item.ident.span() =>
        #dart_enum
        #export_enum_tokens
        #resolution_impl
        #derive_debug_tokens
    }
}
