use super::{render_lifetime_def, render_rust_type, RenderedRustType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::Ident;

use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedFunction,
};

pub fn render_free(
    rust_type: &RustType,
    fn_free_ident: &Ident,
    ffi_prelude: &TokenStream,
) -> TokenStream {
    use TypeKind as K;

    let arg_ident = format_ident!("arg");
    let RenderedRustType {
        tokens: return_type,
        lifetime,
    } = render_rust_type(rust_type, false);
    let lifetime_def_tok = render_lifetime_def(lifetime.as_ref());

    let free: Option<TokenStream> = match &rust_type.kind {
        K::Primitive(_) | K::Unit => None,
        K::Value(val) => todo!("render_free::Value"),
        K::Composite(Composite::Vec, rust_type) => {
            Some(quote_spanned! { arg_ident.span() =>  #arg_ident.free(); })
        }
        K::Composite(_, _) => todo!("render_free::Composite"),
        K::Unknown => None,
    };

    match free {
        Some(free_statement) => {
            quote_spanned! {fn_free_ident.span() =>
                #ffi_prelude
                fn #fn_free_ident#lifetime_def_tok(#arg_ident: #return_type) {
                    #free_statement
                }
            }
        }
        None => TokenStream::new(),
    }
}
