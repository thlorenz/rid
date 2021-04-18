use super::{
    render_access_item, render_free, render_lifetime, render_lifetime_def,
    render_return_type, ReceiverArg, RenderedReturnType,
};
use crate::{
    attrs::Category,
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedFunction, ParsedReceiver, ParsedReference,
    },
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig, VecAccess,
    },
    render_rust::{RenderedArgsPass, TypeAlias},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use render_access_item::RenderedAccessItem;
use render_free::RenderedFree;
use syn::Ident;

pub struct RenderedFunctionExport {
    pub tokens: TokenStream,
    pub type_aliases: Vec<TypeAlias>,
    pub vec_access: Option<VecAccess>,
}

pub fn render_function_export(
    parsed_function: &ParsedFunction,
    impl_ident: Option<Ident>,
    config: Option<RenderFunctionExportConfig>,
) -> RenderedFunctionExport {
    let config = config.unwrap_or(Default::default());
    let mut type_aliases = Vec::<TypeAlias>::new();

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

    let RenderedReturnType {
        tokens: ret_type,
        type_alias: ret_alias,
    } = render_return_type(return_arg, true);
    ret_alias.clone().map(|x| type_aliases.push(x));

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
        None => (RenderedArgsPass::empty(), TokenStream::new(), None),
    };
    let fn_call = render_export_call(fn_ident, receiver_ident, args);

    let RenderedArgsPass {
        tokens: arg_pass,
        type_alias,
    } = arg_pass;
    type_alias.map(|x| type_aliases.push(x));

    let fn_export = quote_spanned! { fn_ident.span() =>
        #ffi_prelude
        fn #rid_fn_ident(#arg_pass) -> #ret_type {
            #arg_resolve
            let #return_ident = #static_impl_call_tok#fn_call;
            #ret_to_pointer
            #return_pointer_ident
        }
    };

    let vec_access = if return_arg.is_vec() {
        // TODO: does this work when type is not aliased?
        let ret_ident = match &ret_alias {
            Some(TypeAlias { alias, .. }) => alias,
            Option::None => &return_arg.ident,
        };

        let fn_len_ident = format_ident!("rid_len_{}", ret_ident);
        let fn_free_ident = format_ident!("rid_free_{}", ret_ident);
        let fn_get_ident = format_ident!("rid_get_item_{}", ret_ident);

        let item_type = return_arg
            .inner_composite_type()
            .expect("Vec should have inner type");

        Some(VecAccess {
            vec_type: return_arg.clone(),
            vec_type_dart: format!("RidVec_{}", ret_ident),
            item_type,
            rust_ffi_prelude: ffi_prelude,
            fn_len_ident,
            fn_free_ident,
            fn_get_ident,
        })
    } else {
        None
    };

    RenderedFunctionExport {
        tokens: fn_export,
        type_aliases,
        vec_access,
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