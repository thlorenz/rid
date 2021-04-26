use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use render_pointer_type::RenderedPointerType;
use syn::Ident;

use crate::{
    attrs::TypeInfo,
    parse::{
        rust_type::{RustType, TypeKind, Value},
        ParsedReceiver,
    },
    render_common::TypeAlias,
};

use super::{render_pointer_type, render_to_return_type};

// NOTE: for now assuming that all receivers are custom structs
pub struct ReceiverArg {
    pub arg_pass: RenderedArgsPass,
    pub arg_resolve: TokenStream,
    pub receiver_ident: syn::Ident,
}

impl ParsedReceiver {
    pub fn render_receiver_arg(&self) -> ReceiverArg {
        let ptr_ident: syn::Ident = format_ident!("ptr");
        let receiver_ident: syn::Ident = format_ident!("receiver");
        let ParsedReceiver {
            ref info,
            reference,
        } = self;
        let kind =
            TypeKind::Value(Value::Custom(info.clone(), info.key.to_string()));
        let rust_type =
            RustType::new(info.key.clone(), kind, reference.clone());
        let arg_pass = render_args_pass(&ptr_ident, info, &rust_type);
        let arg_resolve =
            render_arg_resolve(&ptr_ident, &&receiver_ident, info, &rust_type);
        ReceiverArg {
            arg_pass,
            arg_resolve,
            receiver_ident,
        }
    }
}

pub struct RenderedArgsPass {
    pub tokens: TokenStream,
    pub type_alias: Option<TypeAlias>,
}

impl RenderedArgsPass {
    pub fn empty() -> Self {
        Self {
            tokens: TokenStream::new(),
            type_alias: None,
        }
    }
}

fn render_args_pass(
    ptr_ident: &syn::Ident,
    type_info: &TypeInfo,
    rust_type: &RustType,
) -> RenderedArgsPass {
    let RenderedPointerType {
        alias,
        tokens: ptr_type_toks,
    } = rust_type.render_pointer_type();
    RenderedArgsPass {
        type_alias: alias,
        tokens: quote_spanned! { type_info.key.span() => #ptr_ident: #ptr_type_toks },
    }
}

fn render_arg_resolve(
    ptr_ident: &syn::Ident,
    arg_ident: &syn::Ident,
    type_info: &TypeInfo,
    rust_type: &RustType,
) -> TokenStream {
    let arg_type_toks = rust_type.render_rust_type().tokens;
    let as_ident = rust_type.reference.render_deref();
    quote_spanned! { type_info.key.span() =>
        let #arg_ident: #arg_type_toks = unsafe {
            assert!(!#ptr_ident.is_null());
            #ptr_ident#as_ident.unwrap()
        };
    }
}
