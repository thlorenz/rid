use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DataEnum, DeriveInput, Token, Variant,
};

use crate::{
    common::{
        abort,
        resolvers::cstring_free,
        state::{get_state, ImplementationType},
    },
    parse::rust_type::RustType,
    render_rust::RenderedDisplayImpl,
};

pub struct DisplayImplConfig {
    render_cstring_free: bool,
}

impl Default for DisplayImplConfig {
    fn default() -> Self {
        Self {
            render_cstring_free: true,
        }
    }
}

impl DisplayImplConfig {
    pub fn for_tests() -> Self {
        Self {
            render_cstring_free: false,
        }
    }
}

pub fn rid_display_impl(
    input: &DeriveInput,
    config: DisplayImplConfig,
) -> TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let rust_type = RustType::from_owned_struct(&input.ident);
            render_display(rust_type, &config, &None)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let rust_type = RustType::from_owned_enum(&input.ident);
            let variants = Some(extract_variant_names(variants));
            render_display(rust_type, &config, &variants)
        }
        Data::Union(data) => abort!(
            input.ident,
            "Cannot derive display for an untagged Union type"
        ),
    }
}

fn render_display(
    rust_type: RustType,
    config: &DisplayImplConfig,
    enum_variants: &Option<Vec<String>>,
) -> TokenStream {
    let cstring_free_tokens = if config.render_cstring_free {
        cstring_free()
    } else {
        TokenStream::new()
    };

    let RenderedDisplayImpl {
        tokens: rust_method_tokens,
        fn_display_method_name,
    } = rust_type.render_display_impl(enum_variants);

    let dart_ext_tokens: TokenStream = rust_type
        .render_dart_display_extension(&fn_display_method_name, "///")
        .parse()
        .unwrap();

    // TODO: once model/parsed_struct.rs is normalized to parse::RustType we need to render
    // this enum there as well once we see a field of its type.
    let dart_enum_tokens: TokenStream = if rust_type.is_enum()
        && get_state().needs_implementation(
            &ImplementationType::DartEnum,
            &rust_type.ident.to_string(),
        ) {
        rust_type
            .render_dart_enum(
                enum_variants
                    .as_ref()
                    .expect("Need variants to render enum"),
                "///",
            )
            .parse()
            .unwrap()
    } else {
        TokenStream::new()
    };

    quote! {
        #dart_enum_tokens
        #dart_ext_tokens
        #rust_method_tokens
        #cstring_free_tokens
    }
}

fn extract_variant_names(
    variants: &Punctuated<Variant, Token![,]>,
) -> Vec<String> {
    variants.into_iter().map(|x| x.ident.to_string()).collect()
}
