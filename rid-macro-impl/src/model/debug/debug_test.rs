use super::{rid_debug_impl, DebugImplConfig};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

fn render(input: proc_macro2::TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::DeriveInput>(input).unwrap();
    rid_debug_impl(&item, DebugImplConfig::for_tests())
}

// TODO: we aren't testing the generated dart code
// TODO: we should not render the #[no_mangle]... preamble during tests
mod enums_debug_impl {
    use super::*;

    #[test]
    fn enum_one_field_debug_impl() {
        let res = render(quote! {
            enum Single {
                First
            }
        });

        let expected = quote! {
            mod __rid_mod_rid_single_debug {
                use super::*;
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn rid_single_debug(n: i32) -> *const ::std::os::raw::c_char {
                    let instance = match n {
                        0 => Single::First,
                        _ => panic!("Not a valid Single value",)
                    };
                    let s = format!("{:?}", instance);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn rid_single_debug_pretty(n: i32) -> *const ::std::os::raw::c_char {
                    let instance = match n {
                        0 => Single::First,
                        _ => panic!("Not a valid Single value",)
                    };
                    let s = format!("{:#?}", instance);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        };

        assert_eq!(res.to_string().trim(), expected.to_string().trim())
    }
}

mod structs_debug_impl {
    use super::*;

    #[test]
    fn struct_one_field_debug_impl() {
        let res = render(quote! {
            struct Single { id: u32 }
        });

        let expected = quote! {
            mod __rid_mod_rid_rawsingle_debug {
                use super::*;
                type RawSingle = Single;
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn rid_rawsingle_debug(ptr: *mut RawSingle) -> *const ::std::os::raw::c_char {
                    let rawsingle = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut RawSingle = &mut *ptr;
                        ptr.as_mut().unwrap()
                    };
                    let s = format!("{:?}", rawsingle);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn rid_rawsingle_debug_pretty(ptr: *mut RawSingle) -> *const ::std::os::raw::c_char {
                    let rawsingle = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut RawSingle = &mut *ptr;
                        ptr.as_mut().unwrap()
                    };
                    let s = format!("{:#?}", rawsingle);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        };

        assert_eq!(res.to_string().trim(), expected.to_string().trim())
    }
}
