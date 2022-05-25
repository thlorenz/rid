use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};

use crate::parse::rust_type::{TypeKind, Value};

use super::reply_variant::ReplyVariant;

pub fn render_reply_into_dart(
    enum_ident: &syn::Ident,
    variants: &[ReplyVariant],
) -> TokenStream {
    let variant_tokens = variants.iter().map(render_variant);

    quote_spanned! { enum_ident.span() =>
        impl rid::_allo_isolate::IntoDart for #enum_ident {
            fn into_dart(self) -> rid::_allo_isolate::ffi::DartCObject {
                use #enum_ident::*;
                let (base, mut data): (i64, Vec<u8>) = match self {
                    #(#variant_tokens)*
                };
                let mut ret_data = base.to_be_bytes().to_vec();
                ret_data.append(&mut data);
                ret_data.into_dart()
            }
        }
    }
}

fn render_variant(variant: &ReplyVariant) -> TokenStream {
    let ident = &variant.ident;
    let slot = variant.slot as i64;
    match (variant.has_req_id, variant.has_data.as_ref()) {
        // Vector with id
        (true, Some(TypeKind::Composite(_, Some(a), None))) => {
            let typ_ident = a.rust_ident();
            quote_spanned! { ident.span() =>
                #ident(id, d) => {
                    let rid_vec: Vec<#typ_ident> = d.into();
                    (rid::_encode_with_id(#slot, id), rid_vec)
                },
            }
        }
        // Vector without id
        (false, Some(TypeKind::Composite(_, Some(a), None))) => {
            let typ_ident = a.rust_ident();
            quote_spanned! { ident.span() =>
                #ident(d) => {
                    let rid_vec: Vec<#typ_ident> = d.into();
                    (rid::_encode_without_id(#slot), rid_vec)
                },
            }
        }
        // String with id
        (true, Some(TypeKind::Value(Value::String))) => {
            quote_spanned! { ident.span() =>
                #ident(id, d) => (rid::_encode_with_id(#slot, id), d.bytes()),
            }
        }
        // String without id
        (false, Some(TypeKind::Value(Value::String))) => {
            quote_spanned! { ident.span() =>
                #ident(d) => (rid::_encode_without_id(#slot), d.bytes()),
            }
        }
        // Primitive data with ID
        (true, Some(TypeKind::Primitive(a))) => quote_spanned! {ident.span() =>
            #ident(id, d) => (rid::_encode_with_id(#slot, id), d.into()),
        },
        // Primitive data without ID
        (false, Some(TypeKind::Primitive(a))) => {
            quote_spanned! {ident.span() =>
                #ident(d) => (rid::_encode_without_id(#slot), d.into()),
            }
        }
        // No data with ID
        (true, None) => quote_spanned! {ident.span() =>
            #ident(id) => (rid::_encode_with_id(#slot, id), vec![]),
        },
        // No data without ID
        (false, None) => quote_spanned! {ident.span() =>
            #ident => (rid::_encode_without_id(#slot), vec![]),
        },
        _ => quote_spanned! {ident.span() =>
            #ident => (rid::_encode_without_id(#slot), vec![]),
        },
    }
}
