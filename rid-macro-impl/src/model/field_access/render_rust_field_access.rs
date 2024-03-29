use std::{collections::HashMap, ops::Deref};

use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};

use crate::{
    accesses::{
        AccessKind, AccessRender, HashMapAccess, RenderRustAccessConfig,
        RenderableAccess, VecAccess,
    },
    attrs,
    common::{
        abort,
        state::{get_state, ImplementationType},
        tokens::{resolve_hash_map_ptr, resolve_ptr, resolve_vec_ptr},
    },
    parse::{
        rust_type::{self, TypeKind},
        ParsedStruct, ParsedStructField,
    },
    render_dart::vec,
    render_rust::ffi_prelude,
};

pub struct RenderRustFieldAccessResult {
    pub tokens: TokenStream,
    pub collection_accesses: HashMap<String, Box<dyn RenderableAccess>>,
}

impl ParsedStruct {
    pub fn render_rust_fields_access(
        &self,
        config: &RenderRustAccessConfig,
    ) -> RenderRustFieldAccessResult {
        let mut collection_accesses: HashMap<
            String,
            Box<dyn RenderableAccess>,
        > = HashMap::new();

        let tokens: TokenStream = self
            .fields
            .iter()
            .map(|field| {
                let (tokens, access) =
                    self.render_rust_field_access(config, &field);
                if let Some(access) = access {
                    collection_accesses.insert(access.key(), access);
                }
                tokens
            })
            .collect();
        RenderRustFieldAccessResult {
            tokens,
            collection_accesses,
        }
    }

