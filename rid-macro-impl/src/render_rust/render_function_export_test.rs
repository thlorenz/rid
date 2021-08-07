use std::{collections::HashMap, panic::panic_any};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    attrs::{self, FunctionConfig, TypeInfo, TypeInfoMap},
    common::dump_tokens,
    parse::ParsedFunction,
    render_common::{
        render_vec_accesses, PointerTypeAlias, RenderFunctionExportConfig,
        RenderableAccess, VecAccess,
    },
};

use super::{
    render_function_export::render_function_export, RenderedFunctionExport,
};

fn stringify_type_aliases(type_aliases: &[PointerTypeAlias]) -> String {
    let mut names: Vec<String> =
        type_aliases.iter().map(|x| x.alias.to_string()).collect();
    names.dedup();
    names.sort();
    names.join(", ")
}

fn render_vec_access(vec_access: Option<VecAccess>) -> (TokenStream, String) {
    let type_infos = TypeInfoMap::default();
    match vec_access {
        Some(access) => {
            let rust = access.render_rust().tokens;
            let dart = access.render_dart(&type_infos, "");
            (rust, dart)
        }
        None => (TokenStream::new(), "".to_string()),
    }
}

fn render_frees(
    type_aliases: &[PointerTypeAlias],
) -> Vec<(TokenStream, String)> {
    type_aliases
        .iter()
        .filter(|alias| alias.needs_free)
        .map(|alias| {
            let (_, rust) = alias.render_free(TokenStream::new());
            let dart = "".to_string(); // TODO: alias.render_dart("");
            (rust, dart)
        })
        .collect()
}

fn compare_strings_by_line(s1: &str, s2: &str) {
    let lines1: Vec<&str> = s1.lines().collect();
    let lines2: Vec<&str> = s2.lines().collect();
    if lines1.len() != lines2.len() {
        eprintln!("{}", s1);
    }
    assert_eq!(
        lines1.len(),
        lines2.len(),
        "strings should have same amount of lines"
    );

    for (idx, l1) in lines1.into_iter().enumerate() {
        let l1 = l1.trim();
        let l2 = lines2[idx].trim();
        let error = format!(
            "\nlines differ:\n    \"{line}: {l1}\"\n    \"{line}: {l2}\"\n",
            line = idx + 1,
            l1 = l1,
            l2 = l2
        );
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
            ParsedFunction::new(sig, config, owner)
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
        ptr_type_aliases,
        vec_access,
        ..
    } = render_function_export(&parsed_function, None, config);
    let (rust_vec_access, dart_vec_access) = render_vec_access(vec_access);
    let frees = render_frees(&ptr_type_aliases);
    // TODO: dart strings
    let free_tokens = frees.into_iter().map(|(tokens, _)| tokens);

    let tokens = quote! {
        #tokens
        #rust_vec_access
        #(#free_tokens)*
    };
    RenderedFunctionExportSimplified {
        tokens,
        dart_vec_access,
        type_aliases: stringify_type_aliases(&ptr_type_aliases),
    }
}

#[derive(Debug)]
struct RenderedFunctionExportSimplified {
    tokens: TokenStream,
    dart_vec_access: String,
    type_aliases: String,
}

