use syn::Ident;

use crate::{
    common::abort,
    parse::rust_type::{RustType, TypeKind, Value},
};

pub fn render_swift_call(
    fn_ident_str: &str,
    args: &[RustType],
    has_receiver: bool,
) -> String {
    let mut swift_dummy_args: Vec<String> = args
        .iter()
        .map(|x| SwiftArg::from(x).render_dummy_arg())
        .collect();

    if has_receiver {
        swift_dummy_args.insert(0, SwiftArg::Nil.render_dummy_arg());
    }

    format!("{}({})", fn_ident_str, swift_dummy_args.join(","))
}

enum SwiftArg {
    Nil,
    Integer,
    String,
}

impl SwiftArg {
    fn render_dummy_arg(&self) -> String {
        use SwiftArg::*;
        match self {
            Nil => "nil",
            Integer => "0",
            String => "\"\"",
        }
        .to_string()
    }
}

impl From<&RustType> for SwiftArg {
    fn from(rust_type: &RustType) -> Self {
        use TypeKind as K;
        use Value as V;
        match rust_type.kind {
            K::Primitive(_) => Self::Integer,
            K::Value(_) => Self::Nil,
            K::Composite(_, _) => Self::Nil,
            K::Unit => abort!(&rust_type.ident, "Unit cannot be a Swift arg"),
            K::Unknown => {
                abort!(&rust_type.ident, "Unknown cannot be a Swift arg")
            }
        }
    }
}
