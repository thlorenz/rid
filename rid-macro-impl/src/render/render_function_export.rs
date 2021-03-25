use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedFunction,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use render_return_type::RenderedReturnType;
use syn::Ident;
use super::{render_return_type, render_free, render_access_item, render_lifetime, render_lifetime_def};

pub struct RenderFunctionExportConfig {
    pub include_ffi: bool,
    pub include_free: bool,
    pub include_access_item: bool,
}

impl Default for RenderFunctionExportConfig {
    fn default() -> Self {
        Self {
            include_ffi: true,
            include_free: true,
            include_access_item: true,
        }
    }
}

pub fn render_function_export(
    parsed_function: &ParsedFunction,
    impl_ident: Option<Ident>,
    config: Option<RenderFunctionExportConfig>,
) -> TokenStream {
    let config = config.unwrap_or(Default::default());

    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg,
    } = parsed_function;

    let return_ident = format_ident!("ret");
    let return_pointer_ident = format_ident!("ret_ptr");
    let ffi_prelude = match config.include_ffi {
        true => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" 
        },
        false => TokenStream::new()
    };

    let rid_impl_ident_str = match &impl_ident {
        Some(ident) => format!("{}_", ident.to_string()),
        None => "".to_string(),
    };

    let rid_fn_ident =
        format_ident!("rid_export_{}{}", rid_impl_ident_str, fn_ident);

    let static_impl_call_tok = match &impl_ident {
        Some(ident) => quote! { #ident:: },
        None => TokenStream::new(),
    };

    let RenderedReturnType {tokens: ret_type, lifetime }  = render_return_type(return_arg);
    let lifetime_def_tok = render_lifetime_def(lifetime.as_ref());
    let ret_to_pointer = render_to_pointer(&return_ident, &&return_pointer_ident, return_arg);

    let fn_call = render_export_call(fn_ident, args);

    let fn_export = quote_spanned! { fn_ident.span() =>
        #ffi_prelude
        fn #rid_fn_ident#lifetime_def_tok() -> #ret_type {
            let #return_ident = #static_impl_call_tok#fn_call;
            #ret_to_pointer
            #return_pointer_ident
        }
    };
    let fn_free = match config.include_free {
        true => { 
            let fn_free_ident = format_ident!("rid_free_{}{}", rid_impl_ident_str, fn_ident);
            render_free(return_arg, &fn_free_ident, &ffi_prelude)
        }
        false => TokenStream::new(),
    };

    let fn_access = match config.include_access_item {
        true => { 
            let fn_access_ident = format_ident!("rid_acces_item_{}{}", rid_impl_ident_str, fn_ident);
            render_access_item(&return_arg, &fn_access_ident, &ffi_prelude)
        },
        false => TokenStream::new(),
    };

    quote! {
        #fn_export
        #fn_free
        #fn_access
    }
}

// -----------------
// Render To Pointer Conversion
// -----------------
fn render_to_pointer(res_ident: &Ident, res_pointer: &Ident, rust_type: &RustType) -> TokenStream {
    use TypeKind as K;
    // TODO: consider ref
    match &rust_type.kind {
        K::Primitive(_) | K::Unit => quote_spanned! { res_ident.span() => let #res_pointer = #res_ident; } ,
        K::Value(val) => render_value_to_pointer(res_ident, res_pointer, val),
        K::Composite(Composite::Vec, rust_type) => { 
            quote_spanned! { res_ident.span() => let #res_pointer = rid::RidVec::from(#res_ident); }
        },
        K::Composite(_, _) =>  todo!("render_pointer::Composite"),
        K::Unknown => todo!("render_pointer::Unknown - should error here or possibly that validation should happen before hand"),
    }
}

fn render_value_to_pointer(res_ident: &Ident, res_pointer: &Ident, val: &Value) -> TokenStream {
    use Value::*;
    match val {
        CString => {
            quote_spanned! { res_ident.span() => let #res_pointer = #res_ident.into_raw(); }
        }
        String => todo!("render_to_pointer::String"),
        Str => todo!("render_to_pointer::Str"),
        Custom(_, _) => todo!("render_to_pointer::Custom"),
    }
}

// -----------------
// Calling exported Function
// -----------------
fn render_export_call(fn_ident: &Ident, args: &[RustType]) -> TokenStream {
    quote_spanned! { fn_ident.span() => #fn_ident() }
}
