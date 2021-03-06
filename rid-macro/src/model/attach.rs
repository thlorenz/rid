use crate::{
    common::{callsite_error, ParsedDerive},
    model::parsed_struct::ParsedStruct,
};
use syn::{self, Fields, FieldsNamed};

pub fn rid_ffi_model_impl(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = ast.ident;
    let model_struct = match &ast.data {
        syn::Data::Struct(s) => s,
        _ => return callsite_error("model can only be attached to structs"),
    };

    let derive: ParsedDerive = ParsedDerive::from_attrs(&ast.attrs);

    match model_struct {
        syn::DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        } => {
            let parsed_struct = ParsedStruct::new(struct_ident, named.clone(), derive);
            parsed_struct.tokens()
        }
        syn::DataStruct {
            fields: Fields::Unit,
            ..
        } => callsite_error("model attribute makes no sense on empty structs"),
        _ => unimplemented!(),
    }
}
