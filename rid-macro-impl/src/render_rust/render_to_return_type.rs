use crate::{
    attrs::Category,
    common::abort,
    parse::{
        rust_type::{Composite, RustType, TypeKind, Value},
        ParsedReference,
    },
};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Ident;

// -----------------
// Render To Return Conversion
// -----------------
impl RustType {
    /// Renders a reassignment from `res_ident` to `res_pointer`, performing any conversions
    /// necessary to match the return type for the specify rust type.
    ///
    /// # Arguments
    /// * `res_ident` the result identifier to possibly convert
    /// * `res_pointer` the result that will be returned from the function
    pub fn render_to_return(
        &self,
        res_ident: &Ident,
        res_pointer: &Ident,
    ) -> TokenStream {
        use TypeKind as K;
        match &self.kind {
        K::Primitive(_) | K::Unit => quote_spanned! { res_ident.span() => let #res_pointer = #res_ident; } ,
        K::Value(val) => val.render_to_return_type(res_ident, res_pointer, &self.reference),
        K::Composite(Composite::Vec, rust_type) => render_vec_to_return_type(res_ident, res_pointer, rust_type),
        K::Composite(Composite::Option, rust_type) => render_option_to_return_type(res_ident, res_pointer, rust_type),
        K::Composite(_, _) =>  todo!("render_pointer::Composite"),
        K::Unknown => todo!("render_pointer::Unknown - should error here or possibly that validation should happen before hand"),
    }
    }
}

impl Value {
    fn render_to_return_type(
        &self,
        res_ident: &Ident,
        res_pointer: &Ident,
        reference: &ParsedReference,
    ) -> TokenStream {
        use Category as C;
        use Value::*;
        match self {
            CString => {
                quote_spanned! { res_ident.span() => let #res_pointer = #res_ident.into_raw(); }
            }
            String => quote_spanned! { res_ident.span() =>
                let cstring = ::std::ffi::CString::new(#res_ident.as_str())
                    .expect(&format!("Invalid string encountered"));
                let #res_pointer = cstring.into_raw();
            },
            Str => todo!("Value::render_to_return::Str"),
            Custom(type_info, type_name) => match type_info.cat {
                C::Enum => {
                    quote_spanned! { res_ident.span() =>
                        let #res_pointer = #res_ident.clone() as i32;
                    }
                }
                C::Struct => match reference {
                    ParsedReference::Owned => {
                        quote_spanned! { res_ident.span() =>
                            let #res_pointer =
                                std::boxed::Box::into_raw(std::boxed::Box::new(#res_ident));
                        }
                    }
                    ParsedReference::Ref(_) | ParsedReference::RefMut(_) => {
                        quote_spanned! { res_ident.span() =>
                            let #res_pointer = #res_ident;
                        }
                    }
                },
                C::Prim => {
                    quote_spanned! { res_ident.span() => let #res_pointer = #res_ident; }
                }
            },
        }
    }
}

fn render_vec_to_return_type(
    res_ident: &Ident,
    res_pointer: &Ident,
    rust_type: &Option<Box<RustType>>,
) -> TokenStream {
    match rust_type {
        Some(rust_type) => {
            if rust_type.is_primitive() {
                quote_spanned! { res_ident.span() =>
                    let #res_pointer = rid::RidVec::from(#res_ident);
                }
            } else {
                let pointer_type = rust_type.render_pointer_type().tokens;
                quote_spanned! { res_ident.span() =>
                    let vec_with_pointers: Vec<#pointer_type> =
                        #res_ident.into_iter().map(|x| &*x as #pointer_type).collect();
                    let #res_pointer = rid::RidVec::from(vec_with_pointers);
                }
            }
        }
        None => abort!(res_ident, "Vec inner type should be defined"),
    }
}

fn render_option_to_return_type(
    res_ident: &Ident,
    res_pointer: &Ident,
    rust_type: &Option<Box<RustType>>,
) -> TokenStream {
    match rust_type {
        Some(rust_type) => {
            quote_spanned! { res_ident.span() =>
                let #res_pointer = rid::_option_ref_to_pointer(#res_ident);
            }
        }
        None => abort!(res_ident, "Option inner type should be defined"),
    }
}
