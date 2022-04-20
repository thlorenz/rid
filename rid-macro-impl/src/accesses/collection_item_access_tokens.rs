use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    parse::rust_type::RustType,
    render_common::PointerTypeAlias,
    render_rust::{render_return_type, RenderedReturnType},
};

use super::AccessKind;

pub struct CollectionItemAccessTokens {
    /// The tokens for the type that should be returned by the get item wrapper method
    pub item_return_type: TokenStream,

    /// The tokens to convert the type of the wrapped method into the one the wrapper returns
    pub into_return_type: TokenStream,

    /// Type aliases used as a part of the return type
    pub type_alias: Option<PointerTypeAlias>,
}

/// Helper function to figure out the return type and into return type tokens for
/// a specific collection type. Used for `Vec` get by index and `HashMap` get by key.
pub fn collection_item_access_tokens(
    ptr_ident: Ident,
    item_type: &RustType,
    access_kind: &AccessKind,
) -> CollectionItemAccessTokens {
    // CString and str aren't FFI safe so we send the item content as a *char.
    // For consistency we do the same for Strings.
    let (item_return_type, into_return_type, type_alias) = if item_type
        .is_string_like()
    {
        let item_return_type = quote! { *const ::std::os::raw::c_char };
        let into_return_type = if item_type.is_string() {
            quote! {
                let s: &String = unsafe {
                    assert!(!#ptr_ident.is_null());
                    let #ptr_ident: *mut String = #ptr_ident as *mut String;
                    #ptr_ident.as_mut().expect("resolve ptr from collection item as_mut failed")
                };
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        } else if item_type.is_cstring() {
            quote! {
                let cstring: &::std::ffi::CString = unsafe {
                    assert!(!#ptr_ident.is_null());
                    let #ptr_ident: *mut ::std::ffi::CString = #ptr_ident as *mut ::std::ffi::CString;
                    #ptr_ident.as_mut().expect("resolve_ptr.as_mut failed")
                };
                cstring.clone().into_raw()
            }
        } else if item_type.is_str() {
            quote! {
                let s: &str = unsafe {
                    assert!(!#ptr_ident.is_null());
                    let #ptr_ident: *mut str = #ptr_ident as *mut str;
                    #ptr_ident.as_mut()
                        .expect("resolve ptr from collection item as_mut failed")
                };
                let cstring = ::std::ffi::CString::new(s).unwrap();
                cstring.into_raw()
            }
        } else {
            panic!("Should have covered all string-like cases")
        };
        (item_return_type, into_return_type, None)
    } else {
        let RenderedReturnType {
            tokens, type_alias, ..
        } = render_return_type(&item_type, access_kind);
        (tokens, quote! { #ptr_ident }, type_alias)
    };
    CollectionItemAccessTokens {
        item_return_type,
        into_return_type,
        type_alias,
    }
}
