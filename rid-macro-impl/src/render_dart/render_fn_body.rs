use quote::quote_spanned;
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};
use syn::Ident;

use crate::{
    attrs::Category,
    common::abort,
    parse::{
        rust_type::{Composite, RustType, TypeKind, Value},
        ParsedReceiver, ParsedReference,
    },
};

impl RustType {
    pub fn render_dart_fn_body(
        &self,
        rid_fn_ident: &Ident,
        receiver: &Option<ParsedReceiver>,
        indent: &str,
        comment: &str,
    ) -> String {
        let this_arg = match receiver {
            Some(_) => "this",
            None => "",
        };
        use TypeKind as K;
        match &self.kind {
            K::Primitive(_) => {
                abort!(self.ident, "TODO: RustType::render_fn_body K:Primitive")
            }
            K::Unit => abort!(
                self.ident,
                "Should not export rust method that returns nothing"
            ),

            K::Value(_) => format!(
                "{comment}{indent}  return {rid_ffi}.{rid_fn_ident}({this_arg});",
                comment = comment,
                rid_ffi = RID_FFI,
                rid_fn_ident = rid_fn_ident,
                this_arg = this_arg,
                indent = indent
            ),
           K::Composite(Composite::Vec, rust_type) => format!(
                "{comment}{indent}  return {rid_ffi}.{rid_fn_ident}({this_arg});",
                comment = comment,
                rid_ffi = RID_FFI,
                rid_fn_ident = rid_fn_ident,
                this_arg = this_arg,
                indent = indent
            ),
            K::Composite(_, _) => {
                abort!(self.ident, "TODO: RustType::render_fn_body K::Composite")
            }
            K::Unknown => abort!(self.ident, "TODO: RustType::render_fn_body K::Unknown"),
        }
    }
}
