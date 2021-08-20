use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use rid_common::{CSTRING_FREE, UTILS_MODULE};

use crate::common::state::{get_state, ImplementationType};

pub fn utils_module_tokens_if(condition: bool) -> TokenStream {
    if condition {
        utils_module_tokens()
    } else {
        TokenStream::new()
    }
}

pub fn utils_module_tokens() -> TokenStream {
    let util_module_ident = format_ident!("{}", UTILS_MODULE);
    if get_state()
        .needs_implementation(&ImplementationType::UtilsModule, UTILS_MODULE)
    {
        let cstring_free = cstring_free();
        quote! {
            mod __rid_utils_module {
                #cstring_free
            }
        }
    } else {
        TokenStream::new()
    }
}

// -----------------
// CString
// -----------------
fn cstring_free() -> TokenStream {
    let cstring_free_ident = format_ident!("{}", CSTRING_FREE);
    quote_spanned! {
        proc_macro2::Span::call_site() =>
        #[no_mangle]
        pub extern "C" fn #cstring_free_ident(ptr: *mut ::std::os::raw::c_char) {
            if !ptr.is_null() {
                ::core::mem::drop(unsafe { ::std::ffi::CString::from_raw(ptr) });
            }
        }
    }
}

// -----------------
// String
// -----------------
fn string_ref_access() -> TokenStream {
    let string_ref_access_ident = format_ident!("{}", STRING_REF_ACCESS);
    quote_spanned! {
        proc_macro2::Span::call_site() =>
        #[no_mangle]
        pub extern "C" fn #string_ref_access_ident(
            ptr: *mut String,
        ) -> *const ::std::os::raw::c_char {
            let s: &String = unsafe {
                assert!(!ptr.is_null());
                let ptr: *mut String = &mut *ptr;
                ptr.as_mut().expect("resolve_ptr.as_mut failed")
            };

            let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
            cstring.into_raw()
        }
    }
}
