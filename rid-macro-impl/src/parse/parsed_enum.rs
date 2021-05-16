use syn::{
    punctuated::Punctuated, token::Comma, Field, Ident, ItemEnum, Variant,
};

use crate::{
    attrs::{EnumConfig, RidAttr, TypeInfoMap},
    common::abort,
    parse_rid_attrs,
};

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

    /// The config of the enum including information from other attributes found on it
    pub config: EnumConfig,
}

impl ParsedEnum {
    pub fn from(enum_item: &ItemEnum, config: EnumConfig) -> Self {
        let ItemEnum {
            ident,
            variants,
            attrs,
            ..
        } = enum_item;

        // NOTE: there is a parseable variant.discriminant Option that we can parse if we want to
        // support custom discriminants, i.e. `enum Stage { Started = 1 }`
        // For now we just assume it's the same as the slot of the variant.
        let variants = variants
            .into_iter()
            .enumerate()
            .map(|(idx, v)| ParsedEnumVariant::from(v, idx, &config.type_infos))
            .collect();

        Self {
            ident: ident.clone(),
            variants,
            config,
        }
    }

    /// Parsed attributes of the enum
    pub fn attrs(&self) -> &[RidAttr] {
        &self.config.attrs
    }

    /// Information about custom types used inside this enum
    pub fn type_infos(&self) -> &TypeInfoMap {
        &self.config.type_infos
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
    pub fn from(
        variant: &Variant,
        discriminant: usize,
        type_infos: &TypeInfoMap,
    ) -> Self {
        let fields = variant
            .fields
            .iter()
            .enumerate()
            .map(|(idx, f)| ParsedEnumVariantField::from(f, idx, type_infos))
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
    fn from(f: &Field, slot: usize, type_infos: &TypeInfoMap) -> Self {
        let rust_type = RustType::from_plain_type(&f.ty);
        let rust_type = match rust_type {
            Some(x) => x,
            None => abort!(f.ident, "invalid rust type"),
        };
        let dart_type = DartType::from(&rust_type, type_infos);
        Self {
            rust_type,
            dart_type,
            slot,
        }
    }
}
