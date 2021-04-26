use proc_macro2::TokenStream;
use quote::quote;

use crate::render_common::{TypeAlias, VecAccess};

use super::{
    render_access_item, render_free, RenderedAccessItem, RenderedFree,
};

pub struct RenderedVecRust {
    pub tokens: TokenStream,
    pub type_aliases: Vec<TypeAlias>,
}

impl VecAccess {
    // TODO: for now ignoring type aliases as they should have been added already when
    // the function using the vec whose access methods we render here were rendered.
    // If it turns out that we don't need those then we don't have to return them from
    // `render_free` or `render_access_item` either.
    pub fn render_rust(&self) -> RenderedVecRust {
        let mut type_aliases = Vec::<TypeAlias>::new();

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
}
