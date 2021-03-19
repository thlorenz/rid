use attrs::TypeInfoMap;

use super::parsed_function::ParsedFunction;
use crate::{
    attrs,
    common::{abort, extract_path_segment, RustType},
};

#[derive(Debug)]
pub struct ParsedImplBlock {
    pub ty: (syn::Ident, RustType),
    pub methods: Vec<ParsedFunction>,
}

impl ParsedImplBlock {
    pub fn new(item_impl: syn::ItemImpl, impl_attrs: &[attrs::RidAttr]) -> Self {
        use syn::*;

        let self_ty = *item_impl.self_ty;
        let impl_type_infos = TypeInfoMap::from(impl_attrs);

        let ty = if let Type::Path(TypePath {
            qself: _, // Option<QSelf>,
            path,     // Path,
        }) = self_ty
        {
            extract_path_segment(&path, None)
        } else {
            abort!(self_ty, "Unexpected impl type {:#?}", self_ty);
        };

        let methods: Vec<ParsedFunction> = item_impl
            .items
            .into_iter()
            .flat_map(|item| match item {
                ImplItem::Method(ImplItemMethod {
                    attrs,          // Vec<Attribute>,
                    vis: _,         // Visibility,
                    defaultness: _, // Option<Token![default]>,
                    block: _,       // Block,
                    sig,            // Signature
                }) => {
                    let method_attrs = attrs::parse_rid_attrs(&attrs);
                    if method_attrs.iter().any(|x| x.is_export()) {
                        Some(ParsedFunction::new(
                            sig,
                            &method_attrs,
                            Some((&ty.0, &impl_type_infos)),
                        ))
                    } else {
                        None
                    }
                }
                ImplItem::Const(_)
                | ImplItem::Type(_)
                | ImplItem::Macro(_)
                | ImplItem::Verbatim(_)
                | ImplItem::__TestExhaustive(_) => None,
            })
            .collect();

        if methods.is_empty() {
            abort!(
                ty.0,
                "Has export attribute but none of the contained methods is exported"
            );
        }

        Self { ty, methods }
    }
}
