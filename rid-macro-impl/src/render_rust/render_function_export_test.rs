use std::{collections::HashMap, panic::panic_any};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    attrs::{self, FunctionConfig, TypeInfo, TypeInfoMap},
    parse::ParsedFunction,
    render_common::{
        render_vec_accesses, RenderFunctionExportConfig, VecAccess,
    },
};

use super::{
    render_function_export::render_function_export, RenderedFunctionExport,
    TypeAlias,
};

fn stringify_type_aliases(type_aliases: &[TypeAlias]) -> String {
    let mut names: Vec<String> =
        type_aliases.iter().map(|x| x.alias.to_string()).collect();
    names.dedup();
    names.sort();
    names.join(", ")
}

fn render_vec_access(vec_access: Option<VecAccess>) -> (TokenStream, String) {
    match vec_access {
        Some(access) => {
            let rust = access.render_rust().tokens;
            let dart = access.render_dart("");
            (rust, dart)
        }
        None => (TokenStream::new(), "".to_string()),
    }
}

fn compare_strings_by_line(s1: &str, s2: &str) {
    let lines1: Vec<&str> = s1.lines().collect();
    let lines2: Vec<&str> = s2.lines().collect();
    assert_eq!(
        lines1.len(),
        lines2.len(),
        "strints should have same amount of lines"
    );

    for (idx, l1) in lines1.into_iter().enumerate() {
        let l1 = l1.trim();
        let l2 = lines2[idx].trim();
        let error =
            format!("\nlines differ:\n    \"{}\"\n    \"{}\"\n", l1, l2);
        if l1 != l2 {
            panic_any(error);
        }
    }
}

