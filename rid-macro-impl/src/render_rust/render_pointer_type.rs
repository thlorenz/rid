use proc_macro2::TokenStream;

use crate::{
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_common::{AccessKind, PointerTypeAlias},
};
use quote::{format_ident, quote, quote_spanned};

/// C style pointer type for a given RustType, i.e. `*const Model`
pub struct RenderedPointerType {
    pub tokens: TokenStream,
    pub alias: Option<PointerTypeAlias>,
}
impl RustType {
    /// Renders C style pointer type for a given RustType, i.e. `*const Model`
    pub fn render_pointer_type(&self) -> RenderedPointerType {
        use crate::parse::ParsedReference::*;
        use TypeKind as K;
        let mut alias = None;

        let tokens = match &self.kind {
            K::Primitive(prim) => prim.render_pointer_type(),
            K::Value(val) => {
                let (al, tokens) = val.render_pointer_type(&self);
                alias = al;
                tokens
            }
            K::Composite(Composite::Vec, rust_type, _) => {
                // similar to same case in ./render_return_type.rs
                todo!("render_pointer_type::custom_composite::vec")
            }
            K::Composite(Composite::Option, rust_type, _) => {
                // similar to same case in ./render_return_type.rs
                todo!("render_pointer_type::custom_composite::option")
            }
            K::Composite(Composite::HashMap, key_type, val_type) => {
                // similar to same case in ./render_return_type.rs
                todo!("render_pointer_type::custom_composite::hash_map")
            }
            K::Composite(composite, rust_type, _) => {
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
    // NOTE: this is super similar to render_value_return_type() inside
    // src/render_rust/render_return_type.rs:197.
    // There are collection item specific differences in the other, but if it turns out that
    // there is really no difference consider merging this code.
    fn render_pointer_type(
        &self,
        rust_type: &RustType,
    ) -> (Option<PointerTypeAlias>, TokenStream) {
        use Value as V;

        match self {
            V::CString | V::String | V::Str
                if rust_type.reference.is_owned() =>
            {
                (None, quote! { *const ::std::os::raw::c_char })
            }
            V::CString | V::String | V::Str => {
                let type_name = &rust_type.rust_ident().to_string();
                let qualified_type_name =
                    &rust_type.fully_qualified_rust_ident().to_string();
                let (alias, aliased_tok) = rust_type.reference.render_pointer(
                    &type_name,
                    &qualified_type_name,
                    false,
                );
                (
                    alias,
                    quote_spanned! { rust_type.rust_ident().span() => #aliased_tok },
                )
            }
            Value::Custom(info, _) => {
                let type_name = &rust_type.rust_ident().to_string();
                let qualified_type_name =
                    &rust_type.fully_qualified_rust_ident().to_string();
                let (alias, aliased_tok) = rust_type.reference.render_pointer(
                    &type_name,
                    &qualified_type_name,
                    false,
                );
                (alias, quote_spanned! { info.key.span() => #aliased_tok })
            }
        }
    }
}
