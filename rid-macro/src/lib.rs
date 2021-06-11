use quote::quote;
use std::{env, process};

use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;

use rid_macro_impl::{
    rid_debug_impl, rid_display_impl, rid_export_impl, rid_ffi_model_impl,
    rid_ffi_reply_impl, rid_message_impl,
};
use syn::{self, parse_macro_input};

const RID_PRINT_MODEL: &str = "RID_PRINT_MODEL";
const RID_PRINT_MESSAGE: &str = "RID_PRINT_MESSAGE";
const RID_PRINT_REPLY: &str = "RID_PRINT_REPLY";

// -----------------
// #[rid::model]
// -----------------
#[proc_macro_attribute]
#[proc_macro_error]
pub fn model(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    if let Ok(_) = env::var(RID_PRINT_MODEL) {
        eprintln!("input: {:#?}", &item);
        rid_ffi_model_impl(&item);
        process::exit(0)
    } else {
        let exports = rid_ffi_model_impl(&item);
        let q = quote! {
            #item
            #exports
        };
        q.into()
    }
}

// -----------------
// #[rid::message]
// -----------------
#[proc_macro_attribute]
#[proc_macro_error]
pub fn message(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    let args = parse_macro_input!(attrs as syn::AttributeArgs);
    if let Ok(_) = env::var(RID_PRINT_MESSAGE) {
        eprintln!("input: {:#?}", &item);
        eprintln!("args: {:#?}", &args);
        rid_message_impl(&item, &args, Default::default());
        process::exit(0)
    } else {
        let exports = rid_message_impl(&item, &args, Default::default());
        let q = quote! {
            #[repr(C)]
            #item
            #exports
        };
        q.into()
    }
}

// -----------------
// #[rid::reply]
// -----------------
#[proc_macro_attribute]
#[proc_macro_error]
pub fn reply(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    let args = parse_macro_input!(attrs as syn::AttributeArgs);
    if let Ok(_) = env::var(RID_PRINT_REPLY) {
        eprintln!("input: {:#?}", &item);
        eprintln!("args: {:#?}", &args);
        rid_ffi_reply_impl(&item, &args);
        process::exit(0)
    } else {
        let impls = rid_ffi_reply_impl(&item, &args);
        let q = quote! {
            #[repr(C)]
            #item
            #impls
        };
        q.into()
    }
}

// -----------------
// #[rid::export]
// -----------------
#[proc_macro_attribute]
#[proc_macro_error]
pub fn export(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    let args = parse_macro_input!(attrs as syn::AttributeArgs);
    // TODO: is there any way to avoid this clone and/or is cloning the input cheaper?
    let exports = rid_export_impl(item.clone(), args, Default::default());
    let q = quote! {
        #item
        #exports
    };
    q.into()
}

// -----------------
// #[rid::structs]
// -----------------
#[proc_macro_attribute]
#[proc_macro_error]
pub fn structs(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}

// -----------------
// #[rid::enums]
// -----------------
#[proc_macro_attribute]
#[proc_macro_error]
pub fn enums(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}

// -----------------
// #[derive(rid::Display)]
// -----------------
#[proc_macro_derive(Display)]
#[proc_macro_error]
pub fn display(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::DeriveInput);
    rid_display_impl(&item, Default::default()).into()
}

// -----------------
// #[derive(rid::Debug)]
// -----------------
#[proc_macro_derive(Debug)]
#[proc_macro_error]
pub fn debug(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::DeriveInput);
    rid_debug_impl(&item, Default::default()).into()
}
