use crate::{common::state::ImplementationType, parse::rust_type::RustType};

use super::state::get_state;
use quote::{format_ident, quote_spanned};

use proc_macro2::TokenStream;

pub fn resolve_ptr(ty: &syn::Ident) -> TokenStream {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut #ty = &mut *ptr;
            ptr.as_mut().expect("resolve_ptr.as_mut failed")
        }
    }
}

pub fn resolve_vec_ptr(ty: &syn::Ident) -> TokenStream {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut Vec<#ty> = &mut *ptr;
            ptr.as_mut().expect("resolve_vec_ptr.as_mut failed")
        }
    }
}

pub fn resolve_vec_msg_ptr(
    arg: &syn::Ident,
    len: &syn::Ident,
    ty: &Box<RustType>,
) -> TokenStream {
    // Always *const u8 input
    if ty.kind.is_string_like() {
        let ty = ty.rust_ident();
        let arg_out = format_ident!("{arg}_out");
        quote_spanned! { ty.span() =>
            let #arg: Vec<String> = unsafe {
                use std::os::raw::c_char;
                use std::ffi::CStr;
                use std::slice;
                //TODO: Error handling!
                slice::from_raw_parts(#arg, #len)
                    .into_iter()
                    .map(|&s| CStr::from_ptr(s).to_str().unwrap_or("Nope").to_owned())
                    .collect()
            };
            println!("Finished string processing!");
        }
    } else {
        let ty = ty.rust_ident();
        quote_spanned! { ty.span() =>
            let #arg: Vec<#ty> = unsafe {
                assert!(!#arg.is_null());
                std::slice::from_raw_parts(#arg, #len).to_vec()
            };
        }
    }
}

pub fn resolve_hash_set_ptr(ty: &syn::Ident) -> TokenStream {
    quote_spanned! { ty.span() =>
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut ::std::collections::HashSet<#ty> = &mut *ptr;
            ptr.as_mut().expect("resolve_hash_set_ptr.as_mut failed")
        }
    }
}

pub fn resolve_hash_map_ptr(
    arg: &syn::Ident,
    key_ty: &syn::Ident,
    val_ty: &syn::Ident,
) -> TokenStream {
    quote_spanned! { key_ty.span() =>
        let #arg: &HashMap<#key_ty, #val_ty> = unsafe {
            assert!(!#arg.is_null());
            #arg.as_ref().expect("resolve_hash_map_ptr.as_mut failed")
        };
    }
}

pub fn resolve_hash_map_msg_ptr(
    arg: &syn::Ident,
    key_ty: &syn::Ident,
    val_ty: &syn::Ident,
) -> TokenStream {
    quote_spanned! { key_ty.span() =>
        let #arg: HashMap<#key_ty, #val_ty> = unsafe {
            assert!(!#arg.is_null());
            #arg.read()
        };
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

pub fn resolve_bool_from_u8(arg: &syn::Ident, reassign: bool) -> TokenStream {
    if reassign {
        quote_spanned! { arg.span() => let #arg = if #arg == 0 {  false } else { true }; }
    } else {
        quote_spanned! { arg.span() => if #arg == 0 {  false } else { true } }
    }
}

pub struct ResolvedEnumFromInt {
    pub arg_ident: syn::Ident,
    pub arg_type_ident: syn::Ident,
    pub instance_ident: syn::Ident,
    pub tokens: TokenStream,
}

/// TODO: this is a duplicate of what ParsedEnum now does for us much better
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
