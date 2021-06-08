use syn::Field;

use crate::{
    attrs::TypeInfoMap,
    parse::{dart_type::DartType, rust_type::RustType},
};

#[derive(Debug)]
pub struct VariantField {
    pub ty: syn::Type,
    pub rust_ty: RustType,
    pub dart_ty: DartType,
    pub slot: usize,
}

impl VariantField {
    pub fn new(
        f: Field,
        slot: usize,
        types: &TypeInfoMap,
    ) -> Result<Self, String> {
        let ty = f.ty;
        let rust_ty = RustType::from_type(&ty, types)
            .expect(&format!("Encountered invalid rust type {:#?}", ty));
        let dart_ffi_ty = DartType::from(&rust_ty);

        Ok(Self {
            ty: ty.clone(),
            rust_ty,
            dart_ty: dart_ffi_ty,
            slot,
        })
    }
}
