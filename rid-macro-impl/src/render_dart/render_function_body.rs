use quote::quote_spanned;
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};
use syn::Ident;

const RES_IDENT: &str = "res";
const RET_IDENT: &str = "ret";

use crate::{
    attrs::Category,
    common::abort,
    parse::{
        rust_type::{Composite, RustType, TypeKind, Value},
        ParsedReceiver, ParsedReference,
    },
};

use super::DartArg;

impl RustType {
    pub fn render_dart_function_body(
        &self,
        rid_fn_ident: &Ident,
        receiver: &Option<ParsedReceiver>,
        args: &[DartArg],
        indent: &str,
        comment: &str,
    ) -> String {
        let to_return_type =
            self.render_dart_to_return_type(RES_IDENT, RET_IDENT, comment);

        let this_arg = match receiver {
            Some(_) if args.is_empty() => "this",
            Some(_) => "this, ",
            None => "",
        };
        let params = args
            .iter()
            .map(DartArg::render_parameter)
            .collect::<Vec<String>>()
            .join(", ");

        use TypeKind as K;
        let call = match &self.kind {
            K::Unit => abort!(
                self.rust_ident(),
                "Should not export rust method that returns nothing"
            ),
            // TODO(thlorenz): All the below do the same, need to investigate if that makes sense
            // and merge match arms or apply changes if needed.
            K::Primitive(_) => format!(
                "{comment}{indent}  final {res_ident} = {rid_ffi}.{rid_fn_ident}({this_arg}{params});",
                comment = comment,
                indent = indent,
                res_ident = RES_IDENT,
                rid_ffi = RID_FFI,
                rid_fn_ident = rid_fn_ident,
                this_arg = this_arg,
                params = params
            ),

            K::Value(_) => format!(
                "{comment}{indent}  final {res_ident} = {rid_ffi}.{rid_fn_ident}({this_arg}{params});",
                comment = comment,
                indent = indent,
                res_ident = RES_IDENT,
                rid_ffi = RID_FFI,
                rid_fn_ident = rid_fn_ident,
                this_arg = this_arg,
                params = params
            ),
           K::Composite(Composite::Vec, rust_type, _) => format!(
                "{comment}{indent}  final {res_ident} = {rid_ffi}.{rid_fn_ident}({this_arg}{params});",
                comment = comment,
                indent = indent,
                res_ident = RES_IDENT,
                rid_ffi = RID_FFI,
                rid_fn_ident = rid_fn_ident,
                this_arg = this_arg,
                params = params
            ),
            K::Composite(Composite::Option, rust_type, _) => format!(
                "{comment}{indent}  final {res_ident} = {rid_ffi}.{rid_fn_ident}({this_arg}{params});",
                comment = comment,
                indent = indent,
                res_ident = RES_IDENT,
                rid_ffi = RID_FFI,
                rid_fn_ident = rid_fn_ident,
                this_arg = this_arg,
                params = params
            ),
            K::Composite(Composite::HashMap, key_type, val_type) => {
                // TODO(thlorenz): HashMap 
                abort!(self.rust_ident(), "TODO: RustType::render_fn_body K::Composite::HashMap<{:?}, {:?}>", key_type, val_type)
            },
            K::Composite(kind, _, _) => {
                abort!(self.rust_ident(), "TODO: RustType::render_fn_body K::Composite({:?})", kind)
            }
            K::Unknown => abort!(self.rust_ident(), "TODO: RustType::render_fn_body K::Unknown"),
        };
        format!(
            r###"{call}
{comment}{indent}  {to_return_type}
{comment}{indent}  return {ret_ident};"###,
            comment = comment,
            indent = indent,
            call = call,
            to_return_type = to_return_type,
            ret_ident = RET_IDENT
        )
    }
}
