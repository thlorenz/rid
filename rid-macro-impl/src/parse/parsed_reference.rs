use std::fmt::Debug;

use quote::format_ident;
use syn::{Lifetime, TypeReference};

#[derive(PartialEq)]
pub enum ParsedReference {
    Owned,
    Ref(Option<syn::Ident>),
    RefMut(Option<syn::Ident>),
}

impl Debug for ParsedReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            ParsedReference::Owned => "ParsedReference".to_string(),
            ParsedReference::Ref(ident) => {
                format!("ParsedReference::Ref({:?})", ident)
            }
            ParsedReference::RefMut(ident) => {
                format!("ParsedReference::RefMut({:?})", ident)
            }
        };
        write!(f, "{}", r)
    }
}

impl From<&TypeReference> for ParsedReference {
    fn from(r: &TypeReference) -> Self {
        let TypeReference {
            lifetime,
            mutability,
            ..
        } = r;

        let lifetime_ident = match lifetime {
            Some(Lifetime { ident, .. }) => Some(ident.clone()),
            None => None,
        };

        match mutability.is_some() {
            true => ParsedReference::RefMut(lifetime_ident.clone()),
            false => ParsedReference::Ref(lifetime_ident.clone()),
        }
    }
}

impl ParsedReference {
    pub fn with_lifetime(self, lifetime: syn::Ident) -> Self {
        match self {
            ParsedReference::Owned => self,
            ParsedReference::Ref(_) => ParsedReference::Ref(Some(lifetime)),
            ParsedReference::RefMut(_) => {
                ParsedReference::RefMut(Some(lifetime))
            }
        }
    }
}
