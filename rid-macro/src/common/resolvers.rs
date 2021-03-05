use quote::{format_ident, quote_spanned};

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

pub fn instance_ident(struct_ident: &syn::Ident) -> syn::Ident {
    format_ident!("{}", struct_ident.to_string().to_lowercase())
}
