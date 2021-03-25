use quote::quote;
use std::{env, process};

use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;

use rid_macro_impl::{
    parse_rid_attrs, rid_export_impl, rid_ffi_message_impl, rid_ffi_model_impl,
};
use syn::{self, parse_macro_input};

const RID_PRINT_MODEL: &str = "RID_PRINT_MODEL";
const RID_PRINT_MESSAGE: &str = "RID_PRINT_MESSAGE";

#[proc_macro_derive(Model, attributes(rid))]
#[proc_macro_error]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let Ok(_) = env::var(RID_PRINT_MESSAGE) {
        return TokenStream::new();
    }
    if let Ok(_) = env::var(RID_PRINT_MODEL) {
        //    println!("{:#?}", &input);
        let args = parse_rid_attrs(&input.attrs);
        println!("{:#?}", &args);
        process::exit(0)
    } else {
        rid_ffi_model_impl(input).into()
    }
}

#[proc_macro_derive(Message, attributes(rid))]
#[proc_macro_error]
pub fn message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    if let Ok(_) = env::var(RID_PRINT_MESSAGE) {
        // println!("input: {:#?}", &input);
        rid_ffi_message_impl(input);
        process::exit(0)
    } else {
        rid_ffi_message_impl(input).into()
    }
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
