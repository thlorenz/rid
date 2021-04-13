use proc_macro2::TokenStream;

use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedReference,
};
use quote::{format_ident, quote, quote_spanned};

pub struct RenderedPointerType {
    pub tokens: TokenStream,
}
pub fn render_pointer_type(rust_type: &RustType) -> RenderedPointerType {
    use crate::parse::ParsedReference::*;
    use TypeKind as K;
    let RustType {
        ident,
        kind,
        reference,
    } = rust_type;

    let tokens = match kind {
        K::Primitive(prim) => render_primitive_pointer(prim),
        K::Value(val) => render_value_pointer(val, reference),
        K::Composite(Composite::Vec, rust_type) => {
            // similar to same case in ./render_return_type.rs
            todo!("render_pointer_type::custom_composite::vec")
        }
        K::Composite(composite, rust_type) => {
            todo!("render_pointer_type::custom_composite")
        }
        K::Unit => unimplemented!("render_pointer_type::unit .. need error"),
        K::Unknown => todo!("unknown .. need error"),
    };

    RenderedPointerType { tokens }
}

fn render_primitive_pointer(prim: &Primitive) -> TokenStream {
    use Primitive::*;
    match prim {
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

fn render_value_pointer(
    value: &Value,
    reference: &ParsedReference,
) -> TokenStream {
    use Value as V;

    match value {
        V::CString | V::String | V::Str => {
            quote! { *const ::std::os::raw::c_char }
        }
        Value::Custom(info, name) => {
            let ref_tok = reference.render_pointer();
            let name_tok: TokenStream = name.parse().unwrap();
            quote_spanned! { info.key.span() => #ref_tok #name_tok }
        }
    }
}
