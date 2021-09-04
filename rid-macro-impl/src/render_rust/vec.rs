use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use render_return_type::RenderedReturnType;
use syn::Ident;

use crate::{
    accesses::{
        collection_access_tokens, AccessKind, CollectionAccessTokens,
        RenderedAccessRust, VecAccess,
    },
    common::{abort, tokens::resolve_vec_ptr},
    parse::rust_type::RustType,
    render_common::PointerTypeAlias,
    render_rust::{render_return_type, render_to_return_type},
};

use super::{render_free, RenderedFree};

impl VecAccess {
    // -----------------
    // MethodReturn
    // -----------------

    // TODO: for now ignoring type aliases as they should have been added already when
    // the function using the vec whose access methods we render here were rendered.
    // If it turns out that we don't need those then we don't have to return them from
    // `render_free` or `render_access_item` either.
    pub fn render_rust_method_return(&self) -> RenderedAccessRust {
        let mut type_aliases = HashMap::<String, PointerTypeAlias>::new();

        let RenderedFree {
            tokens: free_tokens,
            type_alias,
        } = render_free(
            &self.vec_type,
            &self.fn_free_ident,
            &self.rust_ffi_prelude,
            &AccessKind::MethodReturn,
        );
        type_alias.map(|x| type_aliases.insert(x.alias.to_string(), x));

        let inner_ty = self
            .vec_type
            .inner_composite_type()
            .expect("Vec should have inner type");
        let RenderedVecAccessItem {
            tokens: access_tokens,
            type_alias,
        } = render_vec_access_item(
            &self.vec_type,
            &inner_ty,
            &self.fn_get_ident,
            &AccessKind::MethodReturn,
            &self.rust_ffi_prelude,
        );

        type_alias.map(|x| type_aliases.insert(x.alias.to_string(), x));

        let tokens = quote! {
            #free_tokens
            #access_tokens
        };
        RenderedAccessRust {
            tokens,
            type_aliases,
        }
    }

    // -----------------
    // FieldAccess
    // -----------------

    /// Main difference to [VecAccess::render_rust] is that we work with references to a vec
    /// that is attached to a model. Thus we do not need `free` and don't wrap inside
    /// a [RidVec]. Instead we pass `*const Vec<T>` around.
    pub fn render_rust_field_access(&self) -> RenderedAccessRust {
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
        } else if self.item_type.is_enum() {
            quote_spanned! { fn_get_ident.span() =>
                #ffi_prelude
                fn #fn_get_ident(ptr: *mut Vec<#item_ty>, idx: usize) -> i32  {
                    let item = #resolve_vec
                        .get(idx)
                        .expect(&format!("Failed to access {fn_get_ident}({idx})",
                            fn_get_ident = #fn_get_ident_str_tokens,
                            idx = idx
                        ));
                    item._rid_into_discriminant()
                }
            }
        } else if self.item_type.is_string_like() {
            let RenderedReturnType {
                tokens: return_ty, ..
            } = render_return_type(
                &self.item_type,
                &AccessKind::FieldReference,
            );

            let res_ident = format_ident!("item");
            let res_pointer = format_ident!("item_ptr");
            let to_return = &self.item_type.render_to_return(
                &res_ident,
                &res_pointer,
                true,
            );

            quote_spanned! { fn_get_ident.span() =>
                #ffi_prelude
                fn #fn_get_ident(ptr: *mut Vec<#item_ty>, idx: usize) -> #return_ty  {
                    let #res_ident = #resolve_vec
                        .get(idx)
                        .expect(&format!("Failed to access {fn_get_ident}({idx})",
                            fn_get_ident = #fn_get_ident_str_tokens,
                            idx = idx
                        ));
                    #to_return
                    #res_pointer
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
        RenderedAccessRust {
            tokens,
            type_aliases: HashMap::new(),
        }
    }
}

// -----------------
// render_vec_access_item
// -----------------
struct RenderedVecAccessItem {
    pub tokens: TokenStream,
    pub type_alias: Option<PointerTypeAlias>,
}

fn render_vec_access_item(
    outer_type: &RustType,
    item_type: &RustType,
    fn_access_ident: &Ident,
    access_kind: &AccessKind,
    ffi_prelude: &TokenStream,
) -> RenderedVecAccessItem {
    let RenderedReturnType {
        tokens: vec_arg_type,
        ..
    } = render_return_type(outer_type, access_kind);

    let CollectionAccessTokens {
        item_return_type,
        into_return_type,
        type_alias,
    } = collection_access_tokens(format_ident!("ptr"), item_type, access_kind);

    let access_fn = quote_spanned! { fn_access_ident.span() =>
        fn #fn_access_ident(vec: #vec_arg_type, idx: usize) -> #item_return_type {
            let ptr = vec[idx];
            #into_return_type

        }
    };
    let tokens = quote! {
        #ffi_prelude
        #access_fn
    };
    RenderedVecAccessItem { tokens, type_alias }
}
