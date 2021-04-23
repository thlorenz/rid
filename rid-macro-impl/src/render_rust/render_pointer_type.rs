use proc_macro2::TokenStream;

use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedReference,
};
use quote::{format_ident, quote, quote_spanned};

use super::TypeAlias;

/// C style pointer type for a given RustType, i.e. `*const Model`
pub struct RenderedPointerType {
    pub tokens: TokenStream,
    pub alias: Option<TypeAlias>,
}
impl RustType {
    /// Renders C style pointer type for a given RustType, i.e. `*const Model`
    pub fn render_pointer_type(&self) -> RenderedPointerType {
        use crate::parse::ParsedReference::*;
        use TypeKind as K;
        let RustType {
            ident,
            kind,
            reference,
        } = self;
        let mut alias = None;

        let tokens = match kind {
            K::Primitive(prim) => prim.render_pointer_type(),
            K::Value(val) => {
                let (al, tokens) = val.render_pointer_type(reference);
                alias = al;
                tokens
            }
            K::Composite(Composite::Vec, rust_type) => {
                // similar to same case in ./render_return_type.rs
                todo!("render_pointer_type::custom_composite::vec")
            }
            K::Composite(composite, rust_type) => {
                todo!("render_pointer_type::custom_composite")
            }
            K::Unit => {
                unimplemented!("render_pointer_type::unit .. need error")
            }
            K::Unknown => todo!("unknown .. need error"),
        };

        RenderedPointerType { tokens, alias }
    }
}

impl Primitive {
    fn render_pointer_type(&self) -> TokenStream {
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
            Bool => quote! { bool },
        }
    }
}

impl Value {
    fn render_pointer_type(
        &self,
        reference: &ParsedReference,
    ) -> (Option<TypeAlias>, TokenStream) {
        use Value as V;

        match self {
            V::CString | V::String | V::Str => {
                (None, quote! { *const ::std::os::raw::c_char })
            }
            Value::Custom(info, name) => {
                let (alias, aliased_tok) =
                    reference.render_pointer(name, false);
                (alias, quote_spanned! { info.key.span() => #aliased_tok })
            }
        }
    }
}
