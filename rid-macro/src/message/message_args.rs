#[derive(Debug)]
pub struct MessageArgs {
    pub to: syn::Ident,
}

impl std::convert::TryFrom<Vec<syn::NestedMeta>> for MessageArgs {
    type Error = Vec<String>;

    fn try_from(args: Vec<syn::NestedMeta>) -> Result<Self, Self::Error> {
        let mut to: Option<syn::Ident> = None;
        let mut errors: Vec<String> = vec![];
        for meta in &args {
            match meta {
                syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) => {
                    if path.is_ident("to") {
                        let lit: syn::Result<syn::Path> = lit_str.parse();

                        let path = match &lit {
                            Ok(lit) => lit.get_ident(),
                            Err(err) => {
                                errors.push(err.to_string());
                                None
                            }
                        };
                        if to.is_none() {
                            to = path.map(|x| x.clone());
                        }
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
            errors.push("Arg 'to' is required, i.e. #[rid::message(to = \"Model\")].".to_string());
        }

        if errors.is_empty() {
            Ok(Self { to: to.unwrap() })
        } else {
            Err(errors)
        }
    }
}
