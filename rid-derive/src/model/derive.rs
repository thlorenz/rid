use crate::{common::errors::derive_error, model::parsed_struct::ParsedStruct};
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

pub fn rid_ffi_model_impl(ast: DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = ast.ident;
    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => {
            let parsed_struct = ParsedStruct::new(struct_ident, named);
            parsed_struct.derive_code()
        }
        _ => derive_error(&struct_ident, "Model can only be derived for structs"),
    }
}
