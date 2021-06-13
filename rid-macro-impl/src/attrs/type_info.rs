use crate::common::abort;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use quote::format_ident;
use syn::Ident;

#[derive(PartialEq, Clone)]
pub enum Category {
    Enum,
    Struct,
    Prim,
}

pub fn add_idents_to_type_map(
    type_infos: &mut TypeInfoMap,
    cat: Category,
    idents: &[Ident],
) {
    for ident in idents {
        let typedef = if cat == Category::Struct {
            Some(raw_typedef_ident(&ident))
        } else {
            None
        };
        type_infos.insert(
            ident.to_string(),
            TypeInfo {
                key: ident.clone(),
                cat: cat.clone(),
                typedef,
            },
        );
    }
}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Enum => write!(f, "Category::Enum"),
            Category::Struct => write!(f, "Category::Struct"),
            Category::Prim => write!(f, "Category::Prim"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeInfo {
    pub key: Ident,
    pub cat: Category,
    pub typedef: Option<Ident>,
}

impl TypeInfo {
    pub fn is_self(&self) -> bool {
        self.cat == Category::Struct && self.key.to_string() == "Self"
    }
}

impl From<(&str, Category)> for TypeInfo {
    fn from((name, cat): (&str, Category)) -> Self {
        let ident = Ident::new(name, proc_macro2::Span::mixed_site());
        let typedef = if cat == Category::Struct {
            Some(raw_typedef_ident(&ident))
        } else {
            None
        };
        Self {
            key: ident,
            cat,
            typedef,
        }
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

impl Default for TypeInfoMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

pub fn raw_typedef_ident(ident: &Ident) -> Ident {
    format_ident!("Raw{}", ident)
}
