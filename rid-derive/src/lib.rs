mod common;
mod message;
mod model;
mod templates;

use std::{env, process};

use message::attach::rid_ffi_message_impl;
use model::attach::rid_ffi_model_impl;
use proc_macro::TokenStream;

use syn::{self, parse_macro_input};

const RID_PRINT_AST: &str = "RID_PRINT_AST";

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    if let Ok(_) = env::var(RID_PRINT_AST) {
        println!("{:#?}", &item);
        process::exit(0)
    } else {
        rid_ffi_model_impl(item).into()
    }
}

#[proc_macro_attribute]
pub fn message(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    if let Ok(_) = env::var(RID_PRINT_AST) {
        println!("{:#?}", &item);
        process::exit(0)
    } else {
        rid_ffi_message_impl(item).into()
    }
}
