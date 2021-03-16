use std::collections::HashMap;

use proc_macro_error::abort;

use super::parsed_function::ParsedFunction;
use crate::{
    attrs,
    common::{extract_path_segment, RustType},
};

#[derive(Debug)]
pub struct ParsedImplBlock {
    ty: (syn::Ident, RustType),
    methods: Vec<ParsedFunction>,
}

impl ParsedImplBlock {
    pub fn new(item_impl: syn::ItemImpl, _args: &[attrs::RidAttr]) -> Self {
        use syn::*;

        // TODO: ignore methods that don't have an export attr
        let methods = item_impl
            .items
            .into_iter()
            .flat_map(|item| match item {
                ImplItem::Method(ImplItemMethod {
                    attrs,       // Vec<Attribute>,
                    vis,         // Visibility,
                    defaultness, // Option<Token![default]>,
                    block,       // Block,
                    sig,         // Signature
                }) => {
                    let attrs = attrs::parse_rid_attrs(&attrs);
                    Some(ParsedFunction::new(sig, attrs))
                }
                ImplItem::Const(_)
                | ImplItem::Type(_)
                | ImplItem::Macro(_)
                | ImplItem::Verbatim(_)
                | ImplItem::__TestExhaustive(_) => None,
            })
            .collect();

        let self_ty = *item_impl.self_ty;
        let ty = if let Type::Path(TypePath {
            qself, // Option<QSelf>,
            path,  // Path,
        }) = self_ty
        {
            extract_path_segment(&path, None)
        } else {
            abort!(self_ty, "Unexpected impl type {:#?}", self_ty);
        };
        Self { ty, methods }
    }
}
