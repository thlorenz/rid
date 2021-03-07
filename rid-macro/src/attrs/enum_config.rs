use proc_macro_error::abort;

use super::RidAttr;

#[derive(Debug)]
pub struct EnumConfig {
    pub to: String,
}

fn extract_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    use syn::*;
    match expr {
        Expr::Path(ExprPath { path, .. }) => path.get_ident(),
        _ => None,
    }
}

impl EnumConfig {
    pub fn new(enum_ident: &syn::Ident, attrs: &[RidAttr]) -> Self {
        let mut to: Option<String> = None;
        for attr in attrs {
            match attr {
                RidAttr::Debug(ident) => {
                    abort!(ident, "debug can only be exposed for model structs")
                }
                RidAttr::Model(_, model) => {
                    let ident = extract_ident(model);
                    match ident {
                        Some(ident) => to = Some(ident.to_string()),
                        None => abort!(ident,
                            "Arg 'to' is assigned incorrectly, use #[rid::message(to = Model)] instead."
                        ),
                    };
                }
            }
        }

        match to {
            Some(to) => Self { to },
            None => abort!(
                enum_ident,
                "Arg 'to' is required, i.e. #[rid::message(to = Model)]."
            ),
        }
    }
}
