use crate::{common::state::get_state, rid_export_impl};
use proc_macro2::TokenStream;
use quote::quote;

use crate::export::ExportConfig;

// -----------------
// Note these are just a few high level integration tests to see that all comes together.
// More detailed tests with arg/return combinations are tested inside ../render_rust/render_function_export_test.rs
// -----------------

fn render(input: TokenStream, config: ExportConfig) -> TokenStream {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    let args = syn::AttributeArgs::new();
    rid_export_impl(item, args, config)
}

fn render_export(input: TokenStream) -> TokenStream {
    render(input, ExportConfig::for_tests())
}

fn render_export_full(input: TokenStream) -> TokenStream {
    render(input, ExportConfig::default())
}

// -----------------
// Struct impl methods
// -----------------
mod struct_impl_methods {
    use crate::common::dump_tokens;

    use super::*;

    // -----------------
    // Returning Self
    // -----------------
    #[test]
    fn no_args_returning_self() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            impl MyStruct {
                #[rid::export]
                pub fn new() -> Self { todo!() }
            }
        };

        let expected = quote! {
            #[allow(non_snake_case)]
            mod __rid_MyStruct_impl_1 {
                use super::*;
                type PointerMut_Self = *mut Self;
                fn rid_export_MyStruct_new() -> PointerMut_Self {
                    let ret = MyStruct::new();
                    let ret_ptr = std::boxed::Box::into_raw(std::boxed::Box::new(ret));
                    ret_ptr
                }
            }
        };

        let tokens = render_export(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    // -----------------
    // Returning Vec
    // -----------------
    #[test]
    fn no_args_returning_vec_u8_ref() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            impl MyStruct {
                #[rid::export]
                fn get_u8s() -> Vec<&u8> {}
            }
        };

        let expected = quote! {
            #[allow(non_snake_case)]
            mod __rid_MyStruct_impl_1 {
                use super::*;
                fn rid_export_MyStruct_get_u8s() -> rid::RidVec<u8> {
                    let ret = MyStruct::get_u8s();
                    let ret: Vec<u8> = ret.into_iter().map(|x| *x).collect();
                    let ret_ptr = rid::RidVec::from(ret);
                    ret_ptr
                }
            }
        };

        let tokens = render_export(input);
        dump_tokens(&tokens);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }
}

// DEBUG: export, use this to debug export issues
mod _debug_export_issues {
    use crate::common::dump_tokens;

    use super::*;

    // #[test]
    fn no_args_returning_vec_u8_ref_full() {
        let _attrs = TokenStream::new();
        let input: TokenStream = quote! {
            #[rid::export]
            impl MyStruct {
                #[rid::export]
                fn get_u8s() -> Vec<&u8> {}
                #[rid::export]
                fn get_u16() -> Vec<&u16> {}
            }
        };

        let tokens = render_export_full(input);
        dump_tokens(&tokens);
    }
}
