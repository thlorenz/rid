use std::ops::Deref;

use crate::{
    common::{abort, missing_struct_enum_info},
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_common::PointerTypeAlias,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

pub struct RenderedReturnType {
    /// Rendered return type
    pub tokens: TokenStream,
    /// Pointer aliases, i.e. type Pointer_RawTodo = *const RawTodo;
    pub type_alias: Option<PointerTypeAlias>,
}

pub fn render_return_type(rust_type: &RustType) -> RenderedReturnType {
    use crate::parse::ParsedReference::*;
    use TypeKind as K;
    let mut type_alias: Option<PointerTypeAlias> = None;

    let type_tok = match &rust_type.kind {
        K::Primitive(prim) => render_primitive_return(prim),
        K::Value(val) => {
            let (alias, tokens ) = render_value_return_type(val, &rust_type);
            type_alias = alias;
            tokens
        }
        K::Composite(Composite::Vec, inner_ty) => match inner_ty {
            Some(ty) => {
                let (alias, tokens) = render_vec_return_type(ty);
                type_alias = alias;
                tokens
            }
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },
        K::Composite(Composite::Option, inner_ty) => match inner_ty {
            Some(ty) => {
                let (alias, tokens) = render_option_return_type(ty);
                type_alias = alias;
                tokens
            }
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },
        K::Composite(composite, rust_type) => {
            todo!("render_return_type::custom_composite")
        },
        K::Unit => quote! { () },
        K::Unknown => todo!("unknown .. need better error .. also gets triggered when exporting custom type without info "),
    };
    let ident = rust_type.rust_ident();
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

fn render_vec_return_type(
    inner_type: &RustType,
) -> (Option<PointerTypeAlias>, TokenStream) {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            let inner_return_type = render_primitive_return(prim);
            let tokens = quote_spanned! { inner_type.rust_ident().span() =>
                rid::RidVec<#inner_return_type>
            };
            (None, tokens)
        }
        K::Value(val) => {
            let (alias, val_tokens) = render_value_return_type(val, inner_type);
            let tokens = quote! { rid::RidVec<#val_tokens> };
            (alias, tokens)
        }
        K::Composite(_, _) => {
            abort!(
                inner_type.rust_ident(),
                "todo!(render_vec_return_type::composite)"
            )
        }
        K::Unit => {
            abort!(
                inner_type.rust_ident(),
                "todo!(render_vec_return_type::unit)"
            )
        }
        K::Unknown => {
            abort!(
                inner_type.rust_ident(),
                "todo!(render_vec_return_type::unknown)"
            )
        }
    }
}
fn render_option_return_type(
    inner_type: &RustType,
) -> (Option<PointerTypeAlias>, TokenStream) {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            let inner_return_type = render_primitive_return(prim);
            let tokens = quote_spanned! { inner_type.rust_ident().span() =>
                *const #inner_return_type
            };
            (None, tokens)
        }
        K::Value(val) => render_value_return_type(val, &inner_type),
        K::Composite(_, _) => {
            abort!(
                inner_type.rust_ident(),
                "todo!(render_option_return_type::composite)"
            )
        }
        K::Unit => {
            abort!(
                inner_type.rust_ident(),
                "todo!(render_option_return_type::unit)"
            )
        }
        K::Unknown => {
            (None, missing_struct_enum_info(&inner_type.rust_ident()))
        }
    }
}

fn render_value_return_type(
    value: &Value,
    ty: &RustType,
) -> (Option<PointerTypeAlias>, TokenStream) {
    use Value as V;

    match value {
        V::CString | V::String | V::Str => {
            (None, quote! { *const ::std::os::raw::c_char })
        }
        V::Custom(info, _) => {
            let (alias, ref_tok) = ty
                .reference
                .render_pointer(&ty.rust_ident().to_string(), false);
            (alias, quote_spanned! { info.key.span() => #ref_tok })
        }
    }
}
