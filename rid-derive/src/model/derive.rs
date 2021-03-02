use crate::model::parsed_struct::ParsedStruct;
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

pub fn rid_ffi_model_impl(ast: DeriveInput) -> proc_macro2::TokenStream {
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
