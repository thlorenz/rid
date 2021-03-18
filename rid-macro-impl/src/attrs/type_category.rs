use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    ops::{Deref, DerefMut},
};

use quote::quote;

use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Token,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Category {
    Enum,
    Struct,
    Prim,
}

impl TryFrom<&Ident> for Category {
    type Error = String;

    fn try_from(ident: &Ident) -> Result<Self, Self::Error> {
        use Category::*;
        match ident.to_string().as_str() {
            "Enum" => Ok(Enum),
            "Struct" => Ok(Struct),
            "Prim" => Ok(Prim),
            x => Err(format!("Unknown type category {:?}", x)),
        }
    }
}

#[derive(Debug)]
pub struct UnvalidatedTypeCategoryInfo {
    pub key: Ident,
    pub cat: Ident,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeInfo {
    pub key: Ident,
    pub cat: Category,
}

impl TypeInfo {
    pub fn is_self(&self) -> bool {
        self.cat == Category::Struct && self.key.to_string() == "Self"
    }
}

#[derive(Debug)]
pub struct TypeInfoMap(pub HashMap<String, TypeInfo>);
impl Deref for TypeInfoMap {
    type Target = HashMap<String, TypeInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TypeInfoMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct ExprTypeInfo {
    pub items: HashMap<String, UnvalidatedTypeCategoryInfo>,
}

impl ExprTypeInfo {
    pub fn into_validated(self) -> Result<TypeInfoMap, String> {
        let mut validated: TypeInfoMap = TypeInfoMap(HashMap::new());
        for (key, val) in self.items {
            let cat = Category::try_from(&val.cat)?;
            validated.insert(key, TypeInfo { key: val.key, cat });
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

impl Parse for ExprTypeInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let fields: Punctuated<Field, Token![,]> = content.parse_terminated(Field::parse)?;
        let mut items = HashMap::new();
        for field in fields {
            items.insert(
                field.name.to_string(),
                UnvalidatedTypeCategoryInfo {
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
    use Category::*;

    use quote::quote;

    #[test]
    fn three_valid_types() {
        let input = quote! {
             { Filter: Enum, MyInt: Prim, Payload: Struct }
        };
        let res = syn::parse2::<ExprTypeInfo>(input).unwrap();
        let ExprTypeInfo { items } = &res;

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
        let res = syn::parse2::<ExprTypeInfo>(input).unwrap();
        let ExprTypeInfo { items } = &res;

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
        let res = syn::parse2::<ExprTypeInfo>(input);
        if let Err(err) = res {
            assert_eq!(err.to_string(), "expected identifier");
        } else {
            assert!(false, "should have errored");
        }
    }
}
