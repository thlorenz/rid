use proc_macro2::TokenStream;

use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedReference,
};
use quote::{format_ident, quote, quote_spanned};

//  Rendered RustType, i.e. `&Model`
pub struct RenderedRustType {
    pub tokens: TokenStream,
}
impl RustType {
    /// Renders RustType, i.e. `&Model`
    pub fn render_rust_type(&self) -> RenderedRustType {
        use crate::parse::ParsedReference::*;
        use TypeKind as K;
        let RustType {
            kind, reference, ..
        } = self;

        let tokens = match kind {
            K::Primitive(prim) => prim.render_rust_type(),
            K::Value(val) => val.render_rust_type(reference),
            K::Composite(Composite::Vec, rust_type, _) => {
                // similar to same case in ./render_return_type.rs
                todo!("render_rust_type::custom_composite::vec")
            }
            K::Composite(Composite::Option, rust_type, _) => {
                todo!("render_rust_type::custom_composite::option")
            }
            K::Composite(Composite::HashMap, key_type, val_type) => {
                todo!("render_rust_type::custom_composite::hashmap")
            }
            K::Composite(composite, rust_type, _) => {
                todo!("render_rust_type::custom_composite::{:?}", composite)
            }
            K::Unit => {
                unimplemented!("render_rust_type::unit .. need error")
            }
            K::Unknown => todo!("unknown .. need error"),
        };

        RenderedRustType { tokens }
    }
}

impl Primitive {
    // TODO: same as in ./render_pointer_type.rs
    pub fn render_rust_type(&self) -> TokenStream {
        use Primitive::*;
        match self {
            U8 => quote! { u8 },
            I8 => quote! { i8 },
            U16 => quote! { u16 },
            I16 => quote! { i16 },
            U32 => quote! { u32 },
            I32 => quote! { i32 },
            U64 => quote! { u64 },
            I64 => quote! { i64 },
            USize => quote! { usize },
            Bool => quote! { u8 },
        }
    }
}

impl Value {
    fn render_rust_type(&self, reference: &ParsedReference) -> TokenStream {
        use Value as V;

        match self {
            V::CString => {
                let ref_tok = reference.render();
                quote! { #ref_tok CString }
            }
            V::String => {
                let ref_tok = reference.render();
                quote! { #ref_tok String }
            }
            V::Str => {
                let ref_tok = reference.render();
                quote! { #ref_tok str }
            }
            Value::Custom(info, name) => {
                let ref_tok = reference.render();
                let name_tok: TokenStream = name.parse().unwrap();
                quote_spanned! { info.key.span() => #ref_tok #name_tok }
            }
        }
    }
}
