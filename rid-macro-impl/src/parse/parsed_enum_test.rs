use assert_matches::assert_matches;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Item};

use crate::attrs::EnumConfig;

use super::{ParsedEnum, ParsedEnumVariant, ParsedEnumVariantField};

fn parse(input: TokenStream) -> ParsedEnum {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        Item::Enum(item) => ParsedEnum::from(&item, EnumConfig::from(&item)),
        _ => panic!("Unexpected item, we're trying to parse enums here"),
    }
}

mod c_style_enums {
    use super::*;

    #[test]
    fn c_style_enum_one_variant() {
        let ident_one: Ident = format_ident!("One");
        let ParsedEnum {
            ident,
            variants,
            config: _,
        } = parse(quote! {
            enum Count { One }
        });
        assert_eq!(ident.to_string(), "Count");
        let variant0 = &variants[0];
        assert_eq!(
            variant0,
            &ParsedEnumVariant {
                ident: ident_one,
                fields: vec![],
                discriminant: 0
            }
        );
    }

    #[test]
    fn c_style_enum_two_variants() {
        let ident_one: Ident = format_ident!("One");
        let ident_two: Ident = format_ident!("Two");
        let ParsedEnum {
            ident, variants, ..
        } = parse(quote! {
            enum Count { One, Two }
        });
        assert_eq!(ident.to_string(), "Count");
        let variant0 = &variants[0];
        let variant1 = &variants[1];
        assert_eq!(
            variant0,
            &ParsedEnumVariant {
                ident: ident_one,
                fields: vec![],
                discriminant: 0
            }
        );
        assert_eq!(
            variant1,
            &ParsedEnumVariant {
                ident: ident_two,
                fields: vec![],
                discriminant: 1
            }
        );
    }
}

mod field_enums {
    use super::*;
    use crate::parse::rust_type::{Primitive, TypeKind, Value};

    #[test]
    fn enum_one_variant_with_u8_field() {
        let ident_one: Ident = format_ident!("One");
        let ParsedEnum {
            ident,
            variants,
            config: _,
        } = parse(quote! {
            enum Count { One(u8) }
        });
        assert_eq!(ident.to_string(), "Count");
        let ParsedEnumVariant {
            ident,
            fields,
            discriminant,
        } = &variants[0];

        assert_eq!(ident, &ident_one, "variant ident");
        assert_eq!(*discriminant, 0, "variant discriminant");
        assert_eq!(
            fields[0].rust_type.kind,
            TypeKind::Primitive(Primitive::U8),
            "variant first field"
        );
    }

    #[test]
    fn enum_one_variant_with_string_field() {
        let ident_one: Ident = format_ident!("One");
        let fields: Vec<ParsedEnumVariantField> = vec![];
        let ParsedEnum {
            ident,
            variants,
            config: _,
        } = parse(quote! {
            enum Count { One(String) }
        });
        assert_eq!(ident.to_string(), "Count");
        let ParsedEnumVariant {
            ident,
            fields,
            discriminant,
        } = &variants[0];

        assert_eq!(ident, &ident_one, "variant ident");
        assert_eq!(*discriminant, 0, "variant discriminant");
        assert_eq!(
            fields[0].rust_type.kind,
            TypeKind::Value(Value::String),
            "variant first field"
        );
    }
}
