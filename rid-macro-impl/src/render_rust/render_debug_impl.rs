use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::Ident;

use crate::{
    attrs::Category,
    common::{
        abort,
        tokens::{
            instance_ident, resolve_enum_from_int, resolve_ptr,
            ResolvedEnumFromInt,
        },
    },
    parse::rust_type::{RustType, TypeKind, Value},
};

pub struct RenderedDebugImpl {
    pub tokens: TokenStream,
    pub fn_debug_method_ident: Ident,
    pub fn_debug_pretty_method_ident: Ident,
}

// NOTE: this and ./render_display_impl.rs are very similar, may make sense to pull out
// commonalities at some point
impl RustType {
    pub fn render_debug_impl(
        &self,
        enum_variants: &Option<Vec<String>>,
    ) -> RenderedDebugImpl {
        match &self.kind {
            TypeKind::Primitive(ident) => {
                abort!(
                    self.rust_ident(),
                    "Cannot render display impl for Builtin Primitive type"
                )
            }
            TypeKind::Value(value) => {
                match value {
                    Value::CString | Value::String | Value::Str => {
                        abort!(
                        self.rust_ident(),
                        "Cannot render display impl for Builtin String types"
                    )
                    }
                    Value::Custom(type_info, name) => match type_info.cat {
                        // TODO: for now treating all enums for which we implement display as
                        // primitives
                        Category::Enum => {
                            let variants = enum_variants.as_ref().expect("Need to pass enum variants to render its Display");
                            self.render_enum_debug_impl(&variants, true)
                        }
                        Category::Struct => self.render_struct_debug_impl(),
                        Category::Prim => abort!(
                        self.rust_ident(),
                        "Cannot render display impl for Custom Primitive type"
                    ),
                    },
                }
            }
            TypeKind::Composite(_, _) => {
                abort!(
                    self.rust_ident(),
                    "TODO: Cannot yet render display impl for Composite type"
                )
            }
            TypeKind::Unit => {
                abort!(
                    self.rust_ident(),
                    "Cannot render display impl for Unit type"
                )
            }
            TypeKind::Unknown => {
                abort!(
                    self.rust_ident(),
                    "Cannot render display impl for Unknown type"
                )
            }
        }
    }

    fn render_struct_debug_impl(&self) -> RenderedDebugImpl {
        let struct_ident = &self.rust_ident();
        let struct_instance_ident = instance_ident(struct_ident);
        let (fn_debug_method_ident, fn_debug_pretty_method_ident) =
            self.get_fn_debug_idents();

        // TODO: consider using type aliases over `*mut` types via `self.render_pointer_type()`
        let resolve_struct_ptr = resolve_ptr(struct_ident);

        let tokens = quote_spanned! { struct_ident.span() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_debug_method_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                let #struct_instance_ident = #resolve_struct_ptr;
                let s = format!("{:?}", #struct_instance_ident);
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_debug_pretty_method_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                let #struct_instance_ident = #resolve_struct_ptr;
                let s = format!("{:#?}", #struct_instance_ident);
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        };

        RenderedDebugImpl {
            tokens,
            fn_debug_method_ident,
            fn_debug_pretty_method_ident,
        }
    }

    fn render_enum_debug_impl(
        &self,
        variants: &[String],
        is_primitive: bool,
    ) -> RenderedDebugImpl {
        let (fn_debug_method_ident, fn_debug_pretty_method_ident) =
            self.get_fn_debug_idents();
        let enum_ident = &self.rust_ident();

        let tokens = if is_primitive {
            // NOTE: assuming `repr(C)` for primitive enums
            let ResolvedEnumFromInt {
                arg_ident,
                arg_type_ident,
                instance_ident,
                tokens: resolve_enum_arg_tokens,
            } = resolve_enum_from_int(enum_ident, variants);

            quote_spanned! { enum_ident.span() =>
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_debug_method_ident(#arg_ident: #arg_type_ident) -> *const ::std::os::raw::c_char {
                    #resolve_enum_arg_tokens
                    let s = format!("{:?}", #instance_ident);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_debug_pretty_method_ident(#arg_ident: #arg_type_ident) -> *const ::std::os::raw::c_char {
                    #resolve_enum_arg_tokens
                    let s = format!("{:#?}", #instance_ident);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        } else {
            let resolve_enum_ptr = resolve_ptr(enum_ident);
            quote_spanned! { enum_ident.span() =>
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_debug_method_ident(ptr: *mut #enum_ident) -> *const ::std::os::raw::c_char {
                    let instance = #resolve_enum_ptr;
                    let s = format!("{:?}", instance);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_debug_pretty_method_ident(ptr: *mut #enum_ident) -> *const ::std::os::raw::c_char {
                    let instance = #resolve_enum_ptr;
                    let s = format!("{:#?}", instance);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        };

        RenderedDebugImpl {
            tokens,
            fn_debug_method_ident,
            fn_debug_pretty_method_ident,
        }
    }

    fn get_fn_debug_idents(&self) -> (Ident, Ident) {
        let method_prefix =
            format!("rid_{}", self.rust_ident().to_string().to_lowercase())
                .to_string();
        let fn_debug_method_ident = format_ident!("{}_debug", method_prefix);
        let fn_debug_pretty_method_ident =
            format_ident!("{}_debug_pretty", method_prefix);

        (fn_debug_method_ident, fn_debug_pretty_method_ident)
    }
}
