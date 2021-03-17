use proc_macro_error::abort;

use super::RidAttr;

#[derive(Debug)]
pub struct StructConfig {
    pub debug: bool,
}

impl StructConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        use RidAttr::*;
        let mut debug = false;
        for attr in attrs {
            match attr {
                Debug(_) => debug = true,
                Model(ident, _) => {
                    abort!(ident, "model can only be set on the message enum")
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
        Self { debug }
    }
}
