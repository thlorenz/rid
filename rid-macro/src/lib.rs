mod common;
mod message;
mod model;
mod templates;

use std::{env, process};

use message::attach::rid_ffi_message_impl;

use model::attach::rid_ffi_model_impl;
use proc_macro::TokenStream;

use syn::{self, parse_macro_input};

const RID_PRINT_MODEL: &str = "RID_PRINT_MODEL";
const RID_PRINT_MESSAGE: &str = "RID_PRINT_MESSAGE";

#[proc_macro_derive(Model, attributes(rid))]
pub fn model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    if let Ok(_) = env::var(RID_PRINT_MODEL) {
        println!("{:#?}", &input);
        process::exit(0)
    } else {
        rid_ffi_model_impl(input).into()
    }
}

#[proc_macro_derive(Message, attributes(rid))]
pub fn message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    if let Ok(_) = env::var(RID_PRINT_MESSAGE) {
        println!("input: {:#?}", &input);
        process::exit(0)
    } else {
        rid_ffi_message_impl(input).into()
    }
}
