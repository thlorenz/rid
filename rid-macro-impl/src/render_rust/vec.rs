use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use crate::{
    common::{abort, tokens::resolve_vec_ptr},
    render_common::{PointerTypeAlias, VecAccess, VecKind},
};

use super::{
    render_access_item, render_free, RenderedAccessItem, RenderedFree,
};

pub struct RenderedVecRust {
    pub tokens: TokenStream,
    pub type_aliases: Vec<PointerTypeAlias>,
}

impl VecAccess {
    pub fn render_rust(&self) -> RenderedVecRust {
        match self.kind {
            VecKind::FieldReference => self.render_rust_field_access(),
            VecKind::MethodReturn => self.render_rust_method_return(),
        }
    }

    // TODO: for now ignoring type aliases as they should have been added already when
    // the function using the vec whose access methods we render here were rendered.
    // If it turns out that we don't need those then we don't have to return them from
    // `render_free` or `render_access_item` either.
    fn render_rust_method_return(&self) -> RenderedVecRust {
        let mut type_aliases = Vec::<PointerTypeAlias>::new();

        let RenderedFree {
            tokens: free_tokens,
            type_alias,
        } = render_free(
            &self.vec_type,
            &self.fn_free_ident,
            &self.rust_ffi_prelude,
        );
        type_alias.map(|x| type_aliases.push(x));

        let RenderedAccessItem {
            tokens: access_tokens,
            type_alias,
        } = render_access_item(
            &self.vec_type,
            &self.fn_get_ident,
            &self.rust_ffi_prelude,
        );
        type_alias.map(|x| type_aliases.push(x));

        let tokens = quote! {
            #free_tokens
            #access_tokens
        };
        RenderedVecRust {
            tokens,
            type_aliases,
        }
    }

    /// Main difference to [VecAccess::render_rust] is that we work with references to a vec
    /// that is attached to a model. Thus we do not need `free` and don't wrap inside
    /// a [RidVec]. Instead we pass `*const Vec<T>` around.
    fn render_rust_field_access(&self) -> RenderedVecRust {
        let ffi_prelude = &self.rust_ffi_prelude;

        let item_ty = self.item_type.rust_ident();
        let resolve_vec = resolve_vec_ptr(&item_ty);

        let fn_len_ident = &self.fn_len_ident;
        let len_impl = quote_spanned! { fn_len_ident.span() =>
            #ffi_prelude
            fn #fn_len_ident(ptr: *mut Vec<#item_ty>) -> usize {
                #resolve_vec.len()
            }
        };
        let fn_get_ident = &self.fn_get_ident;
        let fn_get_ident_str_tokens: TokenStream =
            format!("\"{}\"", fn_get_ident).parse().unwrap();
        let get_impl = if self.item_type.is_struct() {
            quote_spanned! { fn_get_ident.span() =>
                #ffi_prelude
                fn #fn_get_ident(ptr: *mut Vec<#item_ty>, idx: usize) -> *const #item_ty  {
                    let item = #resolve_vec
                        .get(idx)
                        .expect(&format!("Failed to access {fn_get_ident}({idx})",
                            fn_get_ident = #fn_get_ident_str_tokens,
                            idx = idx
                        ));
                    item as *const #item_ty
                }
            }
        } else if self.item_type.is_primitive() {
            quote_spanned! { fn_get_ident.span() =>
                #ffi_prelude
                fn #fn_get_ident(ptr: *mut Vec<#item_ty>, idx: usize) -> #item_ty  {
                    let item = #resolve_vec
                        .get(idx)
                        .expect(&format!("Failed to access {fn_get_ident}({idx})",
                            fn_get_ident = #fn_get_ident_str_tokens,
                            idx = idx
                        ));
                    *item
                }
            }
        } else {
            abort!(item_ty, "Vec types other than owned structs and primitives are not supported yet.");
        };
        let tokens = quote! {
            #len_impl
            #get_impl
        };
        RenderedVecRust {
            tokens,
            type_aliases: vec![],
        }
    }
}
