use proc_macro2::TokenStream;
use syn::Ident;

use crate::{attrs::TypeInfoMap, parse::rust_type::RustType};
use quote::{format_ident, quote};

use super::AccessKind;

pub struct VecAccess {
    /// Type of the vector
    pub vec_type: RustType,

    /// Identifier of type of the vector, i.e. `Vec`
    pub vec_type_ident: Ident,

    /// Identifier used for the vector type, possibly using alias for inner type
    /// Example: `RidVec_Pointer_Item`
    pub vec_type_dart: String,

    /// Type of the item enclosed by the vector
    /// Example: `Pointer_Item` or `Todo`
    pub item_type: RustType,

    /// FFI prelude applied to generated rust functions
    pub rust_ffi_prelude: TokenStream,

    /// Name of function to get vector length
    pub fn_len_ident: Ident,

    /// Name of function to get vector item at index
    pub fn_get_ident: Ident,

    /// Name of function to free vector
    pub fn_free_ident: Ident,

    /// The kind of the vector
    pub kind: AccessKind,
}

impl VecAccess {
    pub fn new(
        vec_ty: &RustType,
        vec_ty_ident: Ident,
        kind: AccessKind,
        ffi_prelude: &TokenStream,
    ) -> Self {
        let item_type = vec_ty
            .inner_composite_type()
            .expect("Vec should have inner type");

        let key = Self::key_from_item_rust_ident(item_type.rust_ident(), &kind);

        let fn_len_ident = format_ident!("rid_len_{}", key);
        let fn_free_ident = format_ident!("rid_free_{}", key);
        let fn_get_ident = format_ident!("rid_get_item_{}", key);

        VecAccess {
            vec_type: vec_ty.clone(),
            vec_type_ident: vec_ty_ident.clone(),
            vec_type_dart: format!("RidVec_{}", vec_ty_ident),
            item_type,
            rust_ffi_prelude: ffi_prelude.clone(),
            fn_len_ident,
            fn_free_ident,
            fn_get_ident,
            kind,
        }
    }

    pub fn key_from_item_rust_ident(
        ident: &Ident,
        kind: &AccessKind,
    ) -> String {
        match kind {
            AccessKind::FieldReference => format!("vec_{}", ident),
            AccessKind::MethodReturn => format!("ridvec_{}", ident),
        }
        .to_lowercase()
    }
}
