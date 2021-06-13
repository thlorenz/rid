use syn::Field;

use crate::{
    attrs::TypeInfoMap,
    common::{abort, missing_msg_field_enum_info},
    parse::{
        dart_type::DartType,
        rust_type::{RustType, TypeKind},
    },
};

#[derive(Debug)]
pub struct VariantField {
    pub ty: syn::Type,
    pub rust_ty: RustType,
    pub dart_ty: DartType,
    pub slot: usize,
}

impl VariantField {
    pub fn new(f: Field, slot: usize, types: &TypeInfoMap) -> Self {
        let ty = f.ty;
        let rust_ty = RustType::from_type(&ty, types);
        let rust_ty = match rust_ty {
            Some(x) => x,
            None => abort!(f.ident, "invalid rust type"),
        };
        if &rust_ty.kind == &TypeKind::Unknown {
            missing_msg_field_enum_info(
                &f.ident.as_ref().unwrap_or(&rust_ty.rust_ident()),
            );
        };
        let dart_ffi_ty = DartType::from(&rust_ty);

        Self {
            ty: ty.clone(),
            rust_ty,
            dart_ty: dart_ffi_ty,
            slot,
        }
    }
}
