use crate::{common::extract_variant_names, parse::rust_type::RustType};

use super::{render_debug, RenderDebugConfig};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

// TODO: we aren't testing the generated dart code
// TODO: we should not render the #[no_mangle]... preamble during tests

fn render_struct_or_enum_derive(
    input: &DeriveInput,
    config: RenderDebugConfig,
) -> TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let rust_type = RustType::from_owned_struct(&input.ident);
            render_debug(rust_type, &None, config.clone())
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let rust_type = RustType::from_owned_enum(&input.ident);
            let variants = Some(extract_variant_names(variants));
            render_debug(rust_type, &variants, config.clone())
        }
        Data::Union(data) => {
            panic!("Cannot derive debug for an untagged Union type")
        }
    }
}

fn render(input: proc_macro2::TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::DeriveInput>(input).unwrap();
    render_struct_or_enum_derive(&item, RenderDebugConfig::for_tests())
}

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
            mod __rid_mod_rid_single_debug {
                use super::*;
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn rid_single_debug(ptr: *mut Single) -> *const ::std::os::raw::c_char {
                    let single = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut Single = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    let s = format!("{:?}", single);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn rid_single_debug_pretty(ptr: *mut Single) -> *const ::std::os::raw::c_char {
                    let single = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut Single = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    let s = format!("{:#?}", single);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        };

        assert_eq!(res.to_string().trim(), expected.to_string().trim())
    }
}
