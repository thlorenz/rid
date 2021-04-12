use syn::Lifetime;

use crate::attrs::TypeInfo;

use super::ParsedReference;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedReceiver {
    pub reference: ParsedReference,
    pub info: TypeInfo,
}

impl ParsedReceiver {
    pub fn new(receiver: &syn::Receiver, info: TypeInfo) -> Self {
        let syn::Receiver {
            attrs: _,      // Vec<Attribute>,
            reference,     // Option<(Token![&], Option<Lifetime>)>,
            mutability,    // Option<Token![mut]>,
            self_token: _, // Token![self],
        } = receiver;

        let lifetime_ident = match reference {
            Some((_, Some(Lifetime { ident, .. }))) => Some(ident.clone()),
            _ => None,
        };

        let r = match reference {
            Some(_) if mutability.is_none() => {
                ParsedReference::Ref(lifetime_ident)
            }
            Some(_) => ParsedReference::RefMut(lifetime_ident),
            None => {
                assert!(mutability.is_none(), "cannot pass mut owned");
                ParsedReference::Owned
            }
        };
        ParsedReceiver { reference: r, info }
    }
}
