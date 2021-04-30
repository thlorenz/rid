use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
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

pub struct RenderedDisplayImpl {
    pub tokens: TokenStream,
    pub fn_display_method_name: String,
}

impl RustType {
    pub fn render_display_impl(
        &self,
        enum_variants: &Option<Vec<String>>,
    ) -> RenderedDisplayImpl {
        match &self.kind {
            TypeKind::Primitive(ident) => {
                abort!(
                    self.ident,
                    "Cannot render display impl for Builtin Primitive type"
                )
            }
            TypeKind::Value(value) => {
                match value {
                    Value::CString | Value::String | Value::Str => {
                        abort!(
                        self.ident,
                        "Cannot render display impl for Builtin String types"
                    )
                    }
                    Value::Custom(type_info, name) => match type_info.cat {
                        // TODO: for now treating all enums for which we implement display as
                        // primitives
                        Category::Enum => {
                            let variants = enum_variants.as_ref().expect("Need to pass enum variants to render its Display");
                            self.render_enum_display_impl(&variants, true)
                        }
                        Category::Struct => self.render_struct_display_impl(),
                        Category::Prim => abort!(
                        self.ident,
                        "Cannot render display impl for Custom Primitive type"
                    ),
                    },
                }
            }
            TypeKind::Composite(_, _) => {
                abort!(
                    self.ident,
                    "TODO: Cannot yet render display impl for Composite type"
                )
            }
            TypeKind::Unit => {
                abort!(self.ident, "Cannot render display impl for Unit type")
            }
            TypeKind::Unknown => {
                abort!(
                    self.ident,
                    "Cannot render display impl for Unknown type"
                )
            }
        }
    }

    fn render_struct_display_impl(&self) -> RenderedDisplayImpl {
        let fn_display_ident = self.get_fn_display_ident();

        let struct_ident = &self.ident;
        // TODO: consider using type aliases over `*mut` types via `self.render_pointer_type()`
        let resolve_struct_ptr = resolve_ptr(struct_ident);

        let tokens = quote_spanned! { struct_ident.span() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_display_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                let instance = #resolve_struct_ptr;
                let s = instance.to_string();
                let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                cstring.into_raw()
            }
        };

        RenderedDisplayImpl {
            tokens,
            fn_display_method_name: fn_display_ident.to_string(),
        }
    }

    fn render_enum_display_impl(
        &self,
        variants: &[String],
        is_primitive: bool,
    ) -> RenderedDisplayImpl {
        let fn_display_ident = self.get_fn_display_ident();
        let enum_ident = &self.ident;

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
                pub extern "C" fn #fn_display_ident(#arg_ident: #arg_type_ident) -> *const ::std::os::raw::c_char {
                    #resolve_enum_arg_tokens
                    let s = #instance_ident.to_string();
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        } else {
            let resolve_enum_ptr = resolve_ptr(enum_ident);
            quote_spanned! { enum_ident.span() =>
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_display_ident(ptr: *mut #enum_ident) -> *const ::std::os::raw::c_char {
                    let instance = #resolve_enum_ptr;
                    let s = instance.to_string();
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
            }
        };

        RenderedDisplayImpl {
            tokens,
            fn_display_method_name: fn_display_ident.to_string(),
        }
    }

    fn get_fn_display_ident(&self) -> Ident {
        let method_prefix =
            format!("rid_{}", self.ident.to_string().to_lowercase())
                .to_string();
        format_ident!("{}_display", method_prefix)
    }
}
