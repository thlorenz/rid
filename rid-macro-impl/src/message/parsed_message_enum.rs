use crate::{
    attrs::{raw_typedef_ident, EnumConfig, TypeInfoMap},
    parse::rust_type,
};

use super::{parsed_variant::ParsedMessageVariant, store::code_store_module};
use quote::{format_ident, quote, quote_spanned, IdentFragment};
use std::collections::HashMap;
use syn::{punctuated::Punctuated, token::Comma, Ident, Variant};

pub struct ParsedMessageEnum {
    /// The enum itself, i.e. Msg
    pub ident: syn::Ident,

    /// The enum variants, i.e. AddTodo(String)
    pub parsed_variants: Vec<ParsedMessageVariant>,

    /// Prefix used for all message methods, i.e. rid_msg
    pub method_prefix: String,

    /// The identifier of the struct receiving the message, i.e. Store
    pub struct_ident: syn::Ident,

    /// The raw pointer identifier of the struct receiving the message, i.e. RawStore
    pub raw_struct_ident: syn::Ident,

    /// Identifier of the module into which the hidden code is wrapped
    pub module_ident: syn::Ident,

    /// The name of the enum used to reply to messages
    pub reply_dart_enum_name: String,

    pub ident_lower_camel: String,
    pub config: EnumConfig,
}

impl ParsedMessageEnum {
    pub fn new(
        ident: &Ident,
        variants: Punctuated<Variant, Comma>,
        config: EnumConfig,
    ) -> Self {
        let ident_str = ident.to_string();
        let ident_lower_camel = lower_camel_case(&ident_str);
        let ident_lower = ident_str.to_lowercase();
        let method_prefix = format!("rid_{}", ident_lower);
        let module_ident = format_ident!("__rid_{}_ffi", ident_lower);

        let parsed_variants =
            parse_variants(variants, &method_prefix, &config.type_infos);
        let struct_ident = format_ident!("{}", config.to);
        let raw_struct_ident = raw_typedef_ident(&struct_ident);
        let reply_ident = rust_type::RustType::from_owned_enum(&config.reply);
        let reply_dart_enum_name = reply_ident.dart_ident(false).to_string();

        Self {
            ident: ident.clone(),
            reply_dart_enum_name,
            parsed_variants,
            method_prefix,
            struct_ident,
            raw_struct_ident,
            module_ident,
            ident_lower_camel,
            config,
        }
    }
}

fn parse_variants(
    variants: Punctuated<Variant, Comma>,
    method_prefix: &str,
    types: &TypeInfoMap,
) -> Vec<ParsedMessageVariant> {
    variants
        .into_iter()
        .map(|v| ParsedMessageVariant::new(v, &method_prefix, types))
        .collect()
}

fn lower_camel_case(s: &str) -> String {
    format!("{}{}", s[0..1].to_lowercase(), s[1..].to_string())
}
