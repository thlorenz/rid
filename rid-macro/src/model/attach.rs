use crate::{
    common::{callsite_error, ParsedDerive},
    model::parsed_struct::ParsedStruct,
};
use syn::{self, Fields, FieldsNamed};

pub fn rid_ffi_model_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = input.ident;
    let model_struct = match &input.data {
        syn::Data::Struct(s) => s,
        _ => return callsite_error("model can only be attached to structs"),
    };

    let derive: ParsedDerive = ParsedDerive::from_attrs(&input.attrs);

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
