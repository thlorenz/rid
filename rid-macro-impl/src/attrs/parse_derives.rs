use syn::{
    punctuated::Punctuated, Attribute, Meta, MetaList, NestedMeta, Path,
    PathSegment, Token,
};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Derive {
    pub debug: bool,
    pub clone: bool,
}

impl Default for Derive {
    fn default() -> Self {
        Self {
            debug: false,
            clone: false,
        }
    }
}

pub fn parse_derive_attrs(all_attrs: &[Attribute]) -> Derive {
    all_attrs.iter().flat_map(parse_derive).fold(
        Derive::default(),
        |acc, derive| Derive {
            debug: acc.debug || derive.debug,
            clone: acc.clone || derive.clone,
        },
    )
}

fn parse_segments(
    segments: &Punctuated<PathSegment, Token![::]>,
    nested: Option<&Punctuated<NestedMeta, Token![,]>>,
) -> Option<Derive> {
    match (segments.first(), nested) {
        (Some(PathSegment { ident, .. }), Some(nested))
            if ident.to_string().as_str() == "derive" =>
        {
            let mut derive: Derive = Default::default();
            for meta in nested {
                match meta {
                    NestedMeta::Meta(Meta::Path(Path { segments, .. })) => {
                        match segments
                            .first()
                            .unwrap()
                            .ident
                            .to_string()
                            .as_str()
                        {
                            "Debug" => derive.debug = true,
                            "Clone" => derive.clone = true,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Some(derive)
        }
        _ => None,
    }
}

fn parse_derive(attr: &Attribute) -> Option<Derive> {
    match attr.parse_meta() {
        Ok(Meta::List(MetaList {
            path: Path { segments, .. },
            nested,
            ..
        })) => parse_segments(&segments, Some(&nested)),
        _ => None,
    }
}
