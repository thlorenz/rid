use proc_macro2::TokenStream;
use syn::Ident;

use crate::{attrs::TypeInfoMap, parse::rust_type::RustType};
use quote::{format_ident, quote};

/// Distinguishes between hash maps that are references to fields on structs or enums vs.
/// hash maps created during a method call and returned to Dart without keeping a reference
/// on the Rust side.
pub enum HashMapKind {
    /// HashMap is a reference to a field held onto by Rust
    FieldReference,
    /// HashMap is instantiated inside a method and returned as RidHashMap, not held onto by Rust
    MethodReturn,
}

pub struct HashMapAccess {
    /// Type of the HashMap
    pub hash_map_type: RustType,

    /// Identifier of type of the hash map, i.e. `HashMap`
    pub hash_map_type_ident: Ident,

    /// Type of the item enclosed by the hashSet
    /// Example: `String`
    pub item_type: RustType,

    /// FFI prelude applied to generated rust functions
    pub rust_ffi_prelude: TokenStream,

    /// The kind of the vector
    pub kind: HashMapKind,

    /// Name of function to get hash map length
    pub fn_len_ident: Ident,

    /// Name of function to get hash map item by key
    pub fn_get_ident: Ident,

    /// Name of function to query if hash map contains a key
    pub fn_contains_ident: Ident,

    /// Name of function to free hash map (not used for field access)
    pub fn_free_ident: Ident,
}

impl HashMapAccess {
    pub fn new(
        hash_map_ty: &RustType,
        hash_map_ty_ident: &Ident,
        kind: HashMapKind,
        ffi_prelude: &TokenStream,
    ) -> Self {
        let item_type = hash_map_ty
            .inner_composite_type()
            .expect("HashMap should have inner type");

        let key = Self::key_from_item_rust_ident(item_type.rust_ident(), &kind);

        let fn_len_ident = format_ident!("rid_len_{}", key);
        let fn_free_ident = format_ident!("rid_free_{}", key);
        let fn_get_ident = format_ident!("rid_get_{}", key);
        let fn_contains_ident = format_ident!("rid_contains_{}", key);

        Self {
            hash_map_type: hash_map_ty.clone(),
            hash_map_type_ident: hash_map_ty_ident.clone(),
            item_type,
            rust_ffi_prelude: ffi_prelude.clone(),
            fn_len_ident,
            fn_free_ident,
            fn_get_ident,
            fn_contains_ident,
            kind,
        }
    }

    pub fn key_from_item_rust_ident(
        ident: &Ident,
        kind: &HashMapKind,
    ) -> String {
        match kind {
            HashMapKind::FieldReference => format!("hash_map{}", ident),
            HashMapKind::MethodReturn => format!("ridhash_map{}", ident),
        }
        .to_lowercase()
    }

    pub fn key(&self) -> String {
        Self::key_from_item_rust_ident(self.item_type.rust_ident(), &self.kind)
    }
}
