use crate::{
    common::abort,
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_common::TypeAlias,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

pub struct RenderedReturnType {
    pub tokens: TokenStream,
    pub type_alias: Option<TypeAlias>,
}

pub fn render_return_type(
    rust_type: &RustType,
    is_in_return_type_position: bool,
) -> RenderedReturnType {
    use crate::parse::ParsedReference::*;
    use TypeKind as K;
    let RustType {
        ident,
        kind,
        reference,
    } = rust_type;

    let mut type_alias: Option<TypeAlias> = None;

    let type_tok = match kind {
        K::Primitive(prim) => render_primitive_return(prim),
        K::Value(val) => {
            let (alias, tokens ) = render_value_return(val, &reference);
            type_alias = alias;
            tokens
        }
        K::Composite(Composite::Vec, rust_type) => match rust_type {
            Some(ty) => {
                let (alias, tokens) = render_vec_return(ty.as_ref());
                type_alias = alias;
                tokens
            }
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },
        K::Composite(composite, rust_type) => {
            todo!("render_return_type::custom_composite")
        }
        K::Unit => quote! { () },
        K::Unknown => todo!("unknown .. need better error .. also gets triggered when exporting custom type without info "),
    };
    let tokens = quote_spanned! { ident.span() => #type_tok };

    RenderedReturnType { tokens, type_alias }
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

fn render_vec_return(
    inner_type: &RustType,
) -> (Option<TypeAlias>, TokenStream) {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            let inner_return_type = render_primitive_return(prim);
            let tokens = quote_spanned! { inner_type.ident.span() =>
                rid::RidVec<#inner_return_type>
            };
            (None, tokens)
        }
        K::Value(val) => {
            let (alias, val_tokens) =
                render_value_return(val, &inner_type.reference);
            let tokens = quote! { rid::RidVec<#val_tokens> };
            (alias, tokens)
        }
        K::Composite(_, _) => {
            abort!(inner_type.ident, "todo!(stringify_vec_return::composite)")
        }
        K::Unit => {
            abort!(inner_type.ident, "todo!(stringify_vec_return::unit)")
        }
        K::Unknown => {
            abort!(inner_type.ident, "todo!(stringify_vec_return::unknown)")
        }
    }
}

fn render_value_return(
    value: &Value,
    reference: &ParsedReference,
) -> (Option<TypeAlias>, TokenStream) {
    use Value as V;

    match value {
        V::CString | V::String | V::Str => {
            (None, quote! { *const ::std::os::raw::c_char })
        }
        V::Custom(info, name) => {
            let (alias, ref_tok) = reference.render_pointer(name, false);
            (alias, quote_spanned! { info.key.span() => #ref_tok })
        }
    }
}
