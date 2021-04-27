use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};

use crate::{
    common::resolvers::{instance_ident, resolve_ptr},
    parse::rust_type::RustType,
};

pub struct RenderedStructDebugImpl {
    pub tokens: TokenStream,
    pub fn_debug_method_name: String,
    pub fn_debug_pretty_method_name: String,
}

impl RenderedStructDebugImpl {
    pub fn empty() -> Self {
        RenderedStructDebugImpl {
            tokens: TokenStream::new(),
            fn_debug_method_name: "".to_string(),
            fn_debug_pretty_method_name: "".to_string(),
        }
    }
}
impl RustType {
    pub fn render_struct_debug_impl(&self) -> RenderedStructDebugImpl {
        let method_prefix =
            format!("rid_{}", self.ident.to_string().to_lowercase())
                .to_string();

        let struct_ident = &self.ident;
        let struct_instance_ident = instance_ident(struct_ident);

        // TODO: consider using type aliases over `*mut` types via `self.render_pointer_type()`
        let resolve_struct_ptr = resolve_ptr(struct_ident);

        let fn_debug_ident = format_ident!("{}_debug", method_prefix);
        let fn_debug_pretty_ident =
            format_ident!("{}_debug_pretty", method_prefix);

        let tokens = quote_spanned! { struct_ident.span() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_debug_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                let #struct_instance_ident = #resolve_struct_ptr;
                let s = format!("{:?}", #struct_instance_ident);
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_debug_pretty_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                let #struct_instance_ident = #resolve_struct_ptr;
                let s = format!("{:#?}", #struct_instance_ident);
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        };

        RenderedStructDebugImpl {
            tokens,
            fn_debug_method_name: fn_debug_ident.to_string(),
            fn_debug_pretty_method_name: fn_debug_pretty_ident.to_string(),
        }
    }
}
