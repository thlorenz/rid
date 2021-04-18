use super::{
    render_access_item, render_free, render_lifetime, render_lifetime_def,
    render_rust_type, ReceiverArg, RenderedRustType,
};
use crate::{
    attrs::Category,
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedFunction, ParsedReceiver, ParsedReference,
    },
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig,
    },
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

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
        false => TokenStream::new(),
    };

    let (rid_fn_ident, rid_impl_ident_str) =
        fn_ident_and_impl_ident_string(&fn_ident, &impl_ident);

    let static_impl_call_tok = match &impl_ident {
        Some(ident) => quote! { #ident:: },
        None => TokenStream::new(),
    };

    let RenderedRustType {
        tokens: ret_type,
        lifetime,
    } = render_rust_type(return_arg, true);
    let lifetime_def_tok = render_lifetime_def(lifetime.as_ref());
    let ret_to_pointer =
        return_arg.render_to_return(&return_ident, &return_pointer_ident);

    let receiver_arg =
        receiver.as_ref().map(ParsedReceiver::render_receiver_arg);
    let (arg_pass, arg_resolve, receiver_ident) = match receiver_arg {
        Some(ReceiverArg {
            arg_pass,
            arg_resolve,
            receiver_ident,
        }) => (arg_pass, arg_resolve, Some(receiver_ident)),
        None => (TokenStream::new(), TokenStream::new(), None),
    };
    let fn_call = render_export_call(fn_ident, receiver_ident, args);

    let fn_export = quote_spanned! { fn_ident.span() =>
        #ffi_prelude
        fn #rid_fn_ident#lifetime_def_tok(#arg_pass) -> #ret_type {
            #arg_resolve
            let #return_ident = #static_impl_call_tok#fn_call;
            #ret_to_pointer
            #return_pointer_ident
        }
    };
    let fn_free = match config.include_free {
        true => {
            let fn_free_ident =
                format_ident!("rid_free_{}{}", rid_impl_ident_str, fn_ident);
            render_free(return_arg, &fn_free_ident, &ffi_prelude)
        }
        false => TokenStream::new(),
    };

    let fn_access = match config.include_access_item {
        true => {
            let fn_access_ident = format_ident!(
                "rid_acces_item_{}{}",
                rid_impl_ident_str,
                fn_ident
            );
            render_access_item(&return_arg, &fn_access_ident, &ffi_prelude)
        }
        false => TokenStream::new(),
    };

    quote! {
        #fn_export
        #fn_free
        #fn_access
    }
}

// -----------------
// Calling exported Function
// -----------------
fn render_export_call(
    fn_ident: &Ident,
    receiver_ident: Option<syn::Ident>,
    args: &[RustType],
) -> TokenStream {
    // TODO: use args when non-empty
    match receiver_ident {
        Some(receiver_ident) => {
            quote_spanned! { fn_ident.span() => #fn_ident(#receiver_ident) }
        }
        None => quote_spanned! { fn_ident.span() => #fn_ident() },
    }
}
