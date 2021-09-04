use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};

use crate::{
    accesses::{HashMapAccess, RenderedAccessRust},
    common::tokens::resolve_hash_map_ptr,
    parse::{rust_type::RustType, ParsedReference},
    render_common::PointerTypeAlias,
};

impl HashMapAccess {
    pub fn render_rust_field_access(&self) -> RenderedAccessRust {
        let ffi_prelude = &self.rust_ffi_prelude;
        let key_ty = self.key_type.rust_ident();
        let val_ty = self.val_type.rust_ident();
        let arg = format_ident!("hash_map_arg");

        let resolve_hash_map = resolve_hash_map_ptr(&arg, &key_ty, &val_ty);

        // TODO(thlorenz): need #[rid::structs]/#[rid::enums] via type_infos that need to be
        // provided

        // -----------------
        // HashMap::len()
        // -----------------
        let fn_len_ident = &self.fn_len_ident;
        let len_impl = quote_spanned! { fn_len_ident.span() =>
            #[rid::export]
            fn #fn_len_ident(map: &HashMap<#key_ty, #val_ty>) -> usize {
                map.len()
            }
        };

        // -----------------
        // HashMap::get(&key)
        // -----------------
        let fn_get_ident = &self.fn_get_ident;
        let fn_get_ident_str_tokens: TokenStream =
            format!("\"{}\"", fn_get_ident).parse().unwrap();

        // TODO(thlorenz): special case for String val types
        // see: ./accesses/collection_item_access_tokens.rs

        let get_impl = quote_spanned! { fn_get_ident.span() =>
            #[rid::export]
            fn #fn_get_ident(map: &HashMap<#key_ty, #val_ty>, key: #key_ty) -> Option<&#val_ty>  {
                map.get(&key)
            }
        };

        // -----------------
        // HashMap::contains_key(&key)
        // -----------------
        let fn_contains_key_ident = &self.fn_contains_key_ident;

        let contains_key_impl = quote_spanned! { fn_contains_key_ident.span() =>
            #[rid::export]
            fn #fn_contains_key_ident(map: &HashMap<#key_ty, #val_ty>, key: #key_ty) -> bool {
                map.contains_key(&key)
            }
        };

        // -----------------
        // HashMap::keys
        // -----------------
        let fn_keys_ident = &self.fn_keys_ident;

        let keys_impl = quote_spanned! { fn_keys_ident.span() =>
            #[rid::export]
            fn #fn_keys_ident(map: &HashMap<#key_ty, #val_ty>) -> Vec<&#key_ty> {
                map.keys().collect()
            }
        };

        let tokens = quote! {
            #len_impl
            #get_impl
            #contains_key_impl
            #keys_impl
        };
        RenderedAccessRust {
            tokens,
            type_aliases: HashMap::new(),
        }
    }

    pub fn render_rust_method_return(&self) -> RenderedAccessRust {
        todo!("HashMapAccess::render_rust_method_return")
    }
}
