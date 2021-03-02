mod common;
mod model;
mod templates;

use std::{env, process};

use model::derive::rid_ffi_model_impl;
use proc_macro::TokenStream;

use syn::{self, parse_macro_input, DeriveInput};

const RID_PRINT_AST: &str = "RID_PRINT_AST";

#[proc_macro_derive(Model)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    if let Ok(_) = env::var(RID_PRINT_AST) {
        println!("{:#?}", &ast);
        process::exit(0)
    } else {
        rid_ffi_model_impl(ast).into()
    }
}
