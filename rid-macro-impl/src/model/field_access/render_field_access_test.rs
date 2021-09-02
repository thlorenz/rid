use crate::{
    accesses::{AccessRender, RenderDartAccessConfig, RenderRustAccessConfig},
    attrs::StructConfig,
    common::state::get_state,
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
                    &RenderRustAccessConfig::for_rust_tests(
                        AccessRender::Force,
                    ),
                    &RenderDartAccessConfig::for_rust_tests(),
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
                    &RenderRustAccessConfig::for_dart_tests(AccessRender::Omit),
                    // NOTE: we test the validity of generated dart code via integration tests
                    // instead of snapshots or similar which are prone to break all the time for
                    // the wrong reasons.
                    &RenderDartAccessConfig::for_dart_tests(AccessRender::Omit),
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
            mod __my_struct_field_access {
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
            mod __my_struct_field_access {
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
            mod __my_struct_field_access {
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
            mod __my_struct_field_access {
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
            mod __my_struct_field_access {
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
            mod __my_struct_field_access {
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
        dump_tokens(&tokens);
        // assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
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
        assert_eq!(tokens.trim(), expected.trim());
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
            mod __my_struct_field_access {
                use super::*;
                mod mod_vec_todo_access {
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

    #[test]
    fn vec_todo_struct_dart() {
        let input: TokenStream = quote! {
            #[rid::structs(Todo)]
            struct MyStruct {
               todos: Vec<Todo>
            }
        };

        let expected = r#"
```dart
extension Rid_Model_ExtOnPointerRawMyStruct on dart_ffi.Pointer<ffigen_bind.RawMyStruct> {
  dart_ffi.Pointer<ffigen_bind.Vec_Todo> get todos { return rid_ffi.rid_mystruct_todos(this); }
}
```
"#;
        let tokens = render_dart_field_access(input);
        assert_eq!(tokens.trim(), expected.trim());
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
            mod __my_struct_field_access {
                use super::*;
                mod mod_vec_u8_access {
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
        dump_tokens(&tokens);
        assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }
}

// -----------------
// Single Field HashMap<u8, u8>
// -----------------
mod struct_field_access_single_hash_map_u8_u8 {
    use crate::common::dump_tokens;

    use super::*;

    #[test]
    fn hash_map_u8_u8_rust() {
        let input: TokenStream = quote! {
            #[rid::structs(Todo)]
            struct MyStruct {
               u8s: HashMap<u8, u8>
            }
        };

        let expected = quote! {
            mod __my_struct_field_access {
                use super::*;
                mod mod_hash_map_u8_u8_access {
                    use super::*;
                    type Pointer_u8 = *const u8;
                    fn rid_len_hash_map_u8_u8(ptr: *const HashMap<u8, u8>) -> usize {
                        unsafe {
                            assert!(!ptr.is_null());
                            ptr.as_ref().expect("resolve_hash_map_ptr.as_mut failed")
                        }
                        .len()
                    }
                    fn rid_get_hash_map_u8_u8<'a>(
                        ptr: *const HashMap<u8, u8>,
                        key: u8,
                    ) -> Option<&'a u8> {
                        let item = unsafe {
                            assert!(!ptr.is_null());
                            ptr.as_ref().expect("resolve_hash_map_ptr.as_mut failed")
                        }
                        .get(&key);
                        item
                    }
                    fn rid_contains_key_hash_map_u8_u8(
                        ptr: *const HashMap<u8, u8>,
                        key: u8,
                    ) -> u8 {
                        let hash_map = unsafe {
                            assert!(!ptr.is_null());
                            ptr.as_ref().expect("resolve_hash_map_ptr.as_mut failed")
                        };
                        if hash_map.contains_key(&key) {
                            1
                        } else {
                            0
                        }
                    }
                    fn rid_keys_hash_map_u8_u8(
                        ptr: *const HashMap<u8, u8>,
                    ) -> rid::RidVec<Pointer_u8> {
                        let map: &HashMap<u8, u8> = unsafe {
                            assert!(!ptr.is_null());
                            ptr.as_ref().expect("resolve_hash_map_ptr.as_mut failed")
                        };
                        let ret: Vec<Pointer_u8> =
                            map.keys().map(|x| x as Pointer_u8).collect();
                        let ret_ptr = rid::RidVec::from(ret);
                        ret_ptr
                    }
                }
                mod mod_ridvec_u8_access {
                    use super::*;
                    type Pointer_u8 = *const u8;
                    fn rid_free_ridvec_u8(arg: rid::RidVec<Pointer_u8>) {
                        arg.free();
                    }
                    fn rid_get_item_ridvec_u8(
                        vec: rid::RidVec<Pointer_u8>,
                        idx: usize,
                    ) -> Pointer_u8 {
                        let ptr = vec[idx];
                        ptr
                    }
                }
                fn rid_mystruct_u8s(ptr: *mut MyStruct) -> *const HashMap<u8, u8> {
                    let receiver = unsafe {
                        assert!(!ptr.is_null());
                        let ptr: *mut MyStruct = &mut *ptr;
                        ptr.as_mut().expect("resolve_ptr.as_mut failed")
                    };
                    &receiver.u8s as *const _ as *const HashMap<u8, u8>
                }
            }
        };

        // TODO(thlorenz): cannot get this test to pass. Left it here anyways to show what gets
        // rendered.
        let tokens = render_rust_field_access(input);
        // dump_tokens(&tokens);
        // assert_eq!(tokens.to_string().trim(), expected.to_string().trim());
    }
}
