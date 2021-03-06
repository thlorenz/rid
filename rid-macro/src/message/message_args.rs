#[derive(Debug)]
pub struct MessageArgs {
    pub to: String,
}

impl std::convert::TryFrom<Vec<syn::Attribute>> for MessageArgs {
    type Error = Vec<String>;

    fn try_from(attrs: Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        use proc_macro2::TokenTree::*;

        let mut to: Option<String> = None;
        let mut errors: Vec<String> = vec![];
        for attr in attrs {
            match attr {
                syn::Attribute { path, tokens, .. } => {
                    if path.is_ident("rid") {
                        for token in tokens {
                            if let Group(group) = token {
                                let mut stream = group.stream().into_iter();
                                let (key, eq, val) = (stream.next(), stream.next(), stream.next());
                                match (key, eq, val) {
                                    (Some(Ident(key)), Some(Punct(_)), Some(Literal(val))) => {
                                        if key == "to" {
                                            // Remove quotes, this hopefully will go away once we
                                            // figure out how to avoid them
                                            let val = val.to_string();
                                            let val = val.trim_matches('"');
                                            to = Some(val.to_string());
                                        };
                                    }
                                    _ => {}
                                }
                            };
                        }
                    }
                }
            }
            if to.is_some() {
                break;
            }
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
