use proc_macro_error::abort;
use syn::Field;

use crate::{
    attrs::{FieldConfig, TypeInfo},
    common::{rust::ValueType, DartType, RustType},
};

#[derive(Debug)]
pub struct VariantField {
    pub ty: syn::Type,
    pub rust_ty: RustType,
    pub dart_ty: DartType,
    pub slot: usize,
}

impl VariantField {
    pub fn new(f: Field, slot: usize, config: &FieldConfig) -> Result<Self, String> {
        let ty = f.ty;
        let rust_ty = RustType::try_from(&ty, &config.types)
            .map_err(|err| format!("Encountered invalid rust type {:#?}\n{}", ty, err))?;

        let dart_ty = match &rust_ty {
            (ident, ref rust_ty) => DartType::try_from(rust_ty, ident).map_err(|err| {
                format!("RustType {:#?} cannot be used in dart\n{}", rust_ty, err)
            })?,
        };

        Ok(Self {
            ty: ty.clone(),
            rust_ty: rust_ty.1,
            dart_ty,
            slot,
        })
    }
}
