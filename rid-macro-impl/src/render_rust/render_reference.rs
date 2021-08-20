use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{parse::ParsedReference, render_common::PointerTypeAlias};
impl ParsedReference {
    pub fn stringify_lifetime(&self) -> String {
        match self.lifetime() {
            Some(lifetime) => format!("'{}", lifetime),
            None => "".to_string(),
        }
    }

    pub fn render(&self) -> TokenStream {
        match self {
            ParsedReference::Owned => TokenStream::new(),
            ParsedReference::Ref(lifetime) => {
                let lifetime_toks = render_lifetime(lifetime.as_ref());
                quote! { &#lifetime_toks }
            }
            ParsedReference::RefMut(lifetime) => {
                let lifetime_toks = render_lifetime(lifetime.as_ref());
                quote! { &#lifetime_toks mut }
            }
        }
    }

    pub fn render_pointer(
        &self,
        type_name: &str,
        qualified_type_name: &str,
        is_primitive: bool,
    ) -> (Option<PointerTypeAlias>, TokenStream) {
        let name_tok: TokenStream = qualified_type_name.parse().unwrap();

        match self {
            ParsedReference::Owned => {
                if is_primitive {
                    (None, name_tok)
                } else {
                    aliased_pointer(type_name, name_tok, true)
                }
            }
            ParsedReference::Ref(_) => {
                aliased_pointer(type_name, name_tok, false)
            }

            ParsedReference::RefMut(_) => {
                aliased_pointer(type_name, name_tok, true)
            }
        }
    }

    pub fn render_deref(&self) -> TokenStream {
        match self {
            ParsedReference::Owned => TokenStream::new(),
            ParsedReference::Ref(_) => quote! { .as_ref() },
            ParsedReference::RefMut(_) => quote! { .as_mut() },
        }
    }

    pub fn is_owned(&self) -> bool {
        match self {
            ParsedReference::Owned => true,
            ParsedReference::Ref(_) | ParsedReference::RefMut(_) => false,
        }
    }
}

pub fn render_lifetime(lifetime: Option<&syn::Ident>) -> TokenStream {
    match lifetime {
        Some(lifetime) => format!("'{}", lifetime).parse().unwrap(),
        None => TokenStream::new(),
    }
}

pub fn render_lifetime_def(lifetime: Option<&syn::Ident>) -> TokenStream {
    match lifetime {
        Some(lt) => {
            let lt: TokenStream = format!("'{}", lt).parse().unwrap();
            quote! { <#lt> }
        }
        None => TokenStream::new(),
    }
}

fn aliased_pointer(
    alias_type_name: &str,
    name_tok: TokenStream,
    is_mut: bool,
) -> (Option<PointerTypeAlias>, TokenStream) {
    let (alias, typedef) = if is_mut {
        let alias = format_ident!(
            "{}{}",
            PointerTypeAlias::POINTER_MUT_ALIAS_PREFIX,
            alias_type_name
        );
        (alias.clone(), quote! { type #alias = *mut #name_tok; })
    } else {
        let alias = format_ident!(
            "{}{}",
            PointerTypeAlias::POINTER_ALIAS_PREFIX,
            alias_type_name
        );
        (alias.clone(), quote! { type #alias = *const #name_tok; })
    };

    let tokens = quote! { #alias };
    (
        Some(PointerTypeAlias {
            alias,
            typedef,
            type_name: alias_type_name.to_string(),
            needs_free: is_mut,
        }),
        tokens,
    )
}
