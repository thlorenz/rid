use proc_macro2::TokenStream;
use syn::Ident;

use crate::{attrs::TypeInfoMap, parse::rust_type::RustType};
use quote::{format_ident, quote};

use super::AccessKind;

pub struct HashMapAccess {
    /// Type of the HashMap
    pub hash_map_type: RustType,

    /// Identifier of type of the hash map, i.e. `HashMap`
    pub hash_map_type_ident: Ident,

    /// Type of the key item enclosed by the HashMap
    /// Example: `String`
    pub key_type: RustType,
    ///
    /// Type of the val item enclosed by the HashMap
    /// Example: `String`
    pub val_type: RustType,

    /// FFI prelude applied to generated rust functions
    pub rust_ffi_prelude: TokenStream,

    /// The kind of the hash map, i.e. returned from a method or a field
    pub kind: AccessKind,

    /// Name of function to get hash map length
    pub fn_len_ident: Ident,

    /// Name of function to get hash map item by key
    pub fn_get_ident: Ident,

    /// Name of function to query if hash map contains a key
    pub fn_contains_key_ident: Ident,

    /// Name of function to retrieve a Vec containing the keys of the hash map
    pub fn_keys_ident: Ident,

    /// Name of function to free hash map (not used for field access)
    pub fn_free_ident: Ident,
}

impl HashMapAccess {
    pub fn new(
        hash_map_ty: &RustType,
        hash_map_ty_ident: &Ident,
        kind: AccessKind,
        ffi_prelude: &TokenStream,
    ) -> Self {
        let (key_type, val_type) = hash_map_ty
            .key_val_composite_types()
            .expect("HashMap should have key/val types");

        let key = Self::key_from_item_rust_ident(
            key_type.rust_ident(),
            val_type.rust_ident(),
            &kind,
        );

        let fn_len_ident = format_ident!("rid_len_{}", key);
        let fn_free_ident = format_ident!("rid_free_{}", key);
        let fn_get_ident = format_ident!("rid_get_{}", key);
        let fn_contains_key_ident = format_ident!("rid_contains_key_{}", key);
        let fn_keys_ident = format_ident!("rid_keys_{}", key);

        Self {
            hash_map_type: hash_map_ty.clone(),
            hash_map_type_ident: hash_map_ty_ident.clone(),
            key_type,
            val_type,
            rust_ffi_prelude: ffi_prelude.clone(),
            fn_len_ident,
            fn_free_ident,
            fn_get_ident,
            fn_contains_key_ident,
            fn_keys_ident,
            kind,
        }
    }

    pub fn key_from_item_rust_ident(
        key_ident: &Ident,
        val_ident: &Ident,
        kind: &AccessKind,
    ) -> String {
        match kind {
            AccessKind::FieldReference => {
                format!("hash_map_{}_{}", key_ident, val_ident)
            }
            AccessKind::MethodReturn => {
                format!("ridhash_map_{}_{}", key_ident, val_ident)
            }
        }
        .to_lowercase()
    }
}

pub fn render_hash_map_accesses(
    hash_map_accesses: &[HashMapAccess],
    type_infos: &TypeInfoMap,
    comment: &str,
) -> Vec<TokenStream> {
    todo!("render_hash_map_accesses")
    /*
    hash_map_accesses
        .iter()
        .map(|x| {
            let rust_tokens = x.render_rust().tokens;

            let implement_vecs = x.render_dart(type_infos, comment);
            let dart_string: String = format!(
                r###"
            {comment} Vector access methods matching the below Rust methods.
            {comment}
            {comment} ```dart
            {implement_vecs}
            {comment} ```"###,
                comment = comment,
                implement_vecs = implement_vecs
            );
            let dart_tokens: TokenStream = dart_string.parse().unwrap();

            quote! {
                #dart_tokens
                #rust_tokens
            }
        })
        .collect()
        */
}
