use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::Ident;

use crate::render_common::TypeAlias;

pub struct RenderedTypeAliasInfo {
    pub alias: String,
    pub fn_ident: Ident,
}

impl TypeAlias {
    pub const POINTER_ALIAS_PREFIX: &'static str = "Pointer_";
    pub const POINTER_MUT_ALIAS_PREFIX: &'static str = "PointerMut_";

    pub fn render_free(
        &self,
        ffi_prefix: TokenStream,
    ) -> (RenderedTypeAliasInfo, TokenStream) {
        let alias = &self.alias;
        let fn_ident = format_ident!("rid_free_{}", self.type_name);

        let tokens = quote_spanned! { self.alias.span() =>
            #ffi_prefix
            fn #fn_ident(ptr: #alias) {
                let instance = unsafe {
                    assert!(!ptr.is_null());
                    let ptr: #alias = &mut *ptr;
                    let ptr = ptr.as_mut().unwrap();
                    Box::from_raw(ptr)
                };
                drop(instance);
            }
        };
        (
            RenderedTypeAliasInfo {
                alias: alias.to_string(),
                fn_ident,
            },
            tokens,
        )
    }
}
