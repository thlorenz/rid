use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
};

use quote::quote;

use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Token,
};

#[derive(Debug, PartialEq)]
pub enum TypeCategory {
    Enum,
    Struct,
    Prim,
}

impl TryFrom<&Ident> for TypeCategory {
    type Error = String;

    fn try_from(ident: &Ident) -> Result<Self, Self::Error> {
        use TypeCategory::*;
        match ident.to_string().as_str() {
            "Enum" => Ok(Enum),
            "Struct" => Ok(Struct),
            "Prim" => Ok(Prim),
            x => Err(format!("Unknown type category {:?}", x)),
        }
    }
}

#[derive(Debug)]
pub struct TypeCategoryItem {
    pub key: Ident,
    pub cat: Ident,
}

#[derive(Debug, PartialEq)]
pub struct ValidatedTypeCategoryItem {
    pub key: Ident,
    pub cat: TypeCategory,
}

#[derive(Debug)]
pub struct ExprTypeCategory {
    pub items: HashMap<String, TypeCategoryItem>,
}

impl ExprTypeCategory {
    pub fn into_validated(self) -> Result<HashMap<String, ValidatedTypeCategoryItem>, String> {
        let mut validated: HashMap<String, ValidatedTypeCategoryItem> = HashMap::new();
        for (key, val) in self.items {
            let cat = TypeCategory::try_from(&val.cat)?;
            validated.insert(key, ValidatedTypeCategoryItem { key: val.key, cat });
        }
        Ok(validated)
    }
}

#[derive(Debug)]
struct Types {
    fields: Punctuated<Field, Token![,]>,
}

#[derive(Debug)]
struct Field {
    name: Ident,
    ty: Ident,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Ident = input.parse()?;
        Ok(Field { name, ty })
    }
}

impl Parse for ExprTypeCategory {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let fields: Punctuated<Field, Token![,]> = content.parse_terminated(Field::parse)?;
        let mut items = HashMap::new();
        for field in fields {
            items.insert(
                field.name.to_string(),
                TypeCategoryItem {
                    key: field.name,
                    cat: field.ty,
                },
            );
        }
        Ok(Self { items })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TypeCategory::*;

    use quote::quote;

    #[test]
    fn three_valid_types() {
        let input = quote! {
             { Filter: Enum, MyInt: Prim, Payload: Struct }
        };
        let res = syn::parse2::<ExprTypeCategory>(input).unwrap();
        let ExprTypeCategory { items } = &res;

        assert_eq!(items.len(), 3);
        assert!(items.get("Filter").is_some());
        assert!(items.get("MyInt").is_some());
        assert!(items.get("Payload").is_some());

        let validated = res.into_validated().unwrap();
        assert_eq!(validated.len(), 3);
        assert_eq!(validated.get("Filter").unwrap().cat, Enum);
        assert_eq!(validated.get("MyInt").unwrap().cat, Prim);
        assert_eq!(validated.get("Payload").unwrap().cat, Struct);
    }

    #[test]
    fn one_unknown_type() {
        let input = quote! {
             { Filter: Invalid }
        };
        let res = syn::parse2::<ExprTypeCategory>(input).unwrap();
        let ExprTypeCategory { items } = &res;

        assert_eq!(items.len(), 1);
        assert!(items.get("Filter").is_some());

        let validated = res.into_validated();
        if let Err(err) = validated {
            assert!(err.to_string().starts_with("Unknown type category"))
        } else {
            assert!(false, "should have errored");
        }
    }

    #[test]
    fn one_valid_one_invalid_type() {
        let input = quote! {
             { Payload: enum, Filter: Invalid }
        };
        let res = syn::parse2::<ExprTypeCategory>(input);
        if let Err(err) = res {
            eprintln!("{:#?}", err);
            assert_eq!(err.to_string(), "expected identifier");
        } else {
            assert!(false, "should have errored");
        }
    }
}
