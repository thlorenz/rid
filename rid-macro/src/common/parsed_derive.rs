#[derive(Debug)]
pub struct ParsedDerive {
    pub debug: bool,
}

impl ParsedDerive {
    pub fn from_attrs(attr: &Vec<syn::Attribute>) -> ParsedDerive {
        attr.iter().flat_map(ParsedDerive::from_attr).fold(
            ParsedDerive { debug: false },
            |acc, x| ParsedDerive {
                debug: acc.debug || x.debug,
            },
        )
    }

    fn from_attr(attr: &syn::Attribute) -> Option<ParsedDerive> {
        use syn::*;

        let meta = attr.parse_meta();
        match meta {
            Ok(Meta::List(MetaList {
                path: Path { segments, .. },
                nested,
                ..
            })) => {
                let is_rid = match segments.first() {
                    Some(segment) if segment.ident == "rid" => true,
                    _ => false,
                };
                if !is_rid {
                    return None;
                }
                let mut debug: bool = false;
                for x in nested {
                    if let NestedMeta::Meta(Meta::Path(Path { segments, .. })) = x {
                        for segment in segments {
                            match segment.ident.to_string().as_str() {
                                "debug" => debug = true,
                                _ => {}
                            }
                        }
                    }
                }
                Some(ParsedDerive { debug })
            }
            Err(_) => None,
            _ => None,
        }
    }
}
