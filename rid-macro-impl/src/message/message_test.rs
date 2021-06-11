use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, Item};

use crate::{
    attrs::{self, EnumConfig, RidAttr},
    common::abort,
    message::ParsedEnum,
};

use super::render_message_enum::MessageRenderConfig;

fn render(
    input: TokenStream,
    config: &MessageRenderConfig,
) -> (TokenStream, String) {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    let rid_args: Vec<Ident> =
        vec![format_ident!("Store"), format_ident!("Reply")];
    let rid_attrs: Vec<RidAttr> = vec![];
    match item {
        Item::Enum(item) => {
            let rid_attrs = attrs::parse_rid_attrs(&item.attrs);
            let enum_config =
                EnumConfig::new(&rid_attrs, &rid_args[0], &rid_args[1]);
            let parsed_enum = ParsedEnum::new(
                &item.ident,
                item.variants.clone(),
                enum_config,
            );
            parsed_enum.render(&config)
        }
        _ => {
            abort!(item, "rid::message attribute can only be applied to enums")
        }
    }
}

fn render_rust(input: &TokenStream) -> TokenStream {
    render(
        input.clone(),
        &MessageRenderConfig {
            rust_only: true,
            ..MessageRenderConfig::bare()
        },
    )
    .0
}

fn render_dart(input: &TokenStream) -> String {
    render(
        input.clone(),
        &MessageRenderConfig {
            dart_code_only: true,
            ..MessageRenderConfig::bare()
        },
    )
    .1
}

// -----------------
// Messages without Fields
// -----------------
mod msg_variants_without_fields {
    use crate::common::{dump_code, dump_tokens};

    use super::*;
    #[test]
    fn msg_init() {
        let msg = quote! {
            pub enum Msg {
                Init,
            }
        };

        //        let expected_rust =
        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        dump_tokens(rust);
        dump_code(&dart);
    }
}