fn parse(
    input: proc_macro2::TokenStream,
    owner: Option<(&syn::Ident, &TypeInfoMap)>,
) -> ParsedFunction {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Fn(syn::ItemFn { attrs, sig, .. }) => {
            let rid_attrs = attrs::parse_rid_attrs(&attrs);
            let config = FunctionConfig::new(&rid_attrs, owner);
            ParsedFunction::new(sig, &config, owner)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

fn render(input: proc_macro2::TokenStream) -> RenderedFunctionExport {
    let parsed_function = parse(input, None);
    let config = Some(RenderFunctionExportConfig::bare());
    render_function_export(&parsed_function, None, config)
}

fn render_full(
    input: proc_macro2::TokenStream,
) -> RenderedFunctionExportSimplified {
    let parsed_function = parse(input, None);
    let config = Some(RenderFunctionExportConfig::bare());
    let RenderedFunctionExport {
        tokens,
        type_aliases,
        vec_access,
    } = render_function_export(&parsed_function, None, config);
    let (rust_vec_access, dart_vec_access) = render_vec_access(vec_access);
    let tokens = quote! {
        #tokens
        #rust_vec_access
    };
    RenderedFunctionExportSimplified {
        tokens,
        dart_vec_access,
        type_aliases: stringify_type_aliases(&type_aliases),
    }
}

struct RenderedFunctionExportSimplified {
    tokens: TokenStream,
    dart_vec_access: String,
    type_aliases: String,
}

fn render_impl(
    input: proc_macro2::TokenStream,
    owner: &str,
) -> RenderedFunctionExportSimplified {
    let config = Some(RenderFunctionExportConfig::bare());
    let type_info = TypeInfo::from((owner, attrs::Category::Struct));
    let mut map = HashMap::new();
    map.insert(owner.to_string(), type_info.clone());
    let parsed_function =
        parse(input, Some((&type_info.key, &TypeInfoMap(map))));

    let RenderedFunctionExport {
        tokens,
        type_aliases,
        vec_access,
    } = render_function_export(
        &parsed_function,
        Some(type_info.key.clone()),
        config,
    );
    let (rust_vec_access, dart_vec_access) = render_vec_access(vec_access);
    let tokens = quote! {
        #tokens
        #rust_vec_access
    };
    RenderedFunctionExportSimplified {
        tokens,
        dart_vec_access,
        type_aliases: stringify_type_aliases(&type_aliases),
    }
}

// -----------------
// No Args Primitive Return
// -----------------
mod no_args_prim_return {
    use super::*;

    #[test]
    fn return_u8() {
        let res = render(quote! {
            fn me() -> u8 {}
        });

        let expected = quote! {
            fn rid_export_me() -> u8 {
                let ret = me();
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
    #[test]
    fn return_i64() {
        let res = render(quote! {
            fn me() -> i64 {}
        });

        let expected = quote! {
            fn rid_export_me() -> i64 {
                let ret = me();
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
}

// -----------------
// No Args Composite Vec Return
// -----------------
mod no_args_composite_vec_return {
    use super::*;

    #[test]
    fn return_vec_u8() {
        let res = render(quote! {
            fn me() -> Vec<u8> {}
        });
        let expected = quote! {
            fn rid_export_me() -> rid::RidVec<u8> {
                let ret = me();
                let ret_ptr = rid::RidVec::from(ret);
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
}

mod no_args_composite_vec_return_full {
    use super::*;

    #[test]
    fn return_vec_u8() {
        let res = render_full(quote! {
            fn me() -> Vec<u8> {}
        });
        let expected = quote! {
            fn rid_export_me() -> rid::RidVec<u8> {
                let ret = me();
                let ret_ptr = rid::RidVec::from(ret);
                ret_ptr
            }
            fn rid_free_Vec(arg: rid::RidVec<u8>) {
                arg.free();
            }
            fn rid_get_item_Vec(vec: rid::RidVec<u8>, idx: usize) -> u8 {
                vec[idx]
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
        assert_eq!(res.type_aliases, "");

        /* TODO: rendering dart access functions for Vec<primitive> is not properly
         * implemented yet. Mainly vec return type is wrong.
            let expected_dart = include_str!(
                "./fixtures/function_export.return_vec_u8.dart.snapshot"
            );
            compare_strings_by_line(&res.dart_vec_access, expected_dart);
        */
    }

    #[test]
    fn return_vec_struct_ref() {
        let res = render_full(quote! {
            #[rid::structs(MyStruct)]
            fn filter_items() -> Vec<&MyStruct> {}
        });

        let expected = quote! {
            fn rid_export_filter_items() -> rid::RidVec<Pointer_MyStruct> {
                let ret = filter_items();
                let vec_with_pointers: Vec<Pointer_MyStruct> =
                    ret.into_iter().map(|x| &*x as Pointer_MyStruct).collect();
                let ret_ptr = rid::RidVec::from(vec_with_pointers);
                ret_ptr
            }
            fn rid_free_Pointer_MyStruct(arg: rid::RidVec<Pointer_MyStruct>) {
                arg.free();
            }
            fn rid_get_item_Pointer_MyStruct(
                vec: rid::RidVec<Pointer_MyStruct>,
                idx: usize
            ) -> Pointer_MyStruct {
                vec[idx]
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
        assert_eq!(res.type_aliases, "Pointer_MyStruct");

        let expected_dart = include_str!(
            "./fixtures/function_export.return_vec_struct_ref.dart.snapshot"
        );
        compare_strings_by_line(&res.dart_vec_access, expected_dart);
    }
}

// -----------------
// No Args String Return
// -----------------
mod no_args_string_return {
    use super::*;

    #[test]
    fn return_cstring() {
        let res = render(quote! {
            fn me() -> CString {}
        });
        let expected = quote! {
            fn rid_export_me() -> *const ::std::os::raw::c_char {
                let ret = me();
                let ret_ptr = ret.into_raw();
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
    // TODO: other string types (String, str), at this point even the above is incorrect as we'd
    // need to alloc a Box for that string and provide a method to free it. Possibly we could
    // convert it into a RidVec<u8>.
}

// -----------------
// Impl Method no args
// -----------------
mod impl_method {
    use super::*;

    #[test]
    fn no_args_non_mut_receiver() {
        let res = render_impl(
            quote! {
                #[rid::export]
                fn id(&self) -> u32 { self.id }
            },
            "Model",
        );
        let expected = quote! {
            fn rid_export_Model_id(ptr: Pointer_Model) -> u32 {
                let receiver: &Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_ref().unwrap()
                };
                let ret = Model::id(receiver);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
        assert_eq!(res.type_aliases, "Pointer_Model");
    }

    /* TODO: Should not export Rust method that returns nothing (at least for now)
    #[test]
    fn no_args_mut_receiver_return_void() {
        let res = render_impl(
            quote! {
                #[rid::export]
                fn inc_id(&mut self) { self.id += 1 }
            },
            "Model",
        );
        let expected = quote! {
            fn rid_export_Model_inc_id(ptr: PointerMut_Model) -> () {
                let receiver: &mut Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_mut().unwrap()
                };
                let ret = Model::inc_id(receiver);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
        assert_eq!(res.type_aliases, "PointerMut_Model");
    }
     */

    #[test]
    fn no_args_non_mut_receiver_return_struct_ref() {
        let res = render_impl(
            quote! {
                #[rid::export]
                #[rid::structs(Item)]
                fn first(&self) -> &Item { &self.id }
            },
            "Model",
        );
        let expected = quote! {
            fn rid_export_Model_first(ptr: Pointer_Model) -> Pointer_Item {
                let receiver: &Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_ref().unwrap()
                };
                let ret = Model::first(receiver);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
        assert_eq!(res.type_aliases, "Pointer_Item, Pointer_Model");
    }
}