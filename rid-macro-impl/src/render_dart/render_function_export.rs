use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Ident;

use crate::{
    parse::ParsedFunction,
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig, VecAccess,
    },
    render_dart::DartArg,
};

pub fn render_function_export(
    parsed_function: &ParsedFunction,
    impl_ident: Option<Ident>,
    indent: &str,
    config: Option<RenderFunctionExportConfig>,
) -> String {
    let config = config.unwrap_or(Default::default());
    let comment = if config.comment_dart_code { "/// " } else { "" };

    let ParsedFunction {
        fn_ident,
        fn_ident_alias,
        receiver,
        args,
        return_arg,
        ..
    } = parsed_function;

    let (rid_fn_ident, ..) =
        fn_ident_and_impl_ident_string(&fn_ident, &impl_ident);

    let return_pointer_type = return_arg.render_dart_pointer_type();

    let dart_args: Vec<DartArg> = args
        .iter()
        .enumerate()
        .map(|(slot, arg)| {
            DartArg::from(arg, parsed_function.type_infos(), slot)
        })
        .collect();

    let fn_body = return_arg.render_dart_function_body(
        &rid_fn_ident,
        receiver,
        &dart_args,
        indent,
        &comment,
    );

    let input_parameters = dart_args
        .iter()
        .map(DartArg::render_typed_parameter)
        .collect::<Vec<String>>()
        .join(", ");

    let export_ident = fn_ident_alias.as_ref().unwrap_or(fn_ident);

    format!(
        r###"
{comment}{indent}{return_pointer_type} {fn_name}({input_parameters}) {{
{fn_body}
{comment}{indent}}}
"###,
        return_pointer_type = return_pointer_type,
        fn_name = export_ident,
        input_parameters = input_parameters,
        fn_body = fn_body,
        comment = comment,
        indent = indent
    )
    .trim()
    .to_string()
}
