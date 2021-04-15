use syn::Field;

use crate::attrs::{parse_rid_attrs_old, TypeInfoMap};

use super::{DartType, RustType};

#[derive(Debug)]
pub struct ParsedField {
    pub ident: syn::Ident,
    pub method_ident: syn::Ident,
    pub ty: syn::Type,
    pub rust_ty: Result<RustType, String>,
    pub dart_ty: Result<DartType, String>,
}

impl ParsedField {
    pub fn new(f: &Field, method_prefix: &str, types: &TypeInfoMap) -> Self {
        let ident = f.ident.as_ref().unwrap().clone();
        let method_ident = method_ident_from_field(method_prefix, &ident);
        let ty = f.ty.clone();

        let rust_res = RustType::try_from(&ty, types);
        let dart_ty = match &rust_res {
            Ok((ident, ref rust_ty)) => DartType::try_from(rust_ty, ident),
            Err(_) => {
                Err("Dart type not determined due to invalid Rust type"
                    .to_string())
            }
        };
        let rust_ty = rust_res.map(|(_, rust_ty)| rust_ty);

        Self {
            ident,
            method_ident,
            ty,
            rust_ty,
            dart_ty,
        }
    }
}

fn method_ident_from_field(
    method_prefix: &str,
    field_ident: &syn::Ident,
) -> syn::Ident {
    let fn_name = format!("{}_{}", method_prefix, field_ident);
    syn::Ident::new(&fn_name, field_ident.span())
}
