use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

use crate::{
    common::tokens::resolve_string_ptr,
    parse::rust_type::{RustType, TypeKind},
    render_rust::render_rust_type,
};

pub struct RustArg {
    pub arg_ident: Ident,
    pub type_tokens: TokenStream,
    pub resolver_tokens: TokenStream,
}

impl RustArg {
    pub fn from(ty: &RustType, slot: usize) -> Self {
        use TypeKind::*;
        match &ty.kind {
            Primitive(p) => {
                let arg_ident = format_ident!("arg{}", slot);
                let ty = p.render_rust_type();
                let type_tokens = quote_spanned! { arg_ident.span() => #ty };
                RustArg {
                    arg_ident,
                    type_tokens,
                    resolver_tokens: TokenStream::new(),
                }
            }
            Value(value) => {
                use crate::parse::rust_type::Value::*;
                match value {
                    String => {
                        let arg_ident = format_ident!("arg{}", slot);
                        let type_tokens = quote_spanned! { arg_ident.span() =>  *mut ::std::os::raw::c_char };
                        let resolver_tokens =
                            resolve_string_ptr(&arg_ident, true);
                        RustArg {
                            arg_ident,
                            type_tokens,
                            resolver_tokens,
                        }
                    }
                    Custom(_, type_name) => {
                        let arg_ident = format_ident!("arg{}", slot);
                        let ty = format_ident!("{}", type_name);
                        let type_tokens =
                            quote_spanned! { arg_ident.span() => #ty };
                        RustArg {
                            arg_ident,
                            type_tokens,
                            resolver_tokens: TokenStream::new(),
                        }
                    }
                    CString => todo!("RustArg::from::Value(CString)"),
                    Str => todo!("RustArg::from::Value(Str)"),
                }
            }
            Composite(_, _) => todo!("RustArg::from::Composite"),
            Unit => todo!("RustArg::from::Unit"),
            Unknown => unimplemented!("RustArg::from::Unknown"),
        }
    }

    pub fn render_typed_parameter(&self, span: Option<Span>) -> TokenStream {
        let RustArg {
            arg_ident,
            type_tokens,
            ..
        } = self;
        match span {
            Some(span) => quote_spanned! { span => #arg_ident: #type_tokens },
            None => quote! { #arg_ident: #type_tokens },
        }
    }
}