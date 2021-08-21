use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use crate::{
    common::tokens::resolve_hash_map_ptr,
    render_common::{
        AccessKind, HashMapAccess, PointerTypeAlias, RenderedAccessRust,
    },
};

impl HashMapAccess {
    pub fn render_rust_field_access(&self) -> RenderedAccessRust {
        let ffi_prelude = &self.rust_ffi_prelude;
        let key_ty = self.key_type.rust_ident();
        let val_ty = self.val_type.rust_ident();

        let resolve_hash_map = resolve_hash_map_ptr(&key_ty, &val_ty);

        // -----------------
        // HashMap::len()
        // -----------------
        // NOTE: HashMap::len() is cheap as it just retrieves underlying `table.items` fields
        // @see: https://github.com/rust-lang/hashbrown/blob/2eaeebbe0fe5ed1627a7ff3e31d5ae084975a1f6/src/raw/mod.rs#L923-L925
        let fn_len_ident = &self.fn_len_ident;
        let len_impl = quote_spanned! { fn_len_ident.span() =>
            #ffi_prelude
            fn #fn_len_ident(ptr: *mut HashMap<#key_ty, #val_ty>) -> usize {
                #resolve_hash_map.len()
            }
        };

        // -----------------
        // HashMap::get(&key)
        // -----------------
        let fn_get_ident = &self.fn_get_ident;
        let fn_get_ident_str_tokens: TokenStream =
            format!("\"{}\"", fn_get_ident).parse().unwrap();

        // TODO(thlorenz): HashMap consider non-primitive key and/or val types
        let get_impl = quote_spanned! { fn_get_ident.span() =>
            #ffi_prelude
            fn #fn_get_ident<'a>(ptr: *mut HashMap<#key_ty, #val_ty>, key: #key_ty) -> Option<&'a #val_ty>  {
                let item = #resolve_hash_map.get(&key);
                item
            }
        };

        // -----------------
        // HashMap::contains_key(&key)
        // -----------------
        let fn_contains_key_ident = &self.fn_contains_key_ident;
        // TODO(thlorenz): HashMap consider non-primitive key types
        let contains_key_impl = quote_spanned! { fn_contains_key_ident.span() =>
            #ffi_prelude
            fn #fn_contains_key_ident(ptr: *mut HashMap<#key_ty, #val_ty>, key: #key_ty) -> u8  {
                let hash_map = #resolve_hash_map;
                if hash_map.contains_key(&key) {
                    1
                } else {
                    0
                }
            }
        };

        // -----------------
        // HashMap::keys
        // -----------------
        // TODO: HashMap - currently we use a `rid:export` for this in the tests, but ideally
        // we should just add this implementation

        let tokens = quote! {
            #len_impl
            #get_impl
            #contains_key_impl
        };
        RenderedAccessRust {
            tokens,
            type_aliases: vec![],
        }
    }

    pub fn render_rust_method_return(&self) -> RenderedAccessRust {
        todo!("HashMapAccess::render_rust_method_return")
    }
}
