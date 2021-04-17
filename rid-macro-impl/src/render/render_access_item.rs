use super::{render_lifetime_def, render_rust_type, RenderedRustType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedFunction, ParsedReference,
};

pub fn render_access_item(
    rust_type: &RustType,
    fn_access_ident: &Ident,
    ffi_prelude: &TokenStream,
) -> TokenStream {
    use TypeKind as K;

    let arg_ident = format_ident!("arg");
    let RenderedRustType {
        tokens: return_type,
        ..
    } = render_rust_type(rust_type, false);

    let access_fn: Option<TokenStream> = match &rust_type.kind {
        K::Primitive(_) | K::Unit => None,
        K::Value(val) => todo!("render_free::Value"),
        K::Composite(Composite::Vec, inner_type) => match inner_type {
            Some(ty) => Some(render_vec_access_item(
                &rust_type,
                ty.as_ref(),
                fn_access_ident,
            )),
            None => {
                todo!("render_access_item: blow up since a composite should include inner type")
            }
        },
        K::Composite(_, _) => todo!("render_free::Composite"),
        K::Unknown => None,
    };

    match access_fn {
        Some(access_fn) => {
            quote! {
                #ffi_prelude
                #access_fn
            }
        }
        None => TokenStream::new(),
    }
}

fn render_vec_access_item(
    outer_type: &RustType,
    item_type: &RustType,
    fn_access_ident: &Ident,
) -> TokenStream {
    let RenderedRustType {
        tokens: vec_arg_type,
        lifetime,
    } = render_rust_type(outer_type, false);

    let RenderedRustType {
        tokens: item_return_type,
        ..
    } = render_rust_type(
        &item_type.clone().with_lifetime_option(lifetime.clone()),
        false,
    );

    let lifetime_def_tok = render_lifetime_def(lifetime.as_ref());

    quote_spanned! { fn_access_ident.span() =>
        fn #fn_access_ident#lifetime_def_tok(vec: #vec_arg_type, idx: usize) -> #item_return_type {
            vec[idx]
        }
    }
}
