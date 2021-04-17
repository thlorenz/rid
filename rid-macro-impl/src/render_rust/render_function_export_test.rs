use std::collections::HashMap;

use attrs::FunctionConfig;
use quote::quote;

use crate::{
    attrs::{self, TypeInfo, TypeInfoMap},
    parse::ParsedFunction,
};

use super::render_function_export::{
    render_function_export, RenderFunctionExportConfig,
};

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

fn render(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parsed_function = parse(input, None);
    let config = Some(RenderFunctionExportConfig {
        include_ffi: false,
        include_free: false,
        include_access_item: false,
    });
    render_function_export(&parsed_function, None, config)
}

fn render_full(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parsed_function = parse(input, None);
    let config = Some(RenderFunctionExportConfig {
        include_ffi: false,
        include_free: true,
        include_access_item: true,
    });
    render_function_export(&parsed_function, None, config)
}

fn render_impl(
    input: proc_macro2::TokenStream,
    owner: &str,
) -> proc_macro2::TokenStream {
    let config = Some(RenderFunctionExportConfig {
        include_ffi: false,
        include_free: false,
        include_access_item: false,
    });
    let type_info = TypeInfo::from((owner, attrs::Category::Struct));
    let mut map = HashMap::new();
    map.insert(owner.to_string(), type_info.clone());
    let parsed_function =
        parse(input, Some((&type_info.key, &TypeInfoMap(map))));

    render_function_export(
        &parsed_function,
        Some(type_info.key.clone()),
        config,
    )
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
        assert_eq!(res.to_string(), expected.to_string());
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
        assert_eq!(res.to_string(), expected.to_string());
    }
}

// -----------------
// No Args Composite Vec Return
// -----------------
mod no_args_composite_vec_return {
    use super::*;

    // fn filtered_todos(&self) -> Vec<&Todo> {
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
        assert_eq!(res.to_string(), expected.to_string());
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
            fn rid_free_me(arg: rid::RidVec<u8>) {
                arg.free();
            }
            fn rid_acces_item_me(vec: rid::RidVec<u8>, idx: usize) -> u8 {
                vec[idx]
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
    }

    #[test]
    fn return_vec_struct_ref() {
        let res = render_full(quote! {
            #[rid::structs(MyStruct)]
            fn filter_items() -> Vec<&MyStruct> {}
        });

        let expected = quote! {
            fn rid_export_filter_items<'a>() -> rid::RidVec<&'a MyStruct> {
                let ret = filter_items();
                let ret_ptr = rid::RidVec::from(ret);
                ret_ptr
            }
            fn rid_free_filter_items<'a>(arg: rid::RidVec<&'a MyStruct>) {
                arg.free();
            }
            fn rid_acces_item_filter_items<'a>(
                vec: rid::RidVec<&'a MyStruct>,
                idx: usize
            ) -> &'a MyStruct {
                vec[idx]
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
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
        assert_eq!(res.to_string(), expected.to_string());
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
            fn rid_export_Model_id(ptr: *const Model) -> u32 {
                let receiver: &Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_ref().unwrap()
                };
                let ret = Model::id(receiver);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
    }

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
            fn rid_export_Model_inc_id(ptr: *mut Model) -> () {
                let receiver: &mut Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_mut().unwrap()
                };
                let ret = Model::inc_id(receiver);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
    }

    struct Item;
    struct Model {
        id: Item,
    }
    impl Model {
        fn first(&self) -> &Item {
            &self.id
        }
    }
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
            fn rid_export_Model_first<'a>(ptr: *const Model) -> &'a Item {
                let receiver: &Model = unsafe {
                    assert!(!ptr.is_null());
                    ptr.as_ref().unwrap()
                };
                let ret = Model::first(receiver);
                let ret_ptr = ret;
                ret_ptr
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
    }
}
