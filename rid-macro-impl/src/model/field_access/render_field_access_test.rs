use crate::{
    attrs::StructConfig,
    model::field_access::{
        render_dart_field_access::RenderDartFieldAccessConfig,
        render_rust_field_access::RenderRustFieldAccessConfig,
    },
    parse::ParsedStruct,
    rid_export_impl,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

fn render_rust_field_access(input: TokenStream) -> TokenStream {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    let args = syn::AttributeArgs::new();
    match item {
        syn::Item::Struct(struct_item) => {
            let struct_config = StructConfig::from(&struct_item);
            let parsed_struct = ParsedStruct::new(
                &struct_item,
                &struct_item.ident,
                struct_config.clone(),
            );
            parsed_struct
                .render_field_access(
                    &RenderRustFieldAccessConfig::for_rust_tests(),
                    &RenderDartFieldAccessConfig::for_rust_tests(),
                )
                .0
        }
        _ => panic!("Testing struct rendering only"),
    }
}

fn render_dart_field_access(input: TokenStream) -> String {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    let args = syn::AttributeArgs::new();
    match item {
        syn::Item::Struct(struct_item) => {
            let struct_config = StructConfig::from(&struct_item);
            let parsed_struct = ParsedStruct::new(
                &struct_item,
                &struct_item.ident,
                struct_config.clone(),
            );
            parsed_struct
                .render_field_access(
                    &RenderRustFieldAccessConfig::for_dart_tests(),
                    &RenderDartFieldAccessConfig::for_dart_tests(),
                )
                .1
        }
        _ => panic!("Testing struct rendering only"),
    }
}

// -----------------
// Single Field Primitives
// -----------------
mod struct_field_access_single_primitives {
    use crate::common::dump_tokens;

    use super::*;

    // -----------------
    // u8
    // -----------------
    #[test]
    fn primitive_u8_rust() {
        let input: TokenStream = quote! {
            struct MyStruct {
               n: u8
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;
                fn rid_mystruct_n(ptr: *mut MyStruct) -> u8 {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    receiver.n
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn primitive_u8_dart() {
        let input: TokenStream = quote! {
            struct MyStruct {
               n: u8
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  @dart_ffi.Int32()
  int get n { return rid_ffi.rid_mystruct_n(this); }
}
```
 "#;

        let tokens = render_dart_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.trim());
    }

    // -----------------
    // i64
    // -----------------
    #[test]
    fn primitive_i64_rust() {
        let input: TokenStream = quote! {
            struct MyStruct {
               n: i64
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;
                fn rid_mystruct_n(ptr: *mut MyStruct) -> i64 {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    receiver.n
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn primitive_i64_dart() {
        let input: TokenStream = quote! {
            struct MyStruct {
               n: i64
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  @dart_ffi.Int64()
  int get n { return rid_ffi.rid_mystruct_n(this); }
}
```
 "#;

        let tokens = render_dart_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.trim());
    }

    // -----------------
    // bool
    // -----------------
    #[test]
    fn primitive_bool_rust() {
        let input: TokenStream = quote! {
            struct MyStruct {
               n: bool
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;
                fn rid_mystruct_n(ptr: *mut MyStruct) -> u8 {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    if receiver.n {
                        1
                    } else {
                        0
                    }
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn primitive_bool_dart() {
        let input: TokenStream = quote! {
            struct MyStruct {
               n: bool
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  bool get n { return rid_ffi.rid_mystruct_n(this) != 0; }
}
```
 "#;

        let tokens = render_dart_field_access(input);
        eprintln!("{}", tokens);
        assert_eq!(tokens.to_string().trim(), expected.trim());
    }
}

// -----------------
// Single Field Strings
// -----------------
mod struct_field_access_single_strings {
    use crate::common::dump_tokens;

    use super::*;

    // -----------------
    // String
    // -----------------
    #[test]
    fn string_rust() {
        let input: TokenStream = quote! {
            struct MyStruct {
               s: String
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;
                fn rid_mystruct_s(ptr: *mut MyStruct) -> *const ::std::os::raw::c_char {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    let cstring = ::std::ffi::CString::new(receiver.s.as_str())
                        .expect(&format!("Invalid string encountered"));
                    cstring.into_raw()
                }
                fn rid_mystruct_s_len(ptr: *mut MyStruct) -> usize {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    receiver.s.len()
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn string_dart() {
        let input: TokenStream = quote! {
            struct MyStruct {
               s: String
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  String get s {
    dart_ffi.Pointer<dart_ffi.Int8>? ptr = rid_ffi.rid_mystruct_s(this);
    int len = rid_ffi.rid_mystruct_s_len(this);
    String s = ptr.toDartString(len);
    ptr.free();
    return s;
  }
}
```
"#;

        let tokens = render_dart_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.trim());
    }

    // -----------------
    // CString
    // -----------------
    #[test]
    fn cstring_rust() {
        let input: TokenStream = quote! {
            struct MyStruct {
               s: CString
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;

                fn rid_mystruct_s(ptr: *mut MyStruct) -> *const ::std::os::raw::c_char {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    unsafe { &*receiver.s.as_ptr() }
                }
                fn rid_mystruct_s_len(ptr: *mut MyStruct) -> usize {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    receiver.s.as_bytes().len()
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn cstring_dart() {
        let input: TokenStream = quote! {
            struct MyStruct {
               s: String
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  String get s {
    dart_ffi.Pointer<dart_ffi.Int8>? ptr = rid_ffi.rid_mystruct_s(this);
    int len = rid_ffi.rid_mystruct_s_len(this);
    String s = ptr.toDartString(len);
    ptr.free();
    return s;
  }
}
```
"#;

        let tokens = render_dart_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.trim());
    }
}

// -----------------
// Single Field Custom Struct
// -----------------
mod struct_field_access_single_custom_struct {
    use crate::common::dump_tokens;

    use super::*;

    #[test]
    fn todo_struct_rust() {
        let input: TokenStream = quote! {
            #[rid::structs(Todo)]
            struct MyStruct {
               todo: Todo
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;

                fn rid_mystruct_todo(ptr: *mut MyStruct) -> *const Todo {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    &receiver.todo as *const _ as *const Todo
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }

    #[test]
    fn todo_struct_dart() {
        let input: TokenStream = quote! {
            #[rid::structs(Todo)]
            struct MyStruct {
               todo: Todo
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  dart_ffi.Pointer<ffigen_bind.RawTodo> get todo { return rid_ffi.rid_mystruct_todo(this); }
}
```
"#;
        let tokens = render_dart_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.trim());
    }
}

// -----------------
// Single Field Vec<Custom Struct>
// -----------------
mod struct_field_access_single_vec_custom_struct {
    use crate::common::dump_tokens;

    use super::*;

    #[test]
    fn vec_todo_struct_rust() {
        let input: TokenStream = quote! {
            #[rid::structs(Todo)]
            struct MyStruct {
               todos: Vec<Todo>
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;
                fn rid_len_vec_todo(ptr: *mut Vec<Todo>) -> usize {
                    unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut Vec<Todo> = &mut *ptr;
                        ptr.as_mut().expect("resolve_vec_ptr.as_mut failed")
                    }
                    .len()
                }
                fn rid_get_item_vec_todo(ptr: *mut Vec<Todo>, idx: usize) -> *const Todo {
                    let item = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut Vec<Todo> = &mut *ptr;
                        ptr.as_mut().expect("resolve_vec_ptr.as_mut failed")
                    }
                    .get(idx)
                    .expect(&format!(
                        "Failed to access {fn_get_ident}({idx})",
                        fn_get_ident = "rid_get_item_vec_todo",
                        idx = idx
                    ));
                    item as *const Todo
                }
                fn rid_mystruct_todos(ptr: *mut MyStruct) -> *const Vec<Todo> {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    &receiver.todos as *const _ as *const Vec<Todo>
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }
}

// -----------------
// Single Field Vec<u8>
// -----------------
mod struct_field_access_single_vec_u8 {
    use crate::common::dump_tokens;

    use super::*;

    #[test]
    fn vec_todo_struct_rust() {
        let input: TokenStream = quote! {
            struct MyStruct {
               todos: Vec<u8>
            }
        };

        let expected = quote! {
            mod __MyStruct_field_access {
                use super::*;
                fn rid_len_vec_u8(ptr: *mut Vec<u8>) -> usize {
                    unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut Vec<u8> = &mut *ptr;
                        ptr.as_mut().expect("resolve_vec_ptr.as_mut failed")
                    }
                    .len()
                }
                fn rid_get_item_vec_u8(ptr: *mut Vec<u8>, idx: usize) -> u8 {
                    let item = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut Vec<u8> = &mut *ptr;
                        ptr.as_mut().expect("resolve_vec_ptr.as_mut failed")
                    }
                    .get(idx)
                    .expect(&format!(
                        "Failed to access {fn_get_ident}({idx})",
                        fn_get_ident = "rid_get_item_vec_u8",
                        idx = idx
                    ));
                    *item
                }
                fn rid_mystruct_todos(ptr: *mut MyStruct) -> *const Vec<u8> {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    &receiver.todos as *const _ as *const Vec<u8>
                }
            }
        };

        let tokens = render_rust_field_access(input);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }
}
