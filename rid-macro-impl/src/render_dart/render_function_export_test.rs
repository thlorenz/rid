use std::collections::HashMap;

use crate::{
    attrs::{parse_rid_attrs, Category, FunctionConfig, TypeInfo, TypeInfoMap},
    parse::ParsedFunction,
    render_common::RenderFunctionExportConfig,
};
use proc_macro2::TokenStream;
pub use quote::quote;
use syn::Ident;

use super::render_function_export;

fn parse(
    input: TokenStream,
    owner: Option<(&Ident, &TypeInfoMap)>,
) -> ParsedFunction {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Fn(syn::ItemFn { attrs, sig, .. }) => {
            let rid_attrs = parse_rid_attrs(&attrs);
            let config = FunctionConfig::new(&rid_attrs, owner);
            ParsedFunction::new(sig, &config, owner)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

fn render_impl(input: proc_macro2::TokenStream, owner: &str) -> String {
    let type_info = TypeInfo::from((owner, Category::Struct));
    let mut map = HashMap::new();
    map.insert(owner.to_string(), type_info.clone());
    let parsed_function =
        parse(input, Some((&type_info.key, &TypeInfoMap(map))));

    let config = Some(RenderFunctionExportConfig::bare());
    render_function_export(
        &parsed_function,
        Some(type_info.key.clone()),
        "",
        config,
    )
}

mod impl_method {
    use super::*;
    #[test]
    fn no_args_non_mut_receiver_return_struct_ref() {
        let res = render_impl(
            quote! {
                #[rid::export]
                #[rid::structs(Todo)]
                fn get_todo(&self) -> &Todo {
                    &self.todo
                }
            },
            "Model",
        );
        let expected = r###"
dart_ffi.Pointer<ffigen_bind.Todo> get_todo() {
  return rid_ffi.rid_export_Model_get_todo(this);
}
"###
        .trim();
        assert_eq!(res, expected)
    }
}
