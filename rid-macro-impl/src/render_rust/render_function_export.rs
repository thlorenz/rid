use std::collections::HashMap;

use super::{
    render_free, render_lifetime, render_lifetime_def, render_return_type,
    ReceiverArg, RenderedReturnType,
};
use crate::{
    accesses::{AccessKind, VecAccess},
    attrs::Category,
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedFunction, ParsedReceiver, ParsedReference,
    },
    render_common::{
        fn_ident_and_impl_ident_string, PointerTypeAlias,
        RenderFunctionExportConfig,
    },
    render_rust::{ffi_prelude, render_rust_arg, RenderedReceiverArgPass},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use render_free::RenderedFree;
use render_rust_arg::RustArg;
use syn::Ident;

pub struct RenderedFunctionExport {
    pub tokens: TokenStream,
    pub ptr_type_aliases: Vec<PointerTypeAlias>,
    pub vec_access: Option<VecAccess>,
}

pub fn render_function_export(
    parsed_function: &ParsedFunction,
    impl_ident: Option<Ident>,
    config: Option<RenderFunctionExportConfig>,
) -> RenderedFunctionExport {
    let config = config.unwrap_or(Default::default());
    let mut ptr_type_aliases = Vec::<PointerTypeAlias>::new();

    let ParsedFunction {
        fn_ident,
        fn_ident_alias,
        receiver,
        args,
        return_arg,
        ..
    } = parsed_function;

    let return_ident = format_ident!("ret");
    let return_pointer_ident = format_ident!("ret_ptr");
    let ffi_prelude = match config.include_ffi {
        true => ffi_prelude(),
        false => TokenStream::new(),
    };

    let (rid_fn_ident, rid_impl_ident_str) =
        fn_ident_and_impl_ident_string(&fn_ident, &impl_ident);

    let rid_export_ident = fn_ident_alias.as_ref().unwrap_or(&rid_fn_ident);

    let static_impl_call_tok = match &impl_ident {
        Some(ident) => quote! { #ident:: },
        None => TokenStream::new(),
    };

    let RenderedReturnType {
        tokens: ret_type,
        type_alias: ret_alias,
    } = render_return_type(return_arg, &AccessKind::MethodReturn);
    ret_alias.as_ref().map(|x| ptr_type_aliases.push(x.clone()));

    let ret_to_pointer = return_arg.render_to_return(
        &return_ident,
        &return_pointer_ident,
        false,
    );

    let receiver_arg =
        receiver.as_ref().map(ParsedReceiver::render_receiver_arg);
    let (receiver_arg, receiver_arg_resolve, receiver_ident) =
        match receiver_arg {
            Some(ReceiverArg {
                arg_pass,
                arg_resolve,
                receiver_ident,
            }) => (Some(arg_pass), arg_resolve, Some(receiver_ident)),
            None => (None, TokenStream::new(), None),
        };

    // TODO(thlorenz): Need to collect raw type aliases here as well
    let arg_idents: Vec<RustArg> = args
        .iter()
        .enumerate()
        .map(|(slot, arg)| RustArg::from(&arg, slot))
        .collect();

    let typed_arg_tokens = render_incoming_args(&fn_ident, &arg_idents);

    let fn_call = render_export_call(fn_ident, receiver_ident, &arg_idents);

    let call_args_resolvers_tokens = arg_idents.iter().map(
        |RustArg {
             resolver_tokens, ..
         }| resolver_tokens,
    );

    let (receiver_arg, ptr_type_alias) = match receiver_arg {
        Some(RenderedReceiverArgPass { tokens, type_alias }) => {
            (Some(tokens), type_alias)
        }
        None => (None, None),
    };
    ptr_type_alias.map(|x| ptr_type_aliases.push(x));

    // insert comma after receiver arg
    let receiver_arg = match receiver_arg {
        Some(arg) if arg_idents.is_empty() => quote! { #arg },
        Some(arg) => quote! { #arg, },
        None => TokenStream::new(),
    };

    let fn_export = quote_spanned! { fn_ident.span() =>
        #ffi_prelude
        fn #rid_export_ident(#receiver_arg #(#typed_arg_tokens)*) -> #ret_type {
            #receiver_arg_resolve
            #(#call_args_resolvers_tokens)*
            let #return_ident = #static_impl_call_tok#fn_call;
            #ret_to_pointer
            #return_pointer_ident
        }
    };

    let vec_access = if return_arg.is_vec() {
        let inner_return_ty = return_arg.inner_composite_type().unwrap();

        let ret_ident = match &ret_alias {
            Some(PointerTypeAlias { alias, .. }) => alias.clone(),
            None if inner_return_ty.is_primitive() => {
                inner_return_ty.rust_ident().clone()
            }
            None if inner_return_ty.is_enum() => {
                format_ident!("i32")
            }
            None => return_arg.rust_ident().clone(),
        };

        Some(VecAccess::new(
            &return_arg,
            ret_ident,
            AccessKind::MethodReturn,
            &ffi_prelude,
        ))
    } else {
        None
    };

    RenderedFunctionExport {
        tokens: fn_export,
        ptr_type_aliases,
        vec_access,
    }
}

// -----------------
// Taking in function parameters
// -----------------
fn render_incoming_args(
    fn_ident: &Ident,
    arg_idents: &[RustArg],
) -> Vec<TokenStream> {
    if arg_idents.is_empty() {
        vec![]
    } else {
        let last_slot = arg_idents.len() - 1;
        arg_idents
            .iter()
            .enumerate()
            .map(|(slot, x)| {
                x.render_typed_parameter(
                    Some(fn_ident.span()),
                    false,
                    slot != last_slot,
                )
            })
            .collect()
    }
}

// -----------------
// Calling exported Function
// -----------------
fn render_export_call(
    fn_ident: &Ident,
    receiver_ident: Option<Ident>,
    arg_idents: &[RustArg],
) -> TokenStream {
    let arg_idents = if arg_idents.is_empty() {
        vec![]
    } else {
        let last_slot = arg_idents.len() - 1;
        arg_idents
            .iter()
            .enumerate()
            .map(|(slot, x)| {
                let ident = &x.arg_ident;
                if slot == last_slot {
                    quote! { #ident }
                } else {
                    quote! { #ident, }
                }
            })
            .collect()
    };

    match receiver_ident {
        Some(receiver_ident) => {
            if arg_idents.is_empty() {
                quote_spanned! { fn_ident.span() => #fn_ident(#receiver_ident) }
            } else {
                quote_spanned! { fn_ident.span() => #fn_ident(#receiver_ident, #(#arg_idents)*) }
            }
        }
        None => {
            quote_spanned! { fn_ident.span() => #fn_ident(#(#arg_idents)*) }
        }
    }
}
