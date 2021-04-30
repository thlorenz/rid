use crate::common::state::ImplementationType;

use super::state::get_state;
use quote::{format_ident, quote_spanned};
use rid_common::CSTRING_FREE;

use proc_macro2::TokenStream;

pub fn resolve_ptr(ty: &syn::Ident) -> TokenStream {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut #ty = &mut *ptr;
            ptr.as_mut().unwrap()
        }
    }
}

pub fn resolve_vec_ptr(ty: &syn::Ident) -> TokenStream {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut Vec<#ty> = &mut *ptr;
            ptr.as_mut().unwrap()
        }
    }
}

pub fn resolve_string_ptr(arg: &syn::Ident, reassign: bool) -> TokenStream {
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

pub fn cstring_free() -> TokenStream {
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
        TokenStream::new()
    }
}

pub struct ResolvedEnumFromInt {
    pub arg_ident: syn::Ident,
    pub arg_type_ident: syn::Ident,
    pub instance_ident: syn::Ident,
    pub tokens: TokenStream,
}

/// Generates tokens to convert an int to an enum.
///
/// match f {
///     0 => Filter::Completed,
///     1 => Filter::Pending,
///     2 => Filter::All,
///     _ => panic!("Not a valid filter value"),
/// }
pub fn resolve_enum_from_int(
    enum_ident: &syn::Ident,
    variants: &[String],
) -> ResolvedEnumFromInt {
    let arg_type_ident = format_ident!("i32");
    let arg_ident = format_ident!("n");
    let instance_ident = format_ident!("instance");

    let variant_idents: Vec<TokenStream> = Vec::new();
    let variant_tokens: Vec<TokenStream> = variants
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            format!("{} => {}::{},\n", idx, enum_ident, x)
                .parse()
                .unwrap()
        })
        .collect();

    let default_branch: TokenStream =
        format!("_ => panic!(\"Not a valid {} value\",)", enum_ident)
            .parse()
            .unwrap();

    let tokens = quote_spanned! { enum_ident.span() =>
        let #instance_ident = match #arg_ident {
            #(#variant_tokens)*
            #default_branch
        };
    };

    ResolvedEnumFromInt {
        arg_ident,
        arg_type_ident,
        instance_ident,
        tokens,
    }
}
pub fn instance_ident(struct_ident: &syn::Ident) -> syn::Ident {
    format_ident!("{}", struct_ident.to_string().to_lowercase())
}
