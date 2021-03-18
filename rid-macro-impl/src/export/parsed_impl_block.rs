use std::collections::HashMap;

use attrs::TypeInfoMap;

use super::parsed_function::ParsedFunction;
use crate::{
    attrs,
    common::{
        abort, extract_path_segment, ParsedReceiver, PrimitiveType, RustArg, RustType, ValueType,
    },
};

#[derive(Debug)]
pub struct ParsedImplBlock {
    ty: (syn::Ident, RustType),
    methods: Vec<ParsedFunction>,
}

impl ParsedImplBlock {
    pub fn new(item_impl: syn::ItemImpl, impl_attrs: &[attrs::RidAttr]) -> Self {
        use syn::*;

        let self_ty = *item_impl.self_ty;
        let impl_type_infos = TypeInfoMap::from(impl_attrs);

        let ty = if let Type::Path(TypePath {
            qself, // Option<QSelf>,
            path,  // Path,
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
                    attrs,       // Vec<Attribute>,
                    vis,         // Visibility,
                    defaultness, // Option<Token![default]>,
                    block,       // Block,
                    sig,         // Signature
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

#[cfg(test)]
mod tests {
    use crate::common::ParsedReference;

    use super::*;
    use attrs::{Category, TypeInfo};
    use quote::quote;

    fn parse(input: proc_macro2::TokenStream) -> ParsedImplBlock {
        let item = syn::parse2::<syn::Item>(input).unwrap();
        let args = syn::AttributeArgs::new();
        match item {
            syn::Item::Impl(item) => {
                let attrs = attrs::parse_rid_attrs(&item.attrs);
                ParsedImplBlock::new(item, &attrs)
            }
            _ => panic!("Unexpected item, we're trying to parse functions here"),
        }
    }

    #[test]
    fn impl_block_with_new_returning_self() {
        let ParsedImplBlock {
            ty: (ident, ty),
            methods,
        } = parse(quote! {
            #[rid(export)]
            impl MyStruct {
                #[rid(export)]
                pub fn new(id: u8) -> Self {
                    Self { id }
                }
            }
        });
    }

    #[test]
    fn impl_block_with_four_methods_three_with_rid_export_attr() {
        let ParsedImplBlock {
            ty: (ident, ty),
            methods,
        } = parse(quote! {
            #[rid(export)]
            impl MyStruct {
                #[rid(export)]
                pub fn new(id: u8) -> Self {
                    Self { id }
                }
                #[rid(export)]
                pub fn get_id(&self) -> u8 {
                    self.id
                }
                #[rid(export)]
                pub fn set_id(&mut self, id: u8) {
                    self.id = id;
                }
                pub fn inc_id(&mut self, id: u8) {
                    self.id += 1;
                }
            }
        });

        let owner_ty = RustType::Value(ValueType::RCustom(
            TypeInfo {
                key: ident,
                cat: Category::Struct,
            },
            "MyStruct".to_string(),
        ));
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustArg { ty: ret_ty, .. },
        } = &methods[0];

        assert_eq!(ty, owner_ty, "impl type");
        assert_eq!(methods.len(), 3, "exports 3 methods");

        // First Method: pub fn new(id: u8) -> Self
        assert_eq!(fn_ident.to_string(), "new", "function ident");
        assert_eq!(
            ret_ty, &owner_ty,
            "new() -> Self return type is owning struct"
        );
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].ty,
            RustType::Primitive(PrimitiveType::U8),
            "first arg u8"
        );
        assert_eq!(receiver, &None, "no receiver");

        // Second Method: pub fn get_id(&self) -> u8
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustArg { ty: ret_ty, .. },
        } = &methods[1];

        assert_eq!(fn_ident.to_string(), "get_id", "function ident");
        assert_eq!(args.len(), 0, "no arg");
        assert_eq!(
            receiver,
            &Some(ParsedReceiver {
                reference: ParsedReference::Ref(None),
            }),
            "receiver is ref"
        );
        assert_eq!(
            ret_ty,
            &RustType::Primitive(PrimitiveType::U8),
            "returns u8"
        );

        // Third Method: pub fn set_id(&mut self, id: u8)
        let ParsedFunction {
            fn_ident,
            receiver,
            args,
            return_arg: RustArg { ty: ret_ty, .. },
        } = &methods[2];
        assert_eq!(fn_ident.to_string(), "set_id", "function ident");
        assert_eq!(args.len(), 1, "one arg");
        assert_eq!(
            args[0].ty,
            RustType::Primitive(PrimitiveType::U8),
            "first arg u8"
        );
        assert_eq!(
            receiver,
            &Some(ParsedReceiver {
                reference: ParsedReference::RefMut(None),
            }),
            "receiver is ref mut"
        );
        assert_eq!(ret_ty, &RustType::Unit, "returns ()");
    }
}
