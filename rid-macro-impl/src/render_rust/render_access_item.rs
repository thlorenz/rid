use super::{
    render_lifetime_def, render_return_type, RenderedReturnType, TypeAlias,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedFunction, ParsedReference,
};

pub struct RenderedAccessItem {
    pub tokens: TokenStream,
    pub type_alias: Option<TypeAlias>,
}

pub fn render_access_item(
    rust_type: &RustType,
    fn_access_ident: &Ident,
    ffi_prelude: &TokenStream,
) -> RenderedAccessItem {
    use TypeKind as K;

    let mut type_alias = None;

    let arg_ident = format_ident!("arg");
    let access_fn: Option<TokenStream> = match &rust_type.kind {
        K::Primitive(_) | K::Unit => None,
        // TODO: do we need special access code here?
        K::Value(val) => None,
        K::Composite(Composite::Vec, inner_type) => match inner_type {
            Some(ty) => {
                let (alias, tokens) = render_vec_access_item(
                    &rust_type,
                    ty.as_ref(),
                    fn_access_ident,
                );
                type_alias = alias;
                Some(tokens)
            }
            None => {
                todo!("render_access_item: blow up since a composite should include inner type")
            }
        },
        K::Composite(_, _) => todo!("render_free::Composite"),
        K::Unknown => None,
    };

    let tokens = match access_fn {
        Some(access_fn) => {
            quote! {
                #ffi_prelude
                #access_fn
            }
        }
        None => TokenStream::new(),
    };

    RenderedAccessItem { tokens, type_alias }
}

fn render_vec_access_item(
    outer_type: &RustType,
    item_type: &RustType,
    fn_access_ident: &Ident,
) -> (Option<TypeAlias>, TokenStream) {
    let RenderedReturnType {
        tokens: vec_arg_type,
        ..
    } = render_return_type(outer_type, false);

    let RenderedReturnType {
        tokens: item_return_type,
        type_alias,
        ..
    } = render_return_type(&item_type, false);

    let tokens = quote_spanned! { fn_access_ident.span() =>
        fn #fn_access_ident(vec: #vec_arg_type, idx: usize) -> #item_return_type {
            vec[idx]
        }
    };
    (type_alias, tokens)
}