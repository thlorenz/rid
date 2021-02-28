use crate::{
    dart::{DartType, GetterBody},
    parsed_field::ParsedField,
    rust::{RustType, ValueType},
    state::get_state,
};
use rid_common::{DART_FFI, FFI_GEN_BIND};

use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Field};

pub(crate) struct ParsedStruct {
    ident: syn::Ident,
    parsed_fields: Vec<ParsedField>,
    method_prefix: String,
}

impl ParsedStruct {
    pub(crate) fn new(ident: syn::Ident, fields: Punctuated<Field, Comma>) -> Self {
        let method_prefix = format!("rid_{}", ident.to_string().to_lowercase()).to_string();
        let parsed_fields = parse_fields(fields, &method_prefix);

        ParsedStruct {
            ident,
            parsed_fields,
            method_prefix,
        }
    }

    pub(crate) fn derive_code(&self) -> proc_macro2::TokenStream {
        if self.parsed_fields.is_empty() {
            return proc_macro2::TokenStream::new();
        }
        let intro = format!(
            "/// FFI access methods generated for struct '{}'.\n///\n",
            self.ident
        );
        let dart_header = format!("/// Below is the dart extension to call those methods.\n///");
        let comment_header: proc_macro2::TokenStream =
            format!("{}{}", intro, dart_header).parse().unwrap();
        let dart_extension = self.dart_extension();
        let comment = quote!(
            #comment_header
            #dart_extension
        );
        self.rust_module(comment)
    }

    //
    // Rust Module
    //

