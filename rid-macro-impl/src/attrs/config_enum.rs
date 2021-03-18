use crate::common::abort;

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
        use RidAttr::*;
        let mut to: Option<String> = None;
        for attr in attrs {
            match attr {
                Debug(ident) => {
                    abort!(ident, "debug can only be exposed for model structs")
                }
                Model(_, model) => {
                    let ident = extract_ident(model);
                    match ident {
                        Some(ident) => to = Some(ident.to_string()),
                        None => abort!(
                            ident,
                            "Arg 'to' is assigned incorrectly, use #[rid(to = Model)] instead."
                        ),
                    };
                }
                Types(ident, _) => {
                    abort!(ident, "types can only be set on fields")
                }
                Export(ident) => abort!(
                    ident,
                    "export can only be applied to functions and struct impl blocks"
                ),
                Wip => {}
            }
        }

        match to {
            Some(to) => Self { to },
            None => abort!(
                enum_ident,
                "Arg 'to' is required on Messages, i.e. #[rid(to = Model)]."
            ),
        }
    }
}
