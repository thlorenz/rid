use proc_macro_error::abort;

use super::RidAttr;

#[derive(Debug)]
pub struct StructConfig {
    pub debug: bool,
}

impl StructConfig {
    pub fn new(attrs: &[RidAttr]) -> Self {
        let mut debug = false;
        for attr in attrs {
            match attr {
                RidAttr::Debug(_) => debug = true,
                RidAttr::Model(ident, _) => {
                    abort!(ident, "model can only be set on the message enum")
                }
            }
        }
        Self { debug }
    }
}