    fn rust_module(&self, doc_comment: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let methods = self.parsed_fields.iter().map(|f| self.rust_method(f));
        let mod_name = format_ident!("__{}_ffi", self.method_prefix);
        quote! {
            mod #mod_name {
                use super::*;
                #doc_comment
                #(#methods)*
            }
        }
    }

    fn rust_method(&self, field: &ParsedField) -> proc_macro2::TokenStream {
        let field_ident = &field.ident;
        let fn_ident = &field.method_ident;
        let ty = &field.ty;
        let struct_ident = &self.ident;
        let struct_instance_ident = format_ident!(
            "{}_get_{}",
            struct_ident.to_string().to_lowercase(),
            field_ident
        );

        let resolve_struct_ptr = resolve_ptr(struct_ident);

        let method = match &field.rust_ty {
            Ok(RustType::Value(ValueType::CString)) => {
                let fn_len_ident = format_ident!("{}_len", fn_ident);
                quote! {
                    #[no_mangle]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        unsafe { &*#struct_instance_ident.#field_ident.as_ptr() }
                    }
                    #[no_mangle]
                    pub extern "C" fn #fn_len_ident(ptr: *mut #struct_ident) -> usize {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        #struct_instance_ident.#field_ident.as_bytes().len()
                    }
                }
            }
            Ok(RustType::Value(ValueType::RString)) => {
                let fn_len_ident = format_ident!("{}_len", fn_ident);
                quote! {
                    #[no_mangle]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        let cstring = std::ffi::CString::new(#struct_instance_ident.#field_ident.clone())
                            .expect(&format!("Invalid string encountered"));
                        unsafe { &*cstring.as_ptr() }
                    }
                    #[no_mangle]
                    pub extern "C" fn #fn_len_ident(ptr: *mut #struct_ident) -> usize {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        #struct_instance_ident.#field_ident.len()
                    }
                }
            }
            Ok(RustType::Primitive(p)) => {
                use crate::rust::PrimitiveType::*;
                match p {
                    U8 | I8 | U16 | I16 | U32 | I32 | U64 | I64 | USize => quote! {
                        #[no_mangle]
                        pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> #ty {
                            let #struct_instance_ident = #resolve_struct_ptr;
                            #struct_instance_ident.#field_ident
                        }
                    },
                    Bool => quote! {
                        #[no_mangle]
                        pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> u8 {
                            let #struct_instance_ident = #resolve_struct_ptr;
                            if #struct_instance_ident.#field_ident { 1 } else { 0 }
                        }
                    },
                }
            }
            Ok(RustType::Value(ValueType::RVec((_, item_ty)))) => {
                let fn_len_ident = format_ident!("rid_vec_{}_len", item_ty);
                let fn_get_ident = format_ident!("rid_vec_{}_get", item_ty);

                let resolve_vec = resolve_vec_ptr(&item_ty);

                let len_impl = if get_state().needs_implementation(&fn_len_ident.to_string()) {
                    quote! {
                        #[no_mangle]
                        pub extern "C" fn #fn_len_ident(ptr: *mut Vec<#item_ty>) -> usize {
                            #resolve_vec.len()
                        }
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };

                // TODO: get(idx) works only for primitive types ATM which are returned directly
                let get_impl = if get_state().needs_implementation(&fn_get_ident.to_string()) {
                    quote! {
                        #[no_mangle]
                        pub extern "C" fn #fn_get_ident(ptr: *mut Vec<#item_ty>, idx: usize) -> #item_ty {
                            let item = #resolve_vec.get(idx)
                            .expect(&format!("Failed to access {struc}.{ident}[{idx}]",
                                struc = "Model",
                                ident = "Field",
                                idx = idx
                            ));
                            *item
                        }
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };

                quote! {
                    #[no_mangle]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const Vec<#item_ty> {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        &#struct_instance_ident.#field_ident as *const _ as *const Vec<#item_ty>
                    }
                    #len_impl
                    #get_impl
                }
            }
            Ok(RustType::Unknown) => type_error(&field.ty, &"Unhandled Rust Type".to_string()),
            Err(err) => type_error(&field.ty, err),
        };
        let token_stream: proc_macro2::TokenStream = method.into();
        token_stream
    }

    //
    // Dart Extension
    //

    fn dart_extension(&self) -> proc_macro2::TokenStream {
        let indent = "  ";
        let fields = self.parsed_fields.iter();
        let mut dart_getters: Vec<String> = vec![];
        let mut errors: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        for field in fields {
            match &field.dart_ty {
                Ok(dart_ty) => dart_getters.push(self.dart_getter(&field, dart_ty)),
                Err(err) => {
                    let error = type_error(&field.ty, err);
                    errors = quote!(
                        #errors
                        #error
                    );
                }
            }
        }

        let dart_getters = dart_getters.into_iter().fold("".to_string(), |s, m| {
            format!("{}{}\n{}{}", indent, s, indent, m)
        });

        let s = format!(
            r###"
/// ```dart
/// extension Rid_ExtOnPointer{struct_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{  {getters}
/// }}
/// ```
        "###,
            struct_ident = self.ident,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            getters = dart_getters
        );
        let comment: proc_macro2::TokenStream = s.parse().unwrap();
        quote!(
            #errors
            #comment
        )
    }

    fn dart_getter(&self, field: &ParsedField, dart_ty: &DartType) -> String {
        let return_ty = dart_ty.return_type();

        let attrib = match dart_ty.type_attribute() {
            Some(s) => format!("/// {}\n", s),
            None => "".to_string(),
        };

        let (body, delim) = match dart_ty.getter_body(&field.method_ident) {
            GetterBody::Expression(body) => (body, " => "),
            GetterBody::Statement(body) => (body, " "),
        };
        format!(
            "{attrib}/// {return_ty} get {field_ident}{delim}{body}",
            delim = delim,
            attrib = attrib,
            return_ty = return_ty,
            field_ident = &field.ident,
            body = body
        )
    }
}

fn resolve_ptr(ty: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut #ty = &mut *ptr;
            ptr.as_mut().unwrap()
        }
    }
}

fn resolve_vec_ptr(ty: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        unsafe {
            assert!(!ptr.is_null());
            let ptr: *mut Vec<#ty> = &mut *ptr;
            ptr.as_mut().unwrap()
        }
    }
}

fn parse_fields(fields: Punctuated<Field, Comma>, method_prefix: &str) -> Vec<ParsedField> {
    fields
        .into_iter()
        .map(|f| ParsedField::new(f, &method_prefix))
        .collect()
}

fn type_error(ty: &syn::Type, err: &String) -> proc_macro2::TokenStream {
    syn::Error::new(ty.span(), err).to_compile_error()
}
