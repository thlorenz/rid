use std::ops::Deref;

use super::dart::GetterBody;
use crate::{
    attrs::{
        self, raw_typedef_ident, Category, StructConfig, TypeInfo, TypeInfoMap,
    },
    common::{
        errors::type_error,
        parsed_field::ParsedField,
        rust::ValueType,
        state::{get_state, ImplementationType},
        tokens::{cstring_free, instance_ident, resolve_ptr, resolve_vec_ptr},
        DartType, RustType,
    },
    parse::{
        rust_type::{self, TypeKind, Value},
        ParsedReference,
    },
    render_dart::vec,
    render_rust::RenderedDebugImpl,
};
use proc_macro2::TokenStream;
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Field, Ident};

type Tokens = proc_macro2::TokenStream;

pub struct RenderedRustImplMethods {
    pub tokens: TokenStream,
    pub rust_type: rust_type::RustType,
    pub fn_debug_method_name: String,
    pub fn_debug_pretty_method_name: String,
}

pub struct RenderedRustModule {
    pub rust_type: rust_type::RustType,
    pub fn_debug_method_name: String,
    pub fn_debug_pretty_method_name: String,
}

// TODO(thlorenz): when we reimplement this we should consider read-locking the store for the
// access + vec methods
#[derive(Debug)]
pub struct ParsedStruct {
    ident: syn::Ident,
    parsed_fields: Vec<ParsedField>,
    method_prefix: String,
    config: StructConfig,
}

impl ParsedStruct {
    pub fn new(
        ident: syn::Ident,
        fields: &Punctuated<Field, Comma>,
        config: StructConfig,
    ) -> Self {
        let method_prefix =
            format!("rid_{}", ident.to_string().to_lowercase()).to_string();
        let parsed_fields =
            parse_fields(fields, &method_prefix, &config.type_infos);

        Self {
            ident,
            parsed_fields,
            method_prefix,
            config,
        }
    }

    pub fn tokens(&self) -> Tokens {
        if self.parsed_fields.is_empty() {
            return Tokens::new();
        }
        self.rust_module()
    }

    //
    // Rust Module
    //

    fn rust_module(&self) -> Tokens {
        //
        // Rust
        //
        let struct_ident = raw_typedef_ident(&self.ident);
        let ident = &self.ident;
        let typedef = quote_spanned! { self.ident.span() => type #struct_ident = #ident; };

        let (field_method_tokens, implemented_vecs, typedefs) =
            self.rust_field_methods(&struct_ident);

        //
        // Dart
        //
        let builtins_comment: Tokens = if implemented_vecs.len() > 0 {
            let builtins_comment_str =
                implemented_vecs.iter().fold("".to_string(), |acc, v| {
                    format!("{}\n{}", acc, vec::render(v))
                });
            format!(
                r###"
///
/// Access methods for Rust Builtin Types required by the below methods.
///
/// ```dart
{}
/// ```"###,
                builtins_comment_str
            )
            .parse()
            .unwrap()
        } else {
            Tokens::new()
        };

        let dart_extension = self.dart_extension(&struct_ident);
        // TODO: use single quote! here
        let intro = format!(
            "/// FFI access methods generated for Rust struct '{}' implemented on '{}'.\n///\n",
            self.ident, struct_ident,
        );
        let dart_header = format!(
            "/// Below is the dart extension to call those methods.\n///"
        );
        let comment_header: Tokens =
            format!("{}{}", intro, dart_header).parse().unwrap();

        //
        // Module combining Dart and Rust
        //
        let mod_name = format_ident!("__{}_ffi", self.method_prefix);
        let tokens = quote! {
            mod #mod_name {
                use super::*;
                #typedef
                #(#typedefs)*

                #comment_header
                #dart_extension

                #builtins_comment
                #field_method_tokens
            }
        };

        tokens
    }

    fn rust_field_methods(
        &self,
        struct_ident: &Ident,
    ) -> (Tokens, Vec<vec::ImplementVecOld>, Vec<TokenStream>) {
        let mut implemented_vecs: Vec<vec::ImplementVecOld> = vec![];
        let mut typedefs: Vec<TokenStream> = vec![];
        let field_method_tokens: Tokens = self
            .parsed_fields
            .iter()
            .map(|f| {
                let (tokens, mut vecs, typedef) =
                    self.rust_field_method(f, struct_ident);
                implemented_vecs.append(&mut vecs);
                typedefs.push(typedef);
                tokens
            })
            .collect();
        (field_method_tokens, implemented_vecs, typedefs)
    }

