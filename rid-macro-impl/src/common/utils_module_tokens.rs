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
        let cstring_struct_declaration = cstring_struct_declaration();
        let str_struct_declaration = str_struct_declaration();
        let cstring_free = cstring_free();
        let init_msg_isolate = init_msg_isolate();
        let init_reply_isolate = init_reply_isolate();
        quote! {
            mod __rid_utils_module {
                #str_struct_declaration
                #cstring_struct_declaration
                #cstring_free
                #init_msg_isolate
                #init_reply_isolate
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

// cbindgen doesn't know what CString is, also technically it is not FFI safe.
// We just use it to point to items in a collection, Never actually sending a pointer directly and
// resolving it later.
fn cstring_struct_declaration() -> TokenStream {
    quote! {
        #[no_mangle]
        pub struct CString {}
    }
}

// -----------------
// Str
// -----------------
fn str_struct_declaration() -> TokenStream {
    quote! {
        #[no_mangle]
        pub struct str {}
    }
}

// -----------------
// Isolates
// -----------------
fn init_msg_isolate() -> TokenStream {
    quote! {
        #[no_mangle]
        pub extern "C" fn rid_init_msg_isolate(port: i64) {
            rid::_init_msg_isolate(port)
        }
    }
}

fn init_reply_isolate() -> TokenStream {
    quote! {
        #[no_mangle]
        pub extern "C" fn rid_init_reply_isolate(port: i64) {
            rid::_init_reply_isolate(port)
        }
    }
}
