use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Ident;

use crate::{
    parse::ParsedFunction,
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig, VecAccess,
    },
};

pub fn render_function_export(
    parsed_function: &ParsedFunction,
    impl_ident: Option<Ident>,
    indent: &str,
    config: Option<RenderFunctionExportConfig>,
) -> String {
    let config = config.unwrap_or(Default::default());

    let ParsedFunction {
        fn_ident,
        receiver,
        args,
        return_arg,
    } = parsed_function;

    let (rid_fn_ident, ..) =
        fn_ident_and_impl_ident_string(&fn_ident, &impl_ident);
    let return_pointer_type = return_arg.render_dart_pointer_type();

    let comment = if config.comment_dart_code { "/// " } else { "" };
    let fn_body = return_arg.render_dart_fn_body(
        &rid_fn_ident,
        receiver,
        indent,
        &comment,
    );
    format!(
        r###"
{comment}{indent}{return_pointer_type} {fn_name}() {{
{fn_body}
{comment}{indent}}}
"###,
        return_pointer_type = return_pointer_type,
        fn_name = fn_ident,
        fn_body = fn_body,
        comment = comment,
        indent = indent
    )
    .trim()
    .to_string()
}
