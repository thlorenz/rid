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
            if a.is_string_like(){
                quote_spanned! { ident.span() =>
                    #ident(id, d) => {
                        let rid_vec: Vec<#typ_ident> = d.into();
                        let mut byte_vec: Vec<u8> = vec![];
                        for v in rid_vec{
                            let mut str_bytes = v.as_bytes().to_vec();
                            str_bytes.push(0);
                            byte_vec.append(&mut str_bytes);
                        }
                        (rid::_encode_with_id(#slot, id), byte_vec)
                    },
                }
            }else if a.is_primitive(){
                quote_spanned! { ident.span() =>
                    #ident(id, d) => {
                        let rid_vec: Vec<#typ_ident> = d.into();
                        let mut byte_vec: Vec<u8> = vec![];
                        for v in rid_vec{
                            byte_vec.extend_from_slice(&v.to_be_bytes())
                        }
                        (rid::_encode_with_id(#slot, id), byte_vec)
                    },
                }
            }else{
                unimplemented!("Type {typ_ident} isn't supported yet!");
            }
        }
        // Vector without id
        (false, Some(TypeKind::Composite(_, Some(a), None))) => {
            let typ_ident = a.rust_ident();
            if a.is_string_like(){
                quote_spanned! { ident.span() =>
                    #ident(d) => {
                        let rid_vec: Vec<#typ_ident> = d.into();
                        let mut byte_vec: Vec<u8> = vec![];
                        for v in rid_vec{
                            let mut str_bytes = v.as_bytes().to_vec();
                            str_bytes.push(0);
                            byte_vec.append(&mut str_bytes);
                        }
                        (rid::_encode_without_id(#slot), byte_vec)
                    },
                }
            }else if a.is_primitive(){
                quote_spanned! { ident.span() =>
                    #ident(d) => {
                        let rid_vec: Vec<#typ_ident> = d.into();
                        let mut byte_vec: Vec<u8> = vec![];
                        for v in rid_vec{
                            byte_vec.extend_from_slice(&v.to_be_bytes())
                        }
                        (rid::_encode_without_id(#slot), byte_vec)
                    },
                }
            }else{
                unimplemented!("Type {typ_ident} isn't supported yet!");
            }
        }
        // String with id
        (true, Some(TypeKind::Value(Value::String))) => {
            quote_spanned! { ident.span() =>
                #ident(id, d) => {
                    let mut v = d.as_bytes().to_vec();
                    v.push(0);
                    (rid::_encode_with_id(#slot, id), v)
                },
            }
        }
        // String without id
        (false, Some(TypeKind::Value(Value::String))) => {
            quote_spanned! { ident.span() =>
                #ident(d) => {
                    let mut v = d.as_bytes().to_vec();
                    v.push(0);
                    (rid::_encode_without_id(#slot), v)
                },
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
