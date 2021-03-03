use syn::Field;

use crate::common::{DartType, RustType};

#[derive(Debug)]
pub struct VariantField {
    pub ty: syn::Type,
    pub rust_ty: Result<RustType, String>,
    pub dart_ty: Result<DartType, String>,
}

impl VariantField {
    pub fn new(f: Field) -> Self {
        let ty = f.ty;

        let rust_res = RustType::try_from(&ty);
        let dart_ty = match &rust_res {
            Ok((ident, ref rust_ty)) => DartType::try_from(rust_ty, ident),
            Err(_) => Err("Dart type not determined due to invalid Rust type".to_string()),
        };
        let rust_ty = rust_res.map(|(_, rust_ty)| rust_ty);

        Self {
            ty,
            rust_ty,
            dart_ty,
        }
    }
}
