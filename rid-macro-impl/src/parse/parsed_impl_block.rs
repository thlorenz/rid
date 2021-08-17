use attrs::{
    parse_rid_attrs, Category, FunctionConfig, ImplBlockConfig, TypeInfo,
    TypeInfoMap,
};
use rid_common::STORE;

use super::{
    parsed_function::ParsedFunction,
    rust_type::{RustType, TypeKind, Value},
};
use crate::{
    attrs::{self, raw_typedef_ident, RidAttr},
    common::abort,
    parse::rust_type::RustTypeContext,
};

#[derive(Debug)]
pub struct ParsedImplBlock {
    pub ty: RustType,
    pub methods: Vec<ParsedFunction>,
    pub config: ImplBlockConfig,
}

impl ParsedImplBlock {
    pub fn new(item_impl: syn::ItemImpl, config: ImplBlockConfig) -> Self {
        use syn::*;

        let self_ty = *item_impl.self_ty;
        let ty = match RustType::from_type(&self_ty, &config.type_infos) {
            Some(ty) if ty.kind == TypeKind::Unknown =>
            // NOTE: At this point we don't require the user to specify the type of the impl owner.
            // We assume it is a struct. It could be an enum, but most likely all arg types,
            // specifically pointer conversions will just work.  We may even consider merging the
            // Struct + Enum categories into one in the future.

            // TODO: handling simple case that is exporting methods on a simple struct here.
            // Not yet considering composites. We could try to derive those and/or somehow
            // detect non-trivial types and have the user annotate and/or just not allow impl on
            // those.
            {
                let ident = ty.rust_ident();
                let reference = &ty.reference;
                let ident_str = ident.to_string();
                let type_info = TypeInfo {
                    key: ident.clone(),
                    cat: Category::Struct,
                    typedef: Some(raw_typedef_ident(&ident)),
                };
                RustType::new(
                    ident.clone(),
                    TypeKind::Value(Value::Custom(type_info, ident_str)),
                    reference.clone(),
                    RustTypeContext::Default,
                )
            }
            Some(ty) => ty,
            None => abort!(self_ty, "Unexpected impl type {:#?}", self_ty),
        };

        let owner = Some((ty.rust_ident(), &config.type_infos));
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
                        Some(ParsedFunction::new(sig, fn_config, owner))
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

        if ty.rust_ident().to_string().as_str() != STORE {
            methods.iter().for_each(|x| if x.receiver.is_some() {
                abort!(x.fn_ident, "Methods with a receiver, i.e. `&self` can only be exported from the store")
            })
        }

        if methods.is_empty() {
            abort!(
                ty.rust_ident(),
                "Has export attribute but none of the contained methods is exported"
            );
        }

        Self {
            ty,
            methods,
            config,
        }
    }

    /// Parsed attributes of the impl block
    pub fn attrs(&self) -> &[RidAttr] {
        &self.config.attrs
    }

    /// Information about custom types used inside this impl block
    pub fn type_infos(&self) -> &TypeInfoMap {
        &self.config.type_infos
    }
}
