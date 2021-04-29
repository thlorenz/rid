use crate::common::state::ImplementationType;

use super::state::get_state;
use quote::{format_ident, quote_spanned};
use rid_common::CSTRING_FREE;

type Tokens = proc_macro2::TokenStream;

pub fn resolve_ptr(ty: &syn::Ident) -> Tokens {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut #ty = &mut *ptr;
            ptr.as_mut().unwrap()
        }
    }
}

pub fn resolve_vec_ptr(ty: &syn::Ident) -> Tokens {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut Vec<#ty> = &mut *ptr;
            ptr.as_mut().unwrap()
        }
    }
}

pub fn resolve_string_ptr(arg: &syn::Ident, reassign: bool) -> Tokens {
    if reassign {
        quote_spanned! { arg.span() =>
            let #arg = unsafe { ::std::ffi::CString::from_raw(#arg) }
                .to_str()
                .expect("Received String that wasn't valid UTF-8.")
                .to_string();
        }
    } else {
        quote_spanned! { arg.span() =>
            unsafe { ::std::ffi::CString::from_raw(#arg) }
                .to_str()
                .expect("Received String that wasn't valid UTF-8.")
                .to_string()
        }
    }
}

pub fn cstring_free() -> Tokens {
    let cstring_free_ident = format_ident!("{}", CSTRING_FREE);
    if get_state().needs_implementation(&ImplementationType::Free, CSTRING_FREE)
    {
        quote_spanned! {
            proc_macro2::Span::call_site() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #cstring_free_ident(ptr: *mut ::std::os::raw::c_char) {
                if !ptr.is_null() {
                    ::core::mem::drop(unsafe { ::std::ffi::CString::from_raw(ptr) });
                }
            }
        }
    } else {
        Tokens::new()
    }
}

pub fn instance_ident(struct_ident: &syn::Ident) -> syn::Ident {
    format_ident!("{}", struct_ident.to_string().to_lowercase())
}
