use super::dart::GetterBody;
use crate::{
    attrs::StructConfig,
    common::{
        errors::type_error,
        parsed_field::ParsedField,
        resolvers::{cstring_free, instance_ident, resolve_ptr, resolve_vec_ptr},
        rust::ValueType,
        state::get_state,
        DartType, RustType,
    },
    templates::vec,
};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Field};

type Tokens = proc_macro2::TokenStream;

#[derive(Debug)]
pub struct ParsedStruct {
    ident: syn::Ident,
    parsed_fields: Vec<ParsedField>,
    method_prefix: String,
    config: StructConfig,
}

impl ParsedStruct {
    pub fn new(ident: syn::Ident, fields: Punctuated<Field, Comma>, config: StructConfig) -> Self {
        let method_prefix = format!("rid_{}", ident.to_string().to_lowercase()).to_string();
        let parsed_fields = parse_fields(fields, &method_prefix);

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
        let intro = format!(
            "/// FFI access methods generated for struct '{}'.\n///\n",
            self.ident
        );
        let dart_header = format!("/// Below is the dart extension to call those methods.\n///");
        let comment_header: Tokens = format!("{}{}", intro, dart_header).parse().unwrap();
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

    fn rust_module(&self, doc_comment: Tokens) -> Tokens {
        let (field_method_tokens, implemented_vecs) = self.rust_field_methods();
        let rust_derive_tokens = self.rust_derive_methods();
        let builtins_comment: Tokens = if implemented_vecs.len() > 0 {
            let builtins_comment_str = implemented_vecs.iter().fold("".to_string(), |acc, v| {
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

        let mod_name = format_ident!("__{}_ffi", self.method_prefix);
        let tokens = quote! {
            mod #mod_name {
                use super::*;

                #doc_comment
                #builtins_comment
                #field_method_tokens
                #rust_derive_tokens
            }
        };

        tokens
    }

    fn rust_derive_methods(&self) -> Tokens {
        let cstring_free_tokens = cstring_free();
        if self.config.debug {
            let struct_ident = &self.ident;
            let struct_instance_ident = instance_ident(&struct_ident);
            let fn_debug_ident = format_ident!("{}_debug", self.method_prefix);
            let fn_debug_pretty_ident = format_ident!("{}_debug_pretty", self.method_prefix);
            let resolve_struct_ptr = resolve_ptr(struct_ident);

            quote_spanned! { struct_ident.span() =>
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_debug_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                    let #struct_instance_ident = #resolve_struct_ptr;
                    let s = format!("{:?}", #struct_instance_ident);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn #fn_debug_pretty_ident(ptr: *mut #struct_ident) -> *const ::std::os::raw::c_char {
                    let #struct_instance_ident = #resolve_struct_ptr;
                    let s = format!("{:#?}", #struct_instance_ident);
                    let cstring = ::std::ffi::CString::new(s.as_str()).unwrap();
                    cstring.into_raw()
                }
                #cstring_free_tokens
            }
        } else {
            cstring_free_tokens
        }
    }

    fn rust_field_methods(&self) -> (Tokens, Vec<vec::ImplementVec>) {
        let mut implemented_vecs: Vec<vec::ImplementVec> = vec![];
        let field_method_tokens: Tokens = self
            .parsed_fields
            .iter()
            .map(|f| {
                let (tokens, mut vecs) = self.rust_field_method(f);
                implemented_vecs.append(&mut vecs);
                tokens
            })
            .collect();
        (field_method_tokens, implemented_vecs)
    }

    fn rust_field_method(&self, field: &ParsedField) -> (Tokens, Vec<vec::ImplementVec>) {
        let field_ident = &field.ident;
        let fn_ident = &field.method_ident;
        let ty = &field.ty;
        let struct_ident = &self.ident;
        let struct_instance_ident = instance_ident(&struct_ident);
        let mut implemented_vecs: Vec<vec::ImplementVec> = vec![];

        let resolve_struct_ptr = resolve_ptr(struct_ident);

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
            Ok(RustType::Value(ValueType::RCustom(_, _))) => {
                quote_spanned! { fn_ident.span() =>
                    #[no_mangle]
                    #[allow(non_snake_case)]
                    pub extern "C" fn #fn_ident(ptr: *mut #struct_ident) -> *const #ty {
                        let #struct_instance_ident = #resolve_struct_ptr;
                        &#struct_instance_ident.#field_ident as *const _ as *const #ty
                    }
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

                let resolve_vec = resolve_vec_ptr(&item_ty);

                let vec_type = format!("Vec_{}", item_ty);
                let dart_item_type = DartType::try_from(&rust_type, &item_ty)
                    .expect("vec item type should be a valid dart type");
                let vec_impl = if get_state().needs_implementation(&vec_type) {
                    implemented_vecs.push(vec::ImplementVec {
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
            Ok(RustType::Unknown) => type_error(&field.ty, &"Unhandled Rust Type".to_string()),
            Err(err) => type_error(&field.ty, err),
        };

        let tokens: Tokens = method.into();
        (tokens, implemented_vecs)
    }

    //
    // Dart Extension
    //

    fn dart_extension(&self) -> Tokens {
        let (dart_field_getters_string, errors) = self.dart_field_getters();
        let dart_derive_methods_string = self.dart_derive_methods();
        let s = format!(
            r###"
/// ```dart
/// extension Rid_Model_ExtOnPointer{struct_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{  {field_getters}
/// {derive_methods}
/// }}
/// ```
        "###,
            struct_ident = self.ident,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            field_getters = dart_field_getters_string,
            derive_methods = dart_derive_methods_string
        );
        let comment: Tokens = s.parse().unwrap();
        quote!(
            #(#errors)*
            #comment
        )
    }

    fn dart_derive_methods(&self) -> String {
        let mut getters = vec![];
        if self.config.debug {
            let ffi_debug_method = format!("{}_debug", self.method_prefix);
            let ffi_debug_pretty_method = format!("{}_debug_pretty", self.method_prefix);
            getters.push(format!(
                r###"
                /// String debug([bool pretty = false]) {{ 
                ///   final ptr = pretty 
                ///     ? {rid_ffi}.{ffi_debug_pretty_method}(this)
                ///     : {rid_ffi}.{ffi_debug_method}(this);
                ///   final s = ptr.toDartString();
                ///   ptr.free();
                ///   return s;
                /// }}
                "###,
                rid_ffi = RID_FFI,
                ffi_debug_method = ffi_debug_method,
                ffi_debug_pretty_method = ffi_debug_pretty_method
            ));
        }
        getters.join("\n")
    }

    fn dart_field_getters(&self) -> (String, Vec<Tokens>) {
        let indent = "  ";
        let fields = self.parsed_fields.iter();
        let mut dart_getters: Vec<String> = vec![];
        let mut errors: Vec<Tokens> = vec![];
        for field in fields {
            match &field.dart_ty {
                Ok(dart_ty) => dart_getters.push(self.dart_field_getter(&field, dart_ty)),
                Err(err) => errors.push(type_error(&field.ty, err)),
            }
        }

        let dart_getters = dart_getters.into_iter().fold("".to_string(), |s, m| {
            format!("{}{}\n{}{}", indent, s, indent, m)
        });
        (dart_getters, errors)
    }

    fn dart_field_getter(&self, field: &ParsedField, dart_ty: &DartType) -> String {
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

fn parse_fields(fields: Punctuated<Field, Comma>, method_prefix: &str) -> Vec<ParsedField> {
    fields
        .into_iter()
        .map(|f| ParsedField::new(f, &method_prefix))
        .collect()
}