    fn render_rust_field_access(
        &self,
        config: &RenderRustAccessConfig,
        field: &ParsedStructField,
    ) -> (TokenStream, Option<Box<dyn RenderableAccess>>) {
        use TypeKind::*;

        let field_ident = &field.ident;
        let field_ty = &field.rust_type.rust_ident();

        let struct_ident = &self.ident;
        let resolve_receiver = resolve_ptr(struct_ident);

        let fn_ident = &field.method_ident(struct_ident);
        let ffi_prelude = &config.ffi_prelude_tokens;

        let mut collection_access: Option<Box<dyn RenderableAccess>> = None;

        let method = match &field.rust_type.kind {
            // -----------------
            // Primitive
            // -----------------
            Primitive(p) => {
                use rust_type::Primitive::*;
                match p {
                    U8 | I8 | U16 | I16 | U32 | I32 | U64 | I64 | USize => {
                        quote_spanned! { fn_ident.span() =>
                            #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> #field_ty {
                                let receiver = #resolve_receiver;
                                receiver.#field_ident
                            }
                        }
                    }
                    Bool => {
                        quote_spanned! { fn_ident.span() =>
                            #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> u8 {
                                let receiver = #resolve_receiver;
                                if receiver.#field_ident { 1 } else { 0 }
                          }
                        }
                    }
                }
            }
            // -----------------
            // String
            // -----------------
            Value(rust_type::Value::String) => {
                let fn_len_ident = format_ident!("{}_len", fn_ident);

                quote_spanned! { fn_ident.span() =>
                    #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                        let receiver = #resolve_receiver;
                        let cstring = ::std::ffi::CString::new(receiver.#field_ident.as_str())
                            .expect(&format!("Invalid string encountered"));
                        cstring.into_raw()
                    }
                    #ffi_prelude fn #fn_len_ident(ptr: *mut #struct_ident) -> usize {
                        let receiver = #resolve_receiver;
                        receiver.#field_ident.len()
                    }
                }
            }
            // -----------------
            // CString
            // -----------------
            Value(rust_type::Value::CString) => {
                let fn_len_ident = format_ident!("{}_len", fn_ident);

                quote_spanned! { fn_ident.span() =>
                    #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                        let receiver = #resolve_receiver;
                        unsafe { &*receiver.#field_ident.as_ptr() }
                    }
                    #ffi_prelude fn #fn_len_ident(ptr: *mut #struct_ident) -> usize {
                        let receiver = #resolve_receiver;
                        receiver.#field_ident.as_bytes().len()
                    }
                }
            }
            // -----------------
            // Str
            // -----------------
            Value(rust_type::Value::Str) => {
                todo!("model::field_access:Value::Str");
            }

            // -----------------
            // Custom Value
            // -----------------
            Value(rust_type::Value::Custom(info, name)) => {
                use attrs::Category::*;
                match info.cat {
                    // We assume that each enum is #[repr(C)]
                    Enum => quote_spanned! { fn_ident.span() =>
                        #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> i32 {
                            let receiver = #resolve_receiver;
                            receiver.#field_ident._rid_into_discriminant()
                        }
                    },
                    Struct => quote_spanned! { fn_ident.span() =>
                        #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> *const #field_ty {
                            let receiver = #resolve_receiver;
                            &receiver.#field_ident as *const _ as *const #field_ty
                        }
                    },
                    Prim => {
                        todo!("model::field_access:Value::Custom::Prim");
                    }
                }
            }
            // -----------------
            // Vec<T>
            // -----------------
            Composite(rust_type::Composite::Vec, inner_ty, _) => match inner_ty
            {
                Some(item_ty) => {
                    let item_ty_ident = item_ty.rust_ident();
                    let resolve_vec = resolve_vec_ptr(item_ty_ident);
                    let vec_ty = &field.rust_type;

                    // NOTE: that we decide if to actually render the vec inside
                    // ./render_field_access.rs  aggregate_vec_accesses
                    collection_access = Some(Box::new(VecAccess::new(
                        &vec_ty,
                        vec_ty.rust_ident().clone(),
                        AccessKind::FieldReference,
                        &config.ffi_prelude_tokens,
                    )));

                    quote_spanned! { fn_ident.span() =>
                        #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> *const Vec<#item_ty_ident> {
                            let receiver = #resolve_receiver;
                            &receiver.#field_ident as *const _ as *const Vec<#item_ty_ident>
                        }
                    }
                }
                None => {
                    abort!(&fn_ident, "Vec field access should have inner type")
                }
            },
            // -----------------
            // HashMap<K, V>
            // -----------------
            Composite(rust_type::Composite::HashMap, key_ty, val_ty) => {
                match (key_ty, val_ty) {
                    (Some(key_ty), Some(val_ty)) => {
                        let key_ty_ident = key_ty.rust_ident();
                        let val_ty_ident = val_ty.rust_ident();
                        let hash_map_ty = &field.rust_type;

                        collection_access = Some(Box::new(HashMapAccess::new(
                            &hash_map_ty,
                            &hash_map_ty.rust_ident(),
                            AccessKind::FieldReference,
                            &config.ffi_prelude_tokens,
                        )));

                        quote_spanned! { fn_ident.span() =>
                            #ffi_prelude fn #fn_ident(ptr: *mut #struct_ident) -> *const HashMap<#key_ty_ident, #val_ty_ident> {
                                let receiver = #resolve_receiver;
                                &receiver.#field_ident as *const _ as *const HashMap<#key_ty_ident, #val_ty_ident>
                            }
                        }
                    }
                    (_, _) => {
                        abort!(
                            &fn_ident,
                            "HashMap field access should have key and val type"
                        )
                    }
                }
            }
            // -----------------
            // Option<T>
            // -----------------
            Composite(rust_type::Composite::Option, inner, _) => {
                todo!("model::field_access:Composite:Option");
            }
            // -----------------
            // Custom<T>
            // -----------------
            Composite(rust_type::Composite::Custom(_, _), inner, _) => {
                todo!("model::field_access:Composite:Custom");
            }
            Unit => abort!(
                &fn_ident,
                "Accessing fields of type Unit is not supported",
            ),
            Unknown => {
                abort!(&fn_ident, "Cannot access field of unknown Rust type",)
            }
        };

        (method, collection_access)
    }
}
