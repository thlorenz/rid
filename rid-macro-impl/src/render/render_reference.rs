use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::ParsedReference;

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
}

pub fn render_lifetime(lifetime: Option<&syn::Ident>) -> TokenStream {
    match lifetime {
        Some(lifetime) => format!("'{}", lifetime).parse().unwrap(),
        None => TokenStream::new(),
    }
}
