use super::{render_function_export, vec::*};

use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI, STORE};
use syn::Ident;

use crate::{
    common::abort,
    parse::{ParsedFunction, ParsedImplBlock},
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig,
    },
};

const INDENT: &str = "   ";

/// Renders the an extension on the `RawReceiver` to make an instance method accessible via
/// `rawReceiver.method_name` and a memory safe wrapper returning a `toDart` result on  `Receiver`
/// itself as well.
/// At this point the receiver is always the `Store` in order to truly guarantee memory safety for
/// the latter.
pub fn render_instance_method_extension(
    impl_block: &ParsedImplBlock,
    config: Option<RenderFunctionExportConfig>,
) -> TokenStream {
    let config = config.unwrap_or(Default::default());

    struct RenderedInstanceMethods {
        raw: Vec<String>,
        wrapper: Vec<String>,
    }

    let dart_instance_methods = &impl_block
        .methods
        .iter()
        .filter(|x| x.receiver.is_some())
        .fold(
            RenderedInstanceMethods {
                raw: vec![],
                wrapper: vec![],
            },
            |mut acc, x| {
                // This was checked during the parsing phase already
                if x.receiver.as_ref().unwrap().ident().to_string().as_str()
                    != STORE
                {
                    abort!(
                        x.fn_ident,
                        "Can only render instance methods defined on store"
                    );
                }
                acc.raw.push(x.render_function_export(
                    Some(impl_block.ty.rust_ident().clone()),
                    INDENT,
                    None,
                ));
                acc.wrapper.push(x.render_function_reexport(
                    Some(format_ident!(
                        "{}",
                        impl_block.ty.dart_wrapper_rust_string()
                    )),
                    INDENT,
                    None,
                ));
                acc
            },
        );

    if !dart_instance_methods.raw.is_empty() {
        let comment = if config.comment_dart_code { "///" } else { "" };

        let raw_instance_methods = dart_instance_methods.raw.join("\n");
        let wrapper_instance_methods = dart_instance_methods.wrapper.join("\n");

        let extension_str = format!(
        r###"
{comment} ```dart
{comment} // FFI methods generated for exported instance impl methods of struct '{RawStruct}'.
{comment} // Below is the dart extension to call those methods.
{comment}
{comment} extension Rid_ImplInstanceMethods_ExtOnPointer{RawStruct} on {dart_ffi}.Pointer<{ffigen_bind}.{RawStruct}> {{
{raw_instance_methods}
{comment} }}
{comment} // Below are the higher level API wrappers for the same instance method available on {Struct}.
{comment} extension Rid_ImplInstanceMethods_ExtOn{Struct} on {Struct} {{
{wrapper_instance_methods}
{comment} }}
{comment} ```
"###,
        comment = comment,
        RawStruct = impl_block.ty.dart_wrapper_rust_string(),
        Struct = impl_block.ty.rust_ident(),
        dart_ffi = DART_FFI,
        ffigen_bind = FFI_GEN_BIND,
        raw_instance_methods = raw_instance_methods,
        wrapper_instance_methods = wrapper_instance_methods,
    ).trim().to_string();
        extension_str.parse().unwrap()
    } else {
        TokenStream::new()
    }
}
