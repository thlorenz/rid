use crate::{
    attrs::Category,
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
        K::Value(val) => val.render_to_return(res_ident, res_pointer, &self.reference),
        K::Composite(Composite::Vec, rust_type) => {
            quote_spanned! { res_ident.span() => let #res_pointer = rid::RidVec::from(#res_ident); }
        },
        K::Composite(_, _) =>  todo!("render_pointer::Composite"),
        K::Unknown => todo!("render_pointer::Unknown - should error here or possibly that validation should happen before hand"),
    }
    }
}

impl Value {
    fn render_to_return(
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
            String => todo!("Value::render_to_return::String"),
            Str => todo!("Value::render_to_return::Str"),
            Custom(type_info, type_name) => match type_info.cat {
                C::Enum => {
                    quote_spanned! { res_ident.span() =>
                        let #res_pointer = #res_ident.clone() as i32;
                    }
                }
                C::Struct => {
                    quote_spanned! { res_ident.span() =>
                        let #res_pointer = #res_ident;
                    }
                }
                C::Prim => {
                    quote_spanned! { res_ident.span() => let #res_pointer = #res_ident; }
                }
            },
        }
    }
}
