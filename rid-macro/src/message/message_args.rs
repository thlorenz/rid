#[derive(Debug)]
pub struct MessageArgs {
    pub to: String,
}

impl std::convert::TryFrom<Vec<syn::NestedMeta>> for MessageArgs {
    type Error = Vec<String>;

    fn try_from(args: Vec<syn::NestedMeta>) -> Result<Self, Self::Error> {
        let mut to: Option<String> = None;
        let mut errors: Vec<String> = vec![];
        for meta in &args {
            match meta {
                syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) => {
                    if path.is_ident("to") {
                        // TODO: error handling
                        let lit: syn::Path = lit_str.parse().unwrap();
                        to = Some(lit.get_ident().unwrap().to_string());
                    } else {
                        errors.push(format!(
                            "Arg '{}' is not supported on 'rid::message' attribute.",
                            path.get_ident().unwrap()
                        ));
                    }
                }
                _ => {}
            };
        }
        if to.is_none() {
            errors.push("Arg 'to' is required, i.e. #[rid::message(to = Model)]".to_string());
        }

        if errors.is_empty() {
            Ok(Self { to: to.unwrap() })
        } else {
            Err(errors)
        }
    }
}