    fn rust_field_method(
        &self,
        field: &ParsedField,
        struct_ident: &Ident,
    ) -> (Tokens, Vec<vec::ImplementVecOld>, TokenStream) {
        let field_ident = &field.ident;
        let fn_ident = &field.method_ident;
        let ty = &field.ty;

        let struct_instance_ident = instance_ident(&struct_ident);
        let mut implemented_vecs: Vec<vec::ImplementVecOld> = vec![];

        let resolve_struct_ptr = resolve_ptr(struct_ident);

        // HACK: avoiding to rewrite too much code we'll replace anyways
        let mut typedef = TokenStream::new();

        let method = match &field.rust_ty {
            Ok(RustType::Value(ValueType::CString)) => {
                let fn_len_ident = format_ident!("{}_len", fn_ident);
                quote_spanned! { field_ident.span() =>
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        unsafe { &*#struct_instance_ident.#field_ident.as_ptr() }
                    }
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #fn_len_ident(ptr: *mut #struct_ident) -> usize {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        #struct_instance_ident.#field_ident.as_bytes().len()
                    }
                }
            }
            Ok(RustType::Value(ValueType::RString)) => {
                let fn_ident_len = format_ident!("{}_len", fn_ident);
                let cstring_free_tokens = cstring_free();
                quote_spanned! { fn_ident.span() =>
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        let cstring = ::std::ffi::CString::new(#struct_instance_ident.#field_ident.as_str())
                            .expect(&format!("Invalid string encountered"));
                        cstring.into_raw()
                    }
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #fn_ident_len(ptr: *mut #struct_ident) -> usize {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        #struct_instance_ident.#field_ident.len()
                    }
                    #cstring_free_tokens
                }
            }
            Ok(RustType::Value(ValueType::RCustom(info, name))) => {
                use attrs::Category::*;
                match info.cat {
                    // TODO: we are assuming each enum is #[repr(C)]
                    Enum => quote_spanned! { fn_ident.span() =>
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> i32 {
                            let #struct_instance_ident = #resolve_struct_ptr;
                            #struct_instance_ident.#field_ident.clone() as i32
                        }
                    },
                    Struct => {
                        // TODO(thlorenz): quickly hacked in here before to make things work until
                        // it is replaced
                        let raw_ty = format_ident!("Raw{}", info.key);
                        quote_spanned! { fn_ident.span() =>
                            type #raw_ty = #ty;
                            #[no_mangle]
                            #[allow(non_snake_case)]
                            pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const #raw_ty {
                                let #struct_instance_ident = #resolve_struct_ptr;
                                &#struct_instance_ident.#field_ident as *const _ as *const #raw_ty
                            }
                        }
                    }
                    Prim => todo!("parsed_struct:rust_field_method Prim"),
                }
            }
            Ok(RustType::Primitive(p)) => {
                use crate::common::rust::PrimitiveType::*;
                match p {
                    U8 | I8 | U16 | I16 | U32 | I32 | U64 | I64 | USize => {
                        quote_spanned! { fn_ident.span() =>
                            #[no_mangle]
                            #[allow(non_snake_case)]
                            pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> #ty {
                                let #struct_instance_ident = #resolve_struct_ptr;
                                #struct_instance_ident.#field_ident
                            }
                        }
                    }
                    Bool => quote_spanned! { fn_ident.span() =>
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> u8 {
                            let #struct_instance_ident = #resolve_struct_ptr;
                            if #struct_instance_ident.#field_ident { 1 } else { 0 }
                        }
                    },
                }
            }
            Ok(RustType::Value(ValueType::RVec((rust_type, item_ty)))) => {
                let fn_len_ident = format_ident!("rid_vec_{}_len", item_ty);
                let fn_get_ident = format_ident!("rid_vec_{}_get", item_ty);

                let item_ty = match rust_type.deref() {
                    RustType::Value(ty) if ty.is_struct() => {
                        let raw_type = raw_typedef_ident(item_ty);
                        typedef = quote_spanned! { item_ty.span() => type #raw_type = #item_ty; };
                        raw_type
                    }
                    _ => item_ty.clone(),
                };

                let resolve_vec = resolve_vec_ptr(&item_ty);
                let vec_type = format!("Vec_{}", item_ty);

                let dart_item_type = DartType::try_from(&rust_type, &item_ty)
                    .expect("vec item type should be a valid dart type");
                let vec_impl = if get_state().needs_implementation(
                    &ImplementationType::VecAccess,
                    &vec_type,
                ) {
                    implemented_vecs.push(vec::ImplementVecOld {
                        vec_type,
                        dart_item_type,
                        fn_len_ident: fn_len_ident.to_string(),
                        fn_get_ident: fn_get_ident.to_string(),
                    });
                    let len_impl = quote_spanned! { fn_len_ident.span() =>
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "C" fn #fn_len_ident(ptr: *mut Vec<#item_ty>) -> usize {
                            #resolve_vec.len()
                        }
                    };
                    let get_impl = if rust_type.is_primitive() {
                        // TODO: don't hardcode struc and ident in the two below cases
                        quote_spanned! { fn_ident.span() =>
                            #[no_mangle]
                            #[allow(non_snake_case)]
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
                        quote_spanned! { fn_ident.span() =>
                            #[no_mangle]
                            #[allow(non_snake_case)]
                            pub extern "C" fn #fn_get_ident(ptr: *mut Vec<#item_ty>, idx: usize) -> *const #item_ty {
                                let item = #resolve_vec.get(idx)
                                .expect(&format!("Failed to access {struc}.{ident}[{idx}]",
                                    struc = "Model",
                                    ident = "Field",
                                    idx = idx
                                ));
                                item as *const #item_ty
                            }
                        }
                    };
                    quote_spanned! { field_ident.span() =>
                        #len_impl
                        #get_impl
                    }
                } else {
                    Tokens::new()
                };

                quote_spanned! { field_ident.span() =>
                    #[no_mangle]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const Vec<#item_ty> {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        &#struct_instance_ident.#field_ident as *const _ as *const Vec<#item_ty>
                    }
                    #vec_impl
                }
            }
            Ok(RustType::Unit) => {
                type_error(&field.ty, &"Unhandled Rust Type Unit".to_string())
            }
            Ok(RustType::Unknown) => {
                type_error(&field.ty, &"Unhandled Rust Type".to_string())
            }
            Err(err) => type_error(&field.ty, err),
        };

        let tokens: Tokens = method.into();
        (tokens, implemented_vecs, typedef)
    }

    //
    // Dart Extension
    //

    fn dart_extension(&self, struct_ident: &Ident) -> Tokens {
        let (dart_field_getters_string, errors) = self.dart_field_getters();
        let s = format!(
            r###"
/// ```dart
/// extension Rid_Model_ExtOnPointer{struct_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{  {field_getters}
/// }}
/// ```
        "###,
            struct_ident = struct_ident,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            field_getters = dart_field_getters_string,
        );
        let comment: Tokens = s.parse().unwrap();
        quote!(
            #(#errors)*
            #comment
        )
    }

    fn dart_field_getters(&self) -> (String, Vec<Tokens>) {
        let indent = "  ";
        let fields = self.parsed_fields.iter();
        let mut dart_getters: Vec<String> = vec![];
        let mut errors: Vec<Tokens> = vec![];
        for field in fields {
            let is_enum = field.rust_ty.as_ref().map_or(false, |x| x.is_enum());

            match &field.dart_ty {
                Ok(dart_ty) => dart_getters
                    .push(self.dart_field_getter(&field, dart_ty, is_enum)),
                Err(err) => errors.push(type_error(&field.ty, err)),
            }
        }

        let dart_getters =
            dart_getters.into_iter().fold("".to_string(), |s, m| {
                format!("{}{}\n{}{}", indent, s, indent, m)
            });
        (dart_getters, errors)
    }

    fn dart_field_getter(
        &self,
        field: &ParsedField,
        dart_ty: &DartType,
        is_enum: bool,
    ) -> String {
        let return_ty = dart_ty.return_type();

        let attrib = match dart_ty.type_attribute() {
            Some(s) => format!("/// {}\n", s),
            None => "".to_string(),
        };

        let enum_name = if is_enum {
            Some(field.rust_ty.as_ref().unwrap().val_type_name())
        } else {
            None
        };
        let (body, delim) =
            match dart_ty.getter_body(&field.method_ident, &enum_name) {
                GetterBody::Expression(body) => (body, " => "),
                GetterBody::Statement(body) => (body, " "),
            };
        // NOTE: quickly hacked together enum resolution support since this code is going to be
        // replaced anyways
        if is_enum {
            let enum_name = enum_name.as_ref().unwrap();

            format!(
                "{attrib}/// int get {field_ident}{delim}{body}",
                delim = delim,
                attrib = attrib,
                field_ident = &field.ident,
                body = body
            )
        } else {
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
}

fn parse_fields(
    fields: &Punctuated<Field, Comma>,
    method_prefix: &str,
    types: &TypeInfoMap,
) -> Vec<ParsedField> {
    fields
        .into_iter()
        .map(|f| ParsedField::new(f, &method_prefix, types))
        .collect()
}
