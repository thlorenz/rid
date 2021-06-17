use syn::{
    punctuated::Punctuated, token::Comma, Field, Ident, ItemEnum, Variant,
};

use crate::{attrs::RidAttr, common::abort, parse_rid_attrs};

use super::{dart_type::DartType, rust_type::RustType};

// -----------------
// Enum
// -----------------
#[derive(Debug, PartialEq)]
pub struct ParsedEnum {
    /// The enum itself, i.e. Msg
    pub ident: syn::Ident,

    /// The enum variants, i.e. All, Completed
    pub variants: Vec<ParsedEnumVariant>,

    /// Parsed attributes of the enum
    pub attrs: Vec<RidAttr>,
}

impl ParsedEnum {
    pub fn from(enum_item: &ItemEnum) -> Self {
        let ItemEnum {
            ident,
            variants,
            attrs,
            ..
        } = enum_item;

        let attrs = parse_rid_attrs(&attrs);

        // NOTE: there is a parseable variant.discriminant Option that we can parse if we want to
        // support custom discriminants, i.e. `enum Stage { Started = 1 }`
        // For now we just assume it's the same as the slot of the variant.
        let variants = variants
            .into_iter()
            .enumerate()
            .map(|(idx, v)| ParsedEnumVariant::from(v, idx))
            .collect();

        Self {
            ident: ident.clone(),
            variants,
            attrs,
        }
    }
}

// -----------------
// Enum Variant
// -----------------
#[derive(Debug, PartialEq)]
pub struct ParsedEnumVariant {
    /// The name of the variant, i.e. All
    pub ident: Ident,

    /// The fields of the variant, empty for c-like enums
    pub fields: Vec<ParsedEnumVariantField>,

    /// The variant discriminator starting with 0
    pub discriminant: usize,
}

impl ParsedEnumVariant {
    pub fn from(variant: &Variant, discriminant: usize) -> Self {
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(idx, f)| ParsedEnumVariantField::from(f, idx))
            .collect();
        Self {
            ident: variant.ident.clone(),
            fields,
            discriminant,
        }
    }
}

// -----------------
// Enum Variant Field
// -----------------
#[derive(Debug, PartialEq)]
pub struct ParsedEnumVariantField {
    /// The Rust type of the field
    pub rust_type: RustType,

    /// The Dart type of the field
    pub dart_type: DartType,

    /// The slot (starting with 0) of the field
    pub slot: usize,
}

impl ParsedEnumVariantField {
    // NOTE: unlike the `message` enum (src/message/variant_field.rs) here we don't support custom
    // types yet
    fn from(f: &Field, slot: usize) -> Self {
        let rust_type = RustType::from_plain_type(&f.ty);
        let rust_type = match rust_type {
            Some(x) => x,
            None => abort!(f.ident, "invalid rust type"),
        };
        let dart_type = DartType::from(&rust_type);
        Self {
            rust_type,
            dart_type,
            slot,
        }
    }
}
