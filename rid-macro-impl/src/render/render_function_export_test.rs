use quote::quote;

use crate::{attrs, parse::ParsedFunction};

use super::render_function_export::{
    render_function_export, RenderFunctionExportConfig,
};

fn parse(input: proc_macro2::TokenStream) -> ParsedFunction {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    match item {
        syn::Item::Fn(syn::ItemFn { attrs, sig, .. }) => {
            let attrs = attrs::parse_rid_attrs(&attrs);
            ParsedFunction::new(sig, &attrs, None)
        }
        _ => panic!("Unexpected item, we're trying to parse functions here"),
    }
}

fn render(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parsed_function = parse(input);
    let config = Some(RenderFunctionExportConfig {
        include_ffi: false,
        include_free: false,
        include_access_item: false,
    });
    render_function_export(&parsed_function, None, config)
}

fn render_full(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parsed_function = parse(input);
    let config = Some(RenderFunctionExportConfig {
        include_ffi: false,
        include_free: true,
        include_access_item: true,
    });
    render_function_export(&parsed_function, None, config)
}

fn render_impl(
    input: proc_macro2::TokenStream,
    impl_ident: Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    let parsed_function = parse(input);
    let config = Some(RenderFunctionExportConfig {
        include_ffi: false,
        include_free: false,
        include_access_item: false,
    });
    render_function_export(&parsed_function, impl_ident, config)
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
                ret
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
                ret
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
                ret = rid::RidVec::from(ret);
                ret
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
                ret = rid::RidVec::from(ret);
                ret
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
                ret = ret.into_raw();
                ret
            }
        };
        assert_eq!(res.to_string(), expected.to_string());
    }
    // TODO: other string types (String, str), at this point even the above is incorrect as we'd
    // need to alloc a Box for that string and provide a method to free it. Possibly we could
    // convert it into a RidVec<u8>.
}
