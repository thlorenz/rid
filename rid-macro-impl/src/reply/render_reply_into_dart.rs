use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};

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
                let (base, data): (i64, String) = match self {
                    #(#variant_tokens)*
                };
                format!("{}{}{}", base, "^", data).into_dart()
            }
        }
    }
}

fn render_variant(variant: &ReplyVariant) -> TokenStream {
    let ident = &variant.ident;
    let slot = variant.slot as i64;
    match (variant.has_req_id, variant.has_data) {
        (true, true) => quote_spanned! {ident.span() =>
            #ident(id, s) => (rid::_encode_with_id(#slot, id), s.into()),
        },
        (true, false) => quote_spanned! {ident.span() =>
            #ident(id) => (rid::_encode_with_id(#slot, id), "".into()),
        },
        _ => quote_spanned! {ident.span() =>
            #ident => (rid::_encode_without_id(#slot), "".into()),
        },
    }
}
