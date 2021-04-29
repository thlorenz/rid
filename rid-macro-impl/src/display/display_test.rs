use super::{rid_display_impl, DisplayImplConfig};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

fn remove_doc_comments(tokens: TokenStream) -> TokenStream {
    let code = tokens.to_string();
    let lines = code.split("\"]");
    let without_docs: Vec<&str> = lines
        .into_iter()
        .filter(|x| !x.contains("# [doc ="))
        .collect();
    without_docs.join("\n").parse().unwrap()
}

fn render(input: proc_macro2::TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::DeriveInput>(input).unwrap();
    remove_doc_comments(rid_display_impl(&item, DisplayImplConfig::for_tests()))
}

mod enums_display_impl {
    use super::*;

    #[test]
    fn enum_one_field_display_impl() {
        let res = render(quote! {
            enum Single {
                First
            }
        });

        let expected = quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn rid_single_display(n: i32) -> *const ::std::os::raw::c_char {
                let instance = match n {
                    0 => Single::First,
                    _ => panic!("Not a valid Single value",)
                };
                let s = instance.to_string();
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        };

        assert_eq!(res.to_string().trim(), expected.to_string().trim())
    }

    #[test]
    fn enum_three_fields_display_impl() {
        let res = render(quote! {
            enum Single {
                First,
                Second,
                Third,
            }
        });

        let expected = quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn rid_single_display(n: i32) -> *const ::std::os::raw::c_char {
                let instance = match n {
                    0 => Single::First,
                    1 => Single::Second,
                    2 => Single::Third,
                    _ => panic!("Not a valid Single value",)
                };
                let s = instance.to_string();
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        };

        assert_eq!(res.to_string().trim(), expected.to_string().trim())
    }
}

mod structs_display_impl {
    use super::*;

    #[test]
    fn struct_one_field_display_impl() {
        let res = render(quote! {
            struct Single { id: u32 }
        });

        let expected = quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn rid_single_display(ptr: *mut Single) -> *const ::std::os::raw::c_char {
                let instance = unsafe {
                    assert!(!ptr.is_null());
                    let ptr: *mut Single = &mut *ptr;
                    ptr.as_mut().unwrap()
                };
                let s = instance.to_string();
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        };

        assert_eq!(res.to_string().trim(), expected.to_string().trim())
    }
}