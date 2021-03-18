use syn::{Ident, Type, TypePath};

use crate::attrs::TypeInfoMap;

use super::{abort, extract_path_segment, ParsedReference, RustType};

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

    pub fn from_ty(
        ty: Box<Type>,
        type_infos: Option<&TypeInfoMap>,
        owner: Option<&Ident>,
    ) -> Option<RustArg> {
        let (ty, parsed_ref) = match *ty {
            Type::Reference(r) => {
                let pr = Some(ParsedReference::from(&r));
                (r.elem, pr)
            }
            Type::Path(_) => (ty, None),
            _ => return None,
        };
        if let Type::Path(TypePath { ref path, .. }) = *ty {
            let (ident, ty) = {
                let (ident, ty) = extract_path_segment(path, type_infos);
                if let Some(owner) = owner {
                    (ident, ty.with_replaced_self(owner))
                } else {
                    (ident, ty)
                }
            };
            Some(RustArg::new(ident, ty, parsed_ref))
        } else {
            None
        }
    }
}
