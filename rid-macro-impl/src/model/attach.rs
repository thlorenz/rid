use crate::{
    attrs::{parse_rid_attrs_old, StructConfig},
    common::callsite_error,
    model::parsed_struct::ParsedStruct,
};
use syn::{self, Fields, FieldsNamed};

pub fn rid_ffi_model_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident = input.ident;
    let model_struct = match &input.data {
        syn::Data::Struct(s) => s,
        _ => return callsite_error("model can only be attached to structs"),
    };
    let rid_attrs = parse_rid_attrs_old(&input.attrs, None);
    let struct_config = StructConfig::new(&rid_attrs);

    match model_struct {
        syn::DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        } => {
            let parsed_struct =
                ParsedStruct::new(struct_ident, named.clone(), struct_config);
            parsed_struct.tokens()
        }
        syn::DataStruct {
            fields: Fields::Unit,
            ..
        } => callsite_error("model attribute makes no sense on empty structs"),
        _ => unimplemented!(),
    }
}
