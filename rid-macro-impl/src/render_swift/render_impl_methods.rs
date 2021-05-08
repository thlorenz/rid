use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    parse::ParsedImplBlock, render_common::RenderFunctionExportConfig,
};

use super::render_swift_call;

pub fn render_impl_methods(
    impl_ident: &Ident,
    impl_block: &ParsedImplBlock,
    config: Option<RenderFunctionExportConfig>,
) -> TokenStream {
    let config = config.unwrap_or(Default::default());

    let comment = if config.comment_swift_code {
        "/// "
    } else {
        ""
    };

    let swift_method_strings: Vec<String> = impl_block
        .methods
        .iter()
        .map(|x| {
            format!(
                "{comment}    {call};",
                comment = comment,
                call = render_swift_call(
                    &x.fn_ident_alias
                        .as_ref()
                        .unwrap_or(&x.fn_ident)
                        .to_string(),
                    &x.args,
                    x.receiver.is_some()
                )
            )
        })
        .collect();

    if !swift_method_strings.is_empty() {
        let methods = swift_method_strings.join("\n");

        let dummy_methods_str = format!(
        r###"
{comment}Swift methods generated for exported impl methods of struct '{struct_ident}'.
{comment}
{comment}They are necessary to prevent Rust methods being removed via Swift tree shaking.
{comment}
{comment}```swift
{comment}func dummyMethodsFor{impl_ident} {{
{methods}
{comment}}}
{comment}```
"###,
        comment = comment,
        struct_ident = impl_block.ty.ident,
        impl_ident = impl_ident,
        methods = methods,
    ).trim().to_string();

        dummy_methods_str.parse().unwrap()
    } else {
        TokenStream::new()
    }
}
