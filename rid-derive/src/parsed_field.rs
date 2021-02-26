use syn::Field;

use crate::{dart::DartType, rust::RustType};
use std::convert::TryFrom;

pub(crate) struct ParsedField {
    pub(crate) ident: syn::Ident,
    pub(crate) method_ident: syn::Ident,
    pub(crate) ty: syn::Type,
    pub(crate) rust_ty: Result<RustType, String>,
    pub(crate) dart_ty: Result<DartType, String>,
}

impl ParsedField {
    pub(crate) fn new(f: Field, method_prefix: &str) -> Self {
        let ident = f.ident.unwrap();
        let method_ident = method_ident_from_field(method_prefix, &ident);
        let ty = f.ty;
        let rust_ty = RustType::try_from(&ty);
        let dart_ty = DartType::try_from(&ty);

        Self {
            ident,
            method_ident,
            ty,
            rust_ty,
            dart_ty,
        }
    }
}

fn method_ident_from_field(method_prefix: &str, field_ident: &syn::Ident) -> syn::Ident {
    let fn_name = format!("{}_{}", method_prefix, field_ident);
    syn::Ident::new(&fn_name, field_ident.span())
}
