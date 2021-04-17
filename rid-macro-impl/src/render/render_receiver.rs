use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};

use crate::{
    attrs::TypeInfo,
    parse::{
        rust_type::{RustType, TypeKind, Value},
        ParsedReceiver,
    },
};

use super::{render_pointer_type, render_rust_type};

/*
    #[no_mangle]
    pub extern "C" fn get_id_mut(ptr: *mut Model) -> u32 {
        let model: &mut Model = unsafe {
            assert!(!ptr.is_null());
            ptr.as_mut().unwrap()
        };
        Model::id(model)
    }

    #[no_mangle]
    pub extern "C" fn get_id_ref(ptr: *const Model) -> u32 {
        let model: &Model = unsafe {
            assert!(!ptr.is_null());
            ptr.as_ref().unwrap()
        };
        Model::id(model)
    }
*/

// NOTE: for now assuming that all receivers are custom structs
pub struct ReceiverArg {
    pub arg_pass: TokenStream,
    pub arg_resolve: TokenStream,
    pub receiver_ident: syn::Ident,
}

pub fn render_receiver_arg(receiver: &ParsedReceiver) -> ReceiverArg {
    let ptr_ident: syn::Ident = format_ident!("ptr");
    let receiver_ident: syn::Ident = format_ident!("receiver");
    let ParsedReceiver {
        ref info,
        reference,
    } = receiver;
    let kind =
        TypeKind::Value(Value::Custom(info.clone(), info.key.to_string()));
    let rust_type = RustType::new(info.key.clone(), kind, reference.clone());
    let arg_pass = render_args_pass(&ptr_ident, info, &rust_type);
    let arg_resolve =
        render_arg_resolve(&ptr_ident, &&receiver_ident, info, &rust_type);
    ReceiverArg {
        arg_pass,
        arg_resolve,
        receiver_ident,
    }
}

fn render_args_pass(
    ptr_ident: &syn::Ident,
    type_info: &TypeInfo,
    rust_type: &RustType,
) -> TokenStream {
    let ptr_type_toks = render_pointer_type(rust_type).tokens;
    quote_spanned! { type_info.key.span() => #ptr_ident: #ptr_type_toks }
}

fn render_arg_resolve(
    ptr_ident: &syn::Ident,
    arg_ident: &syn::Ident,
    type_info: &TypeInfo,
    rust_type: &RustType,
) -> TokenStream {
    let arg_type_toks = render_rust_type(rust_type, false).tokens;
    let as_ident = rust_type.reference.render_deref();
    quote_spanned! { type_info.key.span() =>
        let #arg_ident: #arg_type_toks = unsafe {
            assert!(!#ptr_ident.is_null());
            #ptr_ident#as_ident.unwrap()
        };
    }
}
