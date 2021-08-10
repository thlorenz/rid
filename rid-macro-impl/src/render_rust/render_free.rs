use super::{render_lifetime_def, render_return_type, RenderedReturnType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::Ident;

use crate::{
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedFunction,
    },
    render_common::{AccessKind, PointerTypeAlias},
};

pub struct RenderedFree {
    pub tokens: TokenStream,
    pub type_alias: Option<PointerTypeAlias>,
}

pub fn render_free(
    rust_type: &RustType,
    fn_free_ident: &Ident,
    ffi_prelude: &TokenStream,
    access_kind: &AccessKind,
) -> RenderedFree {
    use TypeKind as K;

    let arg_ident = format_ident!("arg");
    let RenderedReturnType {
        tokens: return_type,
        type_alias,
        ..
    } = render_return_type(rust_type, access_kind);

    let free: Option<TokenStream> = match &rust_type.kind {
        K::Primitive(_) | K::Unit => None,
        // TODO: in general we shouldn't free refs, but only owned values since the refs
        // are most likely to a model property which is still alive
        K::Value(val) => None,
        K::Composite(Composite::Vec, _, _) => {
            Some(quote_spanned! { arg_ident.span() =>  #arg_ident.free(); })
        }
        K::Composite(Composite::HashMap, _, _) => {
            Some(quote_spanned! { arg_ident.span() =>  #arg_ident.free(); })
        }
        K::Composite(composite, _, _) => {
            todo!("render_free::Composite::{:?}", composite)
        }
        K::Unknown => None,
    };

    match free {
        Some(free_statement) => {
            let tokens = quote_spanned! {fn_free_ident.span() =>
                #ffi_prelude
                fn #fn_free_ident(#arg_ident: #return_type) {
                    #free_statement
                }
            };
            RenderedFree { type_alias, tokens }
        }
        None => RenderedFree {
            type_alias: None,
            tokens: TokenStream::new(),
        },
    }
}
