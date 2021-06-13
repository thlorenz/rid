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

// -----------------
// Basic cases
// -----------------
mod impl_method_basic {
    use crate::common::dump_code;

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
dart_ffi.Pointer<ffigen_bind.RawTodo> get_todo() {
  final res = rid_ffi.rid_export_Model_get_todo(this);
  final ret = res;
  return ret;
}
"###
        .trim();
        assert_eq!(res, expected)
    }

    #[test]
    fn u8_arg_non_mut_receiver_return_u8() {
        let res = render_impl(
            quote! {
                #[rid::export]
                fn run(&self, id: u8) -> u8 {}
            },
            "Model",
        );
        let expected = r###"
int run(@dart_ffi.Int32() int arg0) {
  final res = rid_ffi.rid_export_Model_run(this, arg0);
  final ret = res;
  return ret;
}
"###
        .trim();
        assert_eq!(res, expected)
    }
}

// -----------------
// Option return values
// -----------------
mod impl_method_returning_option {
    use crate::common::dump_code;

    use super::*;

    #[test]
    fn u8_arg_non_mut_receiver_return_option_todo_ref() {
        let res = render_impl(
            quote! {
                #[rid::structs(Todo)]
                fn find_todo(&self, id: u8) -> Option<&Todo> { }
            },
            "Model",
        );
        let expected = r###"
dart_ffi.Pointer<ffigen_bind.RawTodo>? find_todo(@dart_ffi.Int32() int arg0) {
  final res = rid_ffi.rid_export_Model_find_todo(this, arg0);
  final ret = res.address == 0x0 ? null : res;
  return ret;
}
"###
        .trim();
        assert_eq!(res, expected);
    }

    // TODO(thlorenz): This currently returns a `dart_ffi.Pointer<ffigen_bind.u32>?`, but
    // should just return a `u32?` since we aren't returning a reference
    // #[test]
    fn u8_arg_return_option_u32() {
        let res = render_impl(
            quote! {
                #[rid::structs(Todo)]
                fn convert(id: u8) -> Option<u32> { }
            },
            "Model",
        );
        let expected = r###"
}
"###
        .trim();
        // assert_eq!(res, expected);
        dump_code(&res);
    }
}
