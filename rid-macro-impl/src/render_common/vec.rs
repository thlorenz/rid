use proc_macro2::TokenStream;
use syn::Ident;

use crate::{attrs::TypeInfoMap, parse::rust_type::RustType};
use quote::quote;

pub struct VecAccess {
    /// Type of the vector
    pub vec_type: RustType,

    /// Identifier used for the vector type, possibly using alias for inner type
    /// Example: `RidVec_Pointer_Item`
    pub vec_type_dart: String,

    /// Type of the item enclosed by the vector
    /// Example: `Pointer_Item`
    pub item_type: RustType,

    /// FFI prelude applied to generated rust functions
    pub rust_ffi_prelude: TokenStream,

    /// Name of function to get vector length
    pub fn_len_ident: Ident,

    /// Name of function to get vector item at index
    pub fn_get_ident: Ident,

    /// Name of function to free vector
    pub fn_free_ident: Ident,
}

pub fn render_vec_accesses(
    vec_accesses: &[VecAccess],
    type_infos: &TypeInfoMap,
    comment: &str,
) -> Vec<TokenStream> {
    vec_accesses
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
}
