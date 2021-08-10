use crate::{
    common::abort,
    parse::rust_type::{self, Composite, RustType, TypeKind, Value},
};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

impl RustType {
    pub fn render_dart_to_return_type(
        &self,
        res_ident: &str,
        ret_ident: &str,
    ) -> String {
        use TypeKind as K;
        match &self.kind {
            K::Primitive(rust_type::Primitive::Bool) => format!(
                "final {ret_ident} = {res_ident} == 1;",
                ret_ident = ret_ident,
                res_ident = res_ident
            ),
            K::Primitive(_) => format!(
                "final {ret_ident} = {res_ident};",
                ret_ident = ret_ident,
                res_ident = res_ident
            ),
            K::Value(val) => format!(
                "final {ret_ident} = {res_ident};",
                ret_ident = ret_ident,
                res_ident = res_ident
            ),
            K::Composite(Composite::Vec, inner_type, _) => format!(
                "final {ret_ident} = {res_ident};",
                ret_ident = ret_ident,
                res_ident = res_ident
            ),
            K::Composite(Composite::Option, inner_type, _) => {
                match inner_type {
                    Some(ty) => {
                        format!(
                        "final {ret_ident} = {res_ident}.address == 0x0 ? null : {res_ident};",
                        ret_ident = ret_ident,
                        res_ident = res_ident
                    )
                    }
                    None => {
                        abort!(
                            self.rust_ident(),
                            "Rust Option composite should include inner type"
                        )
                    }
                }
            }
            K::Composite(Composite::HashMap, key_type, val_type) => {
                // TODO(thlorenz): HashMap
                abort!(
                    self.rust_ident(),
                    "TODO: RustType::render_dart_to_return_type K::Composite::HashMap<{:?}, {:?}>",
                    key_type, val_type
                )
            }
            K::Composite(kind, _, _) => {
                abort!(
                    self.rust_ident(),
                    "TODO: RustType::render_dart_to_return_type K::Composite({:?})",
                    kind
                )
            }
            K::Unit => abort!(
                self.rust_ident(),
                "Should not export rust method that returns nothing"
            ),

            K::Unknown => abort!(
                self.rust_ident(),
                "TODO: RustType::render_dart_to_return_type K::Unknown"
            ),
        }
    }
}
