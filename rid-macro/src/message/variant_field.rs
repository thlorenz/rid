use proc_macro_error::abort;
use syn::Field;

use crate::{
    attrs::{TypeInfo, VariantConfig},
    common::{rust::ValueType, DartType, RustType},
};

#[derive(Debug)]
pub struct VariantField {
    pub ty: syn::Type,
    pub rust_ty: RustType,
    pub dart_ty: DartType,
    pub custom_ty: Option<TypeInfo>,
    pub slot: usize,
}

impl VariantField {
    pub fn new(f: Field, slot: usize, config: &VariantConfig) -> Result<Self, String> {
        let ty = f.ty;
        let rust_ty = RustType::try_from(&ty)
            .map_err(|err| format!("Encountered invalid rust type {:#?}\n{}", ty, err))?;

        // TODO: We don't currently verify that all type infos are being used. That would require
        // recording this across all variant fields.
        let custom_ty = if let (ident, RustType::Value(ValueType::RCustom(key))) = &rust_ty {
            match config.types.get(key) {
                Some(ty) => Some(ty.clone()),
                None => abort!(
                    ident,
                    // TODO: Include info regarding which custom types are viable, link to URL?
                    "Missing info for custom type {0}. \
                    Specify via '#[rid(types = {{ {0}: Enum }})]' or similar.",
                    key
                ),
            }
        } else {
            None
        };

        let dart_ty = match &rust_ty {
            (ident, ref rust_ty) => DartType::try_from(rust_ty, ident).map_err(|err| {
                format!("RustType {:#?} cannot be used in dart\n{}", rust_ty, err)
            })?,
        };

        Ok(Self {
            ty: ty.clone(),
            rust_ty: rust_ty.1,
            dart_ty,
            custom_ty,
            slot,
        })
    }
}
