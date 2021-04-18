use super::{render_function_export, vec::*};

use proc_macro2::TokenStream;
use quote::quote_spanned;
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};
use syn::Ident;

use crate::{
    parse::{ParsedFunction, ParsedImplBlock},
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig,
    },
};

const INDENT: &str = "  ";

pub fn render_instance_method_extension(
    impl_block: &ParsedImplBlock,
    config: Option<RenderFunctionExportConfig>,
) -> TokenStream {
    let config = config.unwrap_or(Default::default());

    let dart_instance_method_strings = &impl_block
        .methods
        .iter()
        .filter(|x| x.receiver.is_some())
        .map(|x| {
            render_function_export(
                x,
                Some(impl_block.ty.ident.clone()),
                INDENT,
                None,
            )
        })
        .collect::<Vec<String>>();

    if !dart_instance_method_strings.is_empty() {
        let comment = if config.comment_dart_code { "/// " } else { "" };

        let instance_methods = dart_instance_method_strings.join("\n");

        let extension_str = format!(
        r###"
{comment}FFI methods generated for exported instance impl methods of struct '{struct_ident}'.
{comment}
{comment}Below is the dart extension to call those methods.
{comment}
{comment}```dart
{comment}extension Rid_ImplInstanceMethods_ExtOnPointer{struct_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{
{instance_methods}
{comment}}}
{comment}```
"###,
        comment = comment,
        struct_ident = impl_block.ty.ident,
        dart_ffi = DART_FFI,
        ffigen_bind = FFI_GEN_BIND,
        instance_methods = instance_methods,
    ).trim().to_string();

        extension_str.parse().unwrap()
    } else {
        TokenStream::new()
    }
}
