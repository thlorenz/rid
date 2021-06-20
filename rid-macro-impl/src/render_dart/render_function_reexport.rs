use heck::MixedCase;
use rid_common::STORE;
use syn::Ident;

use crate::{
    parse::{dart_type::DartType, ParsedFunction},
    render_common::{
        fn_ident_and_impl_ident_string, RenderFunctionExportConfig,
    },
};

use super::DartArg;

impl ParsedFunction {
    /// Renders the store API wrapper for functions rendered on the corresponding Raw Pointer type.
    /// For example renders extensions on `Store` wrapping extensions on `RawStore`.
    ///
    /// Therefore this step is only performed for instance functions and at this point those are
    /// only allowed on the `Store` itself.
    pub fn render_function_reexport(
        &self,
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
            dart_args,
            ..
        } = self;

        let dart_fn_name = fn_ident.to_string().to_mixed_case();

        let raw_fn_ident = fn_ident_alias.as_ref().unwrap_or(fn_ident);
        let return_type = return_arg.render_dart_type(self.type_infos(), false);

        let input_parameters = dart_args
            .iter()
            .map(DartArg::render_typed_parameter)
            .collect::<Vec<String>>()
            .join(", ");

        let params = dart_args
            .iter()
            .map(DartArg::render_parameter)
            .collect::<Vec<String>>();

        let passed_args = params.join(", ");
        let to_string_args = params
            .iter()
            .map(|x| format!("${}", x))
            .collect::<Vec<String>>()
            .join(", ");

        let to_dart = match DartType::from(&return_arg, self.type_infos())
            .render_to_dart()
        {
            Some(to_dart) => format!(".{}", to_dart),
            None => "".to_string(),
        };

        // NOTE: that we depend on the Store `_read` instance method here, if we need this to work
        // on other #[rid::model] instance we need to use `Store.instance.runLocked(...)` directly
        // instead
        format!(
            r###"
{comment}{indent}{return_type} {fn_name}({input_parameters}) => _read(
{comment}{indent}    ({instance}) => {instance}.{raw_fn_name}({passed_args}){toDart}, '{instance}.{fn_name}({to_string_args})');
        "###,
            return_type = return_type,
            fn_name = dart_fn_name,
            raw_fn_name = raw_fn_ident,
            instance = STORE.to_lowercase(),
            input_parameters = input_parameters,
            passed_args = passed_args,
            to_string_args = to_string_args,
            toDart = to_dart,
            comment = comment,
            indent = indent
        )
    }
}
