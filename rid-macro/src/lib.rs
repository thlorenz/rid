use quote::quote;
use std::{env, process};

use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;

use rid_macro_impl::{
    rid_display_impl, rid_export_impl, rid_ffi_message_impl, rid_ffi_model_impl,
};
use syn::{self, parse_macro_input};

const RID_PRINT_MODEL: &str = "RID_PRINT_MODEL";
const RID_PRINT_MESSAGE: &str = "RID_PRINT_MESSAGE";

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

#[proc_macro_attribute]
#[proc_macro_error]
pub fn message(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    let args = parse_macro_input!(attrs as syn::AttributeArgs);
    if let Ok(_) = env::var(RID_PRINT_MESSAGE) {
        eprintln!("input: {:#?}", &item);
        eprintln!("args: {:#?}", &args);
        rid_ffi_message_impl(&item, &args);
        process::exit(0)
    } else {
        let exports = rid_ffi_message_impl(&item, &args);
        let q = quote! {
            #[repr(C)]
            #item
            #exports
        };
        q.into()
    }
}

#[proc_macro_derive(Display)]
#[proc_macro_error]
pub fn display(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::DeriveInput);
    rid_display_impl(&item, Default::default()).into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn export(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    let args = parse_macro_input!(attrs as syn::AttributeArgs);
    // TODO: is there any way to avoid this clone and/or is cloning the input cheaper?
    let exports = rid_export_impl(item.clone(), args);
    let q = quote! {
        #item
        #exports
    };
    q.into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn types(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn structs(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn enums(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}
