use std::{env, fs, process};

use proc_macro::TokenStream;

use crate::parsed_struct::ParsedStruct;
use syn::{self, parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

mod dart;
mod parsed_field;
mod parsed_struct;
mod rust;

const RID_PRINT_AST: &str = "RID_PRINT_AST";

#[proc_macro_derive(Rid)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    if let Ok(_) = env::var(RID_PRINT_AST) {
        println!("{:#?}", &ast);
        process::exit(0)
    } else {
        rid_ffi_impl(ast).into()
    }
}

fn rid_ffi_impl(ast: DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = ast.ident;
    let struct_fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => todo!(),
        _ => unimplemented!(),
    };
    let parsed_struct = ParsedStruct::new(struct_ident, struct_fields);
    parsed_struct.derive_code()
}

fn _emit_dart_binding(dart: &str) {
    const RID_BINDING_DART: &str = "RID_BINDING_DART";
    if let Ok(file) = env::var(RID_BINDING_DART) {
        fs::write(&file, dart).expect(&format!("Unable to write dart bindings to {}", &file));
    }
}
