mod common;
mod message;
mod model;
mod templates;

use quote::quote;
use std::{convert::TryFrom, env, process};

use common::callsite_error;
use message::{attach::rid_ffi_message_impl, message_args::MessageArgs};
use model::attach::rid_ffi_model_impl;
use proc_macro::TokenStream;

use syn::{self, parse_macro_input};

const RID_PRINT_MODEL: &str = "RID_PRINT_MODEL";
const RID_PRINT_MESSAGE: &str = "RID_PRINT_MESSAGE";

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);
    if let Ok(_) = env::var(RID_PRINT_MODEL) {
        println!("{:#?}", &item);
        process::exit(0)
    } else {
        rid_ffi_model_impl(item).into()
    }
}

#[proc_macro_attribute]
pub fn message(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let item = parse_macro_input!(input as syn::Item);

    if let Ok(_) = env::var(RID_PRINT_MESSAGE) {
        // println!("input: {:#?}", &item);
        println!("args:  {:#?}", &args);
        process::exit(0)
    } else {
        match MessageArgs::try_from(args) {
            Ok(args) => rid_ffi_message_impl(item, args).into(),
            Err(errors) => {
                let errors = errors.into_iter().map(|err| callsite_error(&err));
                let q = quote! {
                    #(#errors)*
                    #item
                };
                q.into()
            }
        }
    }
}
