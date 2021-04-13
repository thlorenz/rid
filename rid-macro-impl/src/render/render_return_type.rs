use crate::parse::{
    rust_type::{Composite, Primitive, RustType, TypeKind, Value},
    ParsedReference,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

pub struct RenderedReturnType {
    pub tokens: TokenStream,
    pub lifetime: Option<Ident>,
}

pub fn render_return_type(rust_type: &RustType) -> RenderedReturnType {
    use crate::parse::ParsedReference::*;
    use TypeKind as K;
    let RustType {
        ident,
        kind,
        reference,
    } = rust_type;
    let mut lifetime: Option<Ident> = None;

    let type_tok = match kind {
        K::Primitive(prim) => render_primitive_return(prim),
        K::Value(val) => render_value_return(val, reference),
        K::Composite(Composite::Vec, rust_type) => match rust_type {
            Some(ty) => {
                let (s, lt) = render_vec_return(ty.as_ref());
                lifetime = lt;
                s
            }
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },
        K::Composite(composite, rust_type) => {
            todo!("render_return_type::custom_composite")
        }
        K::Unit => todo!("unit"),
        K::Unknown => todo!("unknown .. need error"),
    };
    let tokens = quote_spanned! { ident.span() => #type_tok };

    RenderedReturnType { tokens, lifetime }
}

fn render_primitive_return(prim: &Primitive) -> TokenStream {
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

fn render_vec_return(inner_type: &RustType) -> (TokenStream, Option<Ident>) {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            let inner_return_type = render_primitive_return(prim);
            let tokens = quote_spanned! { inner_type.ident.span() =>
                rid::RidVec<#inner_return_type>
            };
            (tokens, None)
        }
        K::Value(val) => {
            let lifetime = format_ident!("a");
            let reference =
                inner_type.reference.ensured_lifetime(lifetime.clone());
            let val_tokens = render_value_return(val, &reference);
            let tokens = quote! { rid::RidVec<#val_tokens> };
            (tokens, reference.lifetime().cloned())
        }
        K::Composite(_, _) => todo!("stringify_vec_return::composite"),
        K::Unit => todo!("stringify_vec_return::unit -- should abort"),
        K::Unknown => {
            todo!("stringify_vec_return::unknown -- should abort")
        }
    }
}

fn render_value_return(
    value: &Value,
    reference: &ParsedReference,
) -> TokenStream {
    use Value as V;

    match value {
        V::CString | V::String | V::Str => {
            quote! { *const ::std::os::raw::c_char }
        }
        V::Custom(info, name) => {
            let ref_tok = reference.render();
            let name_tok: TokenStream = name.parse().unwrap();
            quote_spanned! { info.key.span() => #ref_tok #name_tok }
        }
    }
}
