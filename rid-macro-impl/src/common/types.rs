use syn::Ident;

use super::{ParsedReference, RustType};

#[derive(Debug)]
pub struct RustArg {
    pub ident: Ident,
    pub ty: RustType,
    pub reference: Option<ParsedReference>,
}

impl RustArg {
    pub fn new(ident: Ident, ty: RustType, reference: Option<ParsedReference>) -> Self {
        RustArg {
            ident,
            ty,
            reference,
        }
    }
}
