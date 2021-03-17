#[derive(Debug, PartialEq)]
pub enum ReceiverReference {
    Owned,
    OwnedMut,
    Ref,
    RefMut,
}

#[derive(Debug, PartialEq)]
pub struct ParsedReceiver {
    pub reference: ReceiverReference,
}

impl ParsedReceiver {
    pub fn new(receiver: &syn::Receiver) -> Self {
        let syn::Receiver {
            attrs,      // Vec<Attribute>,
            reference,  // Option<(Token![&], Option<Lifetime>)>,
            mutability, // Option<Token![mut]>,
            self_token, // Token![self],
        } = receiver;

        // NOTE: ignoring lifetime for now, as it isn't important for wrapper function
        let r = match reference {
            Some(_) if mutability.is_none() => ReceiverReference::Ref,
            Some(_) => ReceiverReference::RefMut,
            None if mutability.is_none() => ReceiverReference::Owned,
            None => ReceiverReference::OwnedMut,
        };
        ParsedReceiver { reference: r }
    }
}
