use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

use crate::{
    common::tokens::{
        resolve_bool_from_u8, resolve_hash_map_msg_ptr, resolve_hash_map_ptr,
        resolve_string_ptr, resolve_vec_msg_ptr, resolve_vec_ptr,
    },
    parse::rust_type::{self, RustType, TypeKind},
    render_rust::render_rust_type,
};

#[derive(Debug)]
pub struct RustArg {
    pub virt: bool,
    pub arg_ident: Ident,
    pub type_tokens: TokenStream,
    pub resolver_tokens: TokenStream,
}

impl RustArg {
    pub fn from(ty: &RustType, slot: usize) -> Vec<Self> {
        use TypeKind::*;
        match &ty.kind {
            // -----------------
            // Primitive
            // -----------------
            Primitive(p) => {
                let arg_ident = format_ident!("arg{}", slot);
                let typ = p.render_rust_type();
                let type_tokens = quote_spanned! { arg_ident.span() => #typ };
                let resolver_tokens = match p {
                    rust_type::Primitive::Bool => {
                        resolve_bool_from_u8(&arg_ident, true)
                    }
                    _ => TokenStream::new(),
                };
                vec![RustArg {
                    virt: false,
                    arg_ident,
                    type_tokens,
                    resolver_tokens,
                }]
            }
            Value(value) => {
                use crate::parse::rust_type::Value::*;
                match value {
                    // -----------------
                    // String
                    // -----------------
                    String => {
                        let arg_ident = format_ident!("arg{}", slot);
                        let type_tokens = quote_spanned! { arg_ident.span() =>  *mut ::std::os::raw::c_char };
                        let resolver_tokens =
                            resolve_string_ptr(&arg_ident, true);
                        vec![RustArg {
                            virt: false,
                            arg_ident,
                            type_tokens,
                            resolver_tokens,
                        }]
                    }
                    // -----------------
                    // Custom
                    // -----------------
                    Custom(_, type_name) => {
                        let arg_ident = format_ident!("arg{}", slot);
                        let typ = format_ident!("{}", type_name);
                        let type_tokens =
                            quote_spanned! { arg_ident.span() => #typ };
                        vec![RustArg {
                            virt: false,
                            arg_ident,
                            type_tokens,
                            resolver_tokens: TokenStream::new(),
                        }]
                    }
                    // -----------------
                    // CString
                    // -----------------
                    CString => todo!("RustArg::from::Value(CString)"),
                    // -----------------
                    // Str
                    // -----------------
                    Str => todo!("RustArg::from::Value(Str)"),
                }
            }
            // -----------------
            // Composite HashMap
            // -----------------
            Composite(rust_type::Composite::HashMap, key_ty, val_ty) => {
                let arg_ident = format_ident!("arg{}", slot);
                let key_ty =
                    key_ty.as_ref().expect("HashMap should have key type");
                let val_ty =
                    val_ty.as_ref().expect("HashMap should have val type");
                let key_ty_ident = key_ty.rust_ident();
                let val_ty_ident = val_ty.rust_ident();
                let type_tokens = quote_spanned! { arg_ident.span() =>
                    *const HashMap<#key_ty_ident, #val_ty_ident>
                };
                let resolver_tokens = resolve_hash_map_msg_ptr(
                    &arg_ident,
                    &key_ty_ident,
                    &val_ty_ident,
                );
                vec![RustArg {
                    virt: false,
                    arg_ident,
                    type_tokens,
                    resolver_tokens,
                }]
            }
            // -----------------
            // Composite Vec
            // -----------------
            Composite(rust_type::Composite::Vec, val_ty, _) => {
                let len_ident = format_ident!("len{}", slot);
                let len_type_tokens = quote_spanned! { len_ident.span() =>
                    usize
                };

                let arg_ident = format_ident!("arg{}", slot);
                let val_ty = val_ty.as_ref().expect("Vec should have a type");
                let val_ty_ident = val_ty.rust_ident();
                let type_tokens = quote_spanned! { arg_ident.span() =>
                    *const #val_ty_ident
                };
                let resolver_tokens =
                    resolve_vec_msg_ptr(&arg_ident, &len_ident, &val_ty_ident);
                vec![
                    RustArg {
                        virt: true,
                        arg_ident: len_ident,
                        type_tokens: len_type_tokens,
                        resolver_tokens: quote! {},
                    },
                    RustArg {
                        virt: false,
                        arg_ident,
                        type_tokens,
                        resolver_tokens,
                    },
                ]
                // todo!("RustArg::from::Vec")
            }
            // -----------------
            // Todos
            // -----------------
            Composite(composite, _, _) => {
                todo!("RustArg::from::{:?}", composite)
            }
            // -----------------
            // Invalid
            // -----------------
            Unit => todo!("RustArg::from::Unit"),
            Unknown => unimplemented!("RustArg::from::Unknown"),
        }
    }

    pub fn render_typed_parameter(
        &self,
        span: Option<Span>,
        leading_comma: bool,
        trailing_comma: bool,
    ) -> TokenStream {
        let RustArg {
            arg_ident,
            type_tokens,
            ..
        } = self;
        let lcomma = if leading_comma {
            quote! { , }
        } else {
            TokenStream::new()
        };
        if trailing_comma {
            match span {
                Some(span) => {
                    quote_spanned! { span => #lcomma #arg_ident: #type_tokens, }
                }
                None => quote! { #arg_ident: #type_tokens, },
            }
        } else {
            match span {
                Some(span) => {
                    quote_spanned! { span => #lcomma #arg_ident: #type_tokens }
                }
                None => quote! { #arg_ident: #type_tokens },
            }
        }
    }
}
