use attrs::{
    parse_rid_attrs, Category, FunctionConfig, ImplBlockConfig, TypeInfo,
    TypeInfoMap,
};

use super::{
    parsed_function::ParsedFunction,
    rust_type::{RustType, TypeKind, Value},
};
use crate::{attrs, common::abort};

#[derive(Debug)]
pub struct ParsedImplBlock {
    pub ty: RustType,
    pub methods: Vec<ParsedFunction>,
}

impl ParsedImplBlock {
    pub fn new(
        item_impl: syn::ItemImpl,
        impl_attrs: &[attrs::RidAttr],
    ) -> Self {
        use syn::*;

        let self_ty = *item_impl.self_ty;
        let impl_config = ImplBlockConfig::new(impl_attrs);
        let ty = match RustType::from_type(&self_ty, &impl_config.type_infos) {
            Some(RustType {
                kind: TypeKind::Unknown,
                ident,
                reference,
            }) =>
            // NOTE: At this point we don't require the user to specify the type of the impl owner.
            // We assume it is a struct. It could be an enum, but most likely all arg types,
            // specifically pointer conversions will just work.  We may even consider merging the
            // Struct + Enum categories into one in the future.

            // TODO: handling simple case that is exporting methods on a simple struct here.
            // Not yet considering composites. We could try to derive those and/or somehow
            // detect non-trivial types and have the user annotate and/or just not allow impl on
            // those.
            {
                let ident_str = ident.to_string();
                let type_info = TypeInfo {
                    key: ident.clone(),
                    cat: Category::Struct,
                };
                RustType {
                    kind: TypeKind::Value(Value::Custom(type_info, ident_str)),
                    ident,
                    reference,
                }
            }
            Some(ty) => ty,
            None => abort!(self_ty, "Unexpected impl type {:#?}", self_ty),
        };

        let owner = Some((&ty.ident, &impl_config.type_infos));
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
                    let rid_attrs = parse_rid_attrs(&attrs);
                    let fn_config = FunctionConfig::new(&rid_attrs, owner);
                    if fn_config.is_exported {
                        Some(ParsedFunction::new(sig, &fn_config, owner))
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
                ty.ident,
                "Has export attribute but none of the contained methods is exported"
            );
        }

        Self { ty, methods }
    }
}
