use super::{render_lifetime_def, render_return_type, RenderedReturnType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

use crate::{
    accesses::{collection_access_tokens, AccessKind, CollectionAccessTokens},
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedFunction, ParsedReference,
    },
    render_common::PointerTypeAlias,
};

pub struct RenderedAccessItem {
    pub tokens: TokenStream,
    pub type_alias: Option<PointerTypeAlias>,
}

pub fn render_access_item(
    rust_type: &RustType,
    fn_access_ident: &Ident,
    ffi_prelude: &TokenStream,
    access_kind: &AccessKind,
) -> RenderedAccessItem {
    use TypeKind as K;

    let mut type_alias = None;

    let arg_ident = format_ident!("arg");
    let access_fn: Option<TokenStream> = match &rust_type.kind {
        // -----------------
        // Primitives
        // -----------------
        K::Primitive(_) | K::Unit => None,
        // TODO: do we need special access code here?
        K::Value(val) => None,

        // -----------------
        // Vec
        // -----------------
        K::Composite(Composite::Vec, inner_type, _) => match inner_type {
            Some(ty) => {
                let (alias, tokens) = render_vec_access_item(
                    &rust_type,
                    ty.as_ref(),
                    fn_access_ident,
                    access_kind,
                );
                type_alias = alias;
                Some(tokens)
            }
            None => {
                todo!("render_access_item: blow up since a composite should include inner type")
            }
        },

        // TODO(thlorenz): HashMap
        // -----------------
        // HashMap
        // -----------------
        K::Composite(Composite::HashMap, key_type, val_type) => todo!(
            "render_access_item::Composite::HashMap<{:?}, {:?}>",
            key_type,
            val_type
        ),
        K::Composite(_, _, _) => todo!("render_access_item::Composite"),
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
    access_kind: &AccessKind,
) -> (Option<PointerTypeAlias>, TokenStream) {
    let RenderedReturnType {
        tokens: vec_arg_type,
        ..
    } = render_return_type(outer_type, access_kind);

    let CollectionAccessTokens {
        item_return_type,
        into_return_type,
        type_alias,
    } = collection_access_tokens(format_ident!("ptr"), item_type, access_kind);

    let tokens = quote_spanned! { fn_access_ident.span() =>
        fn #fn_access_ident(vec: #vec_arg_type, idx: usize) -> #item_return_type {
            let ptr = vec[idx];
            #into_return_type

        }
    };
    (type_alias, tokens)
}