fn render_impl(
    input: proc_macro2::TokenStream,
    owner: &str,
    full: bool,
) -> RenderedFunctionExportSimplified {
    let config = Some(RenderFunctionExportConfig::bare());
    let type_info = TypeInfo::from((owner, attrs::Category::Struct));
    let mut map = HashMap::new();
    map.insert(owner.to_string(), type_info.clone());
    let parsed_function =
        parse(input, Some((&type_info.key, &TypeInfoMap(map))));

    let RenderedFunctionExport {
        tokens,
        ptr_type_aliases,
        vec_access,
        ..
    } = render_function_export(
        &parsed_function,
        Some(type_info.key.clone()),
        config,
    );
    let (rust_vec_access, dart_vec_access) = render_vec_access(vec_access);
    let free_tokens = if full {
        let frees = render_frees(&ptr_type_aliases);
        // TODO: dart strings
        frees.into_iter().map(|(tokens, _)| tokens).collect()
    } else {
        Vec::new()
    };
    let tokens = quote! {
        #tokens
        #rust_vec_access
        #(#free_tokens)*
    };
    RenderedFunctionExportSimplified {
        tokens,
        dart_vec_access,
        type_aliases: stringify_type_aliases(&ptr_type_aliases),
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
            fn rid_free_ridvec_u8(arg: rid::RidVec<u8>) {
                arg.free();
            }
            fn rid_get_item_ridvec_u8(vec: rid::RidVec<u8>, idx: usize) -> u8 {
                vec[idx]
            }
        };
        assert_eq!(res.tokens.to_string(), expected.to_string());
        assert_eq!(res.type_aliases, "");

        /* TODO: rendering dart access functions for Vec<primitive> is not properly
         * implemented yet. Mainly vec return type is wrong.
         * Same issue affects `rid_free_..|rid_get_item_..` rust function names above
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
            fn rid_free_ridvec_mystruct(arg: rid::RidVec<Pointer_MyStruct>) {
                arg.free();
            }
            fn rid_get_item_ridvec_mystruct(
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
// Impl Instance Methods no args
// -----------------
mod impl_instance_methods {
    use super::*;

    #[test]
    fn no_args_non_mut_receiver() {
        let res = render_impl(
            quote! {
                #[rid::export]
                fn id(&self) -> u32 { self.id }
            },
            "Model",
            false,
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

    /* TODO: Should not export Rust method that returns nothing since that could only be for side
     * effects which should be done via a message.
     * Need to note that clearly in the abort!() message
     */
    /*
    #[test]
    fn no_args_mut_receiver_return_void() {
        let res = render_impl(
            quote! {
                #[rid::export]
                fn inc_id(&mut self) { self.id += 1 }
            },
            "Model",
        );
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
            false,
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

// -----------------
// Impl static Methods no args
// -----------------
mod impl_static_methods_no_args {
    use super::*;
    #[test]
    fn no_args_return_receiver_owned_by_name() {
        let res = render_impl(
            quote! {
                #[rid::export]
                fn new() -> Model { todo!() }
            },
            "Model",
            true,
        );
        let expected = quote! {
            fn rid_export_Model_new() -> PointerMut_Model {
                let ret = Model::new();
                let ret_ptr = std::boxed::Box::into_raw(std::boxed::Box::new(ret));
                ret_ptr
            }
            fn rid_free_Model(ptr: PointerMut_Model) {
                let instance = unsafe {
                    assert!(!ptr.is_null());
                    let ptr: PointerMut_Model = &mut *ptr;
                    let ptr = ptr.as_mut().unwrap();
                    Box::from_raw(ptr)
                };
                drop(instance);
            }
        };

        assert_eq!(res.tokens.to_string(), expected.to_string());
        // TODO: verify rendered dart
    }
}

// -----------------
// Impl static Methods with args
// -----------------
mod impl_static_methods_with_args {
    use super::*;

    #[test]
    fn u8_arg_returning_u8() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            pub fn add_one(n: u8) -> u8 { todo!() }
        };

        let expected = quote! {
            fn rid_export_Model_add_one(arg0: u8) -> u8 {
                let ret = Model::add_one(arg0);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        let res = render_impl(input, "Model", false);

        assert_eq!(res.tokens.to_string(), expected.to_string());
    }

    #[test]
    fn u8_u32_string_args_returning_u8() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            pub fn run(n: u8, x: u32, s: String) -> u8 { todo!() }
        };

        let expected = quote! {
            fn rid_export_Model_run(
                arg0: u8,
                arg1: u32,
                arg2: *mut ::std::os::raw::c_char
            ) -> u8 {
                let arg2 = unsafe { ::std::ffi::CString::from_raw(arg2) }
                    .to_str()
                    .expect("Received String that wasn't valid UTF-8.")
                    .to_string();
                let ret = Model::run(arg0, arg1, arg2);
                let ret_ptr = ret;
                ret_ptr
        }
        };
        let res = render_impl(input, "Model", false);

        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
}
//
// -----------------
// Impl instance Methods with args
// -----------------
mod impl_instance_methods_with_args {
    use super::*;

    #[test]
    fn self_ref_u8_arg_returning_u8() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            pub fn add_one(&self, n: u8) -> u8 { todo!() }
        };

        let expected = quote! {
            fn rid_export_Model_add_one(ptr: Pointer_Model, arg0: u8) -> u8 {
                let receiver: &Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_ref().unwrap()
                };
                let ret = Model::add_one(receiver, arg0);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        let res = render_impl(input, "Model", false);

        assert_eq!(res.tokens.to_string(), expected.to_string());
    }

    #[test]
    fn self_u8_u32_string_args_returning_u8() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            pub fn run(self, n: u8, x: u32, s: String) -> u8 { todo!() }
        };

        let expected = quote! {
            fn rid_export_Model_run(
                ptr: PointerMut_Model,
                arg0: u8,
                arg1: u32,
                arg2: *mut ::std::os::raw::c_char
            ) -> u8 {
                let receiver: Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.unwrap()
                };
                let arg2 = unsafe { ::std::ffi::CString::from_raw(arg2) }
                    .to_str()
                    .expect("Received String that wasn't valid UTF-8.")
                    .to_string();
                let ret = Model::run(receiver, arg0, arg1, arg2);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        let res = render_impl(input, "Model", false);

        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
}

// -----------------
// Standalone Functions Special Returns
// -----------------
mod standalone_special_returns {
    use super::*;

    #[test]
    fn u32_arg_returning_option_ref_todo() {
        let input: TokenStream = quote! {
            #[rid::export]
            #[rid::structs(Todo)]
            fn todo_by_id(id: u32) -> Option<&Todo> {
                todo!()
            }
        };

        let expected = quote! {
            fn rid_export_todo_by_id(arg0: u32) -> Pointer_Todo {
                let ret = todo_by_id(arg0);
                let ret_ptr = rid::_option_ref_to_pointer(ret);
                ret_ptr
            }
        };
        let res = render(input);
        assert_eq!(res.tokens.to_string(), expected.to_string());
    }
}
