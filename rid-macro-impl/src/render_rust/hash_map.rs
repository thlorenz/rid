use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};

use crate::{
    common::tokens::resolve_hash_map_ptr,
    parse::{rust_type::RustType, ParsedReference},
    render_common::{
        AccessKind, HashMapAccess, PointerTypeAlias, RenderableAccess,
        RenderedAccessRust, VecAccess,
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
            fn #fn_len_ident(ptr: *const HashMap<#key_ty, #val_ty>) -> usize {
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
            fn #fn_get_ident<'a>(ptr: *const HashMap<#key_ty, #val_ty>, key: #key_ty) -> Option<&'a #val_ty>  {
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
            fn #fn_contains_key_ident(ptr: *const HashMap<#key_ty, #val_ty>, key: #key_ty) -> u8  {
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
        let type_alias = PointerTypeAlias::for_const_pointer(
            &key_ty.to_string(),
            &key_ty.to_token_stream(),
            false,
        );
        let key_ty_alias = &type_alias.alias;

        let fn_keys_ident = &self.fn_keys_ident;
        let keys_impl = quote_spanned! { fn_keys_ident.span() =>
            #ffi_prelude
            fn #fn_keys_ident(ptr: *const HashMap<#key_ty, #val_ty>) -> rid::RidVec<#key_ty_alias> {
                let map: &HashMap<#key_ty, #val_ty> = #resolve_hash_map;
                let ret: Vec<#key_ty_alias> = map.keys().map(|x| x as #key_ty_alias).collect();
                let ret_ptr = rid::RidVec::from(ret);
                ret_ptr
            }
        };

        // -----------------
        // HashMap::keys VecAccess
        // -----------------

        // In order to iterate the RidVec<key> that is returned from the above method,
        // we need to ensure the access functions are rendered.
        let vec_ty = RustType::from_vec_with_pointer_alias(
            &key_ty,
            ParsedReference::Ref(None),
        );
        let vec_access = VecAccess::new(
            &vec_ty,
            vec_ty.rust_ident().clone(),
            AccessKind::MethodReturn,
            &ffi_prelude,
        );

        let mut nested_accesses: HashMap<String, Box<dyn RenderableAccess>> =
            HashMap::new();
        nested_accesses.insert(vec_access.key(), Box::new(vec_access));

        let tokens = quote! {
            #len_impl
            #get_impl
            #contains_key_impl
            #keys_impl
        };
        let type_aliases = {
            let mut map = HashMap::new();
            map.insert(type_alias.alias.to_string(), type_alias);
            map
        };
        RenderedAccessRust {
            tokens,
            type_aliases,
            nested_accesses: Some(nested_accesses),
        }
    }

    pub fn render_rust_method_return(&self) -> RenderedAccessRust {
        todo!("HashMapAccess::render_rust_method_return")
    }
}
