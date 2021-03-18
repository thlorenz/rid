use syn::{Lifetime, TypeReference};

#[derive(Debug, PartialEq)]
pub enum ParsedReference {
    Owned,
    OwnedMut,
    Ref(Option<syn::Ident>),
    RefMut(Option<syn::Ident>),
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
