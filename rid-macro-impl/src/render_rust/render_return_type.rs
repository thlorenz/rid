use std::ops::Deref;

use crate::{
    attrs::Category,
    common::{abort, missing_struct_enum_info},
    parse::{
        rust_type::{Composite, Primitive, RustType, TypeKind, Value},
        ParsedReference,
    },
    render_common::{AccessKind, PointerTypeAlias},
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

pub fn render_return_type(
    rust_type: &RustType,
    access_kind: &AccessKind,
) -> RenderedReturnType {
    use crate::parse::ParsedReference::*;
    use TypeKind as K;
    let mut type_alias: Option<PointerTypeAlias> = None;

    let type_tok = match &rust_type.kind {
        // -----------------
        // Primitives
        // -----------------
        K::Primitive(prim) => render_primitive_return(prim, &rust_type),

        // -----------------
        // Values
        // -----------------
        K::Value(val) => {
            let (alias, tokens ) = render_value_return_type(val, &rust_type, access_kind);
            type_alias = alias;
            tokens
        }

        // -----------------
        // Composites Vec
        // -----------------
        K::Composite(Composite::Vec, inner_ty, _) => match inner_ty {
            Some(ty) => {
                let (alias, tokens) = render_vec_return_type(ty, access_kind);
                type_alias = alias;
                tokens
            }
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },

        // -----------------
        // Composites Option
        // -----------------
        K::Composite(Composite::Option, inner_ty, _) => match inner_ty {
            Some(ty) => {
                let (alias, tokens) = render_option_return_type(ty, access_kind);
                type_alias = alias;
                tokens
            }
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },
        
        // -----------------
        // Composites HashMap
        // -----------------
        K::Composite(Composite::HashMap, key_ty, val_ty) => {
            todo!("render_return_type::custom_composite HashMap<{:?}, {:?}>", key_ty, val_ty)
        }
        K::Composite(composite, rust_type, _) => {
            todo!("render_return_type::custom_composite {:?}", composite)
        },

        // -----------------
        // Unit
        // -----------------
        K::Unit => quote! { () },

        // -----------------
        // Invalid
        // -----------------
        K::Unknown => todo!("unknown .. need better error .. also gets triggered when exporting custom type without info "),
    };
    let ident = rust_type.rust_ident();
    let tokens = quote_spanned! { ident.span() => #type_tok };

    RenderedReturnType { tokens, type_alias }
}

fn render_primitive_return(prim: &Primitive, ty: &RustType) -> TokenStream {
    use Primitive::*;
    let ref_token = match ty.reference {
        ParsedReference::Owned => TokenStream::new(),
        // We dereference primitive refs since they implement the copy trait
        ParsedReference::Ref(_) => TokenStream::new(),
        ParsedReference::RefMut(_) => {
            abort!(ty.rust_ident(), "Cannot return RefMut types")
        }
    };
    match prim {
        U8 => quote! {  #ref_token u8 },
        I8 => quote! {  #ref_token i8 },
        U16 => quote! { #ref_token u16 },
        I16 => quote! { #ref_token i16 },
        U32 => quote! { #ref_token u32 },
        I32 => quote! { #ref_token i32 },
        U64 => quote! { #ref_token u64 },
        I64 => quote! { #ref_token i64 },
        USize => quote! { #ref_token usize },
        Bool => quote! { #ref_token u8 },
    }
}

fn render_vec_return_type(
    inner_type: &RustType,
    access_kind: &AccessKind,
) -> (Option<PointerTypeAlias>, TokenStream) {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            let inner_return_type = render_primitive_return(prim, inner_type);
            let tokens = quote_spanned! { inner_type.rust_ident().span() =>
                rid::RidVec<#inner_return_type>
            };
            (None, tokens)
        }
        K::Value(val) => {
            let (alias, val_tokens) =
                render_value_return_type(val, inner_type, access_kind);
            let tokens = quote! { rid::RidVec<#val_tokens> };
            (alias, tokens)
        }
        K::Composite(_, _, _) => {
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
    access_kind: &AccessKind,
) -> (Option<PointerTypeAlias>, TokenStream) {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            let inner_return_type = render_primitive_return(prim, inner_type);
            let tokens = quote_spanned! { inner_type.rust_ident().span() =>
                *const #inner_return_type
            };
            (None, tokens)
        }
        K::Value(val) => {
            render_value_return_type(val, &inner_type, access_kind)
        }
        K::Composite(_, _, _) => {
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
    access_kind: &AccessKind,
) -> (Option<PointerTypeAlias>, TokenStream) {
    use Category as C;
    use Value as V;

    match value {
        V::CString | V::String | V::Str => {
            (None, quote! { *const ::std::os::raw::c_char })
        }
        V::Custom(type_info, type_name) => match type_info.cat {
            C::Enum if access_kind == &AccessKind::MethodReturn => {
                (None, quote_spanned! { type_info.key.span() => i32 })
            }
            _ => {
                let (alias, ref_tok) = ty
                    .reference
                    .render_pointer(&ty.rust_ident().to_string(), false);
                (alias, quote_spanned! { type_info.key.span() => #ref_tok })
            }
        },
    }
}
