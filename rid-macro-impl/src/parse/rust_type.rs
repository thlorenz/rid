use quote::{format_ident, quote_spanned};
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Ident, Path,
    PathArguments, PathSegment, Type, TypePath,
};

use std::fmt::Debug;

use crate::attrs::{raw_typedef_ident, Category, TypeInfo, TypeInfoMap};

use super::ParsedReference;

#[derive(Debug, Clone, PartialEq)]
pub struct RustType {
    ident: Ident,
    raw_ident: Ident,

    /// If a type alias is needed, namely if the ident differs from the rust ident
    /// This is currently the case for structs and vecs
    pub needs_type_alias: bool,

    pub kind: TypeKind,
    pub reference: ParsedReference,
}

impl RustType {
    pub fn new(
        ident: Ident,
        kind: TypeKind,
        reference: ParsedReference,
    ) -> Self {
        let raw_ident = raw_typedef_ident(&ident);
        let needs_type_alias = kind.is_struct() || kind.is_vec();
        Self {
            ident,
            raw_ident,
            needs_type_alias,
            kind,
            reference,
        }
    }

    /// Ident that should be used inside generated Rust/Dart wrapper methods
    pub fn ident(&self) -> &Ident {
        if self.needs_type_alias {
            &self.raw_ident
        } else {
            &self.ident
        }
    }

    /// Ident that came directly from the annotated Rust code
    pub fn rust_ident(&self) -> &Ident {
        &self.ident
    }

    /// Used at this point only to name Dart presentations of Rust enums
    pub fn dart_ident(&self, prefix: bool) -> Ident {
        if prefix {
            format_ident!("Rid{}", self.ident())
        } else {
            self.ident().clone()
        }
    }

    pub fn from_owned_struct(ident: &Ident) -> Self {
        let type_info = TypeInfo {
            cat: Category::Struct,
            key: ident.clone(),
            typedef: Some(raw_typedef_ident(&ident)),
        };
        let value = Value::Custom(type_info, ident.to_string());
        let kind = TypeKind::Value(value);
        let reference = ParsedReference::Owned;
        Self::new(ident.clone(), kind, reference)
    }

    pub fn from_owned_enum(ident: &Ident) -> Self {
        let type_info = TypeInfo {
            cat: Category::Enum,
            key: ident.clone(),
            typedef: None,
        };
        let value = Value::Custom(type_info, ident.to_string());
        let kind = TypeKind::Value(value);
        let reference = ParsedReference::Owned;
        Self::new(ident.clone(), kind, reference)
    }

    pub fn self_unaliased(self, owner_name: String) -> Self {
        RustType::new(
            self.ident,
            self.kind.self_unaliased(owner_name),
            self.reference,
        )
    }

    pub fn with_lifetime_option(self, lifetime: Option<Ident>) -> Self {
        match lifetime {
            Some(lifetime) => self.with_lifetime(lifetime),
            None => self,
        }
    }

    pub fn with_lifetime(self, lifetime: Ident) -> Self {
        RustType::new(
            self.ident,
            self.kind,
            self.reference.with_lifetime(lifetime),
        )
    }

    pub fn ensured_lifetime(self, lifetime: Ident) -> Self {
        RustType::new(
            self.ident,
            self.kind,
            self.reference.ensured_lifetime(lifetime),
        )
    }

    pub fn is_primitive(&self) -> bool {
        self.kind.is_primitive()
    }

    pub fn is_composite(&self) -> bool {
        self.kind.is_composite()
    }

    pub fn is_vec(&self) -> bool {
        self.kind.is_vec()
    }

    pub fn inner_composite_type(&self) -> Option<RustType> {
        self.kind.inner_composite_rust_type()
    }

    pub fn is_enum(&self) -> bool {
        self.kind.is_enum()
    }
}

// --------------
// TypeKind
// --------------
#[derive(Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    Value(Value),
    Composite(Composite, Option<Box<RustType>>),
    Unit,
    Unknown,
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Primitive(prim1), TypeKind::Primitive(prim2)) => {
                prim1 == prim2
            }
            (TypeKind::Value(val1), TypeKind::Value(val2)) => val1 == val2,
            (
                TypeKind::Composite(com1, ty1),
                TypeKind::Composite(com2, ty2),
            ) => com1 == com2 && ty1 == ty2,
            (TypeKind::Unit, TypeKind::Unit) => true,
            (TypeKind::Unknown, TypeKind::Unknown) => true,
            _ => false,
        }
    }
}

impl Debug for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            TypeKind::Primitive(p) => format!("TypeKind::Primitive({:?})", p),
            TypeKind::Value(val) => format!("TypeKind::Value({:?})", val),
            TypeKind::Composite(com, inner) => {
                format!("TypeKind::Composite({:?}, {:?})", com, inner)
            }
            TypeKind::Unit => "TypeKind::Unit".to_string(),
            TypeKind::Unknown => "TypeKind::Unknown".to_string(),
        };
        write!(f, "{}", kind)
    }
}

impl TypeKind {
    fn self_unaliased(self, owner_name: String) -> Self {
        match self {
            TypeKind::Value(val) => Self::Value(val.self_unaliased(owner_name)),
            _ => self,
        }
    }

    pub fn is_primitive(&self) -> bool {
        if let TypeKind::Primitive(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_composite(&self) -> bool {
        if let TypeKind::Composite(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_vec(&self) -> bool {
        if let TypeKind::Composite(Composite::Vec, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_option(&self) -> bool {
        if let TypeKind::Composite(Composite::Option, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_enum(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_enum()
        } else {
            false
        }
    }

    pub fn is_struct(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_struct()
        } else {
            false
        }
    }

    pub fn inner_composite_rust_type(&self) -> Option<RustType> {
        match self {
            TypeKind::Primitive(_) => None,
            TypeKind::Value(_) => None,
            TypeKind::Composite(Composite::Vec, inner) => {
                inner.as_ref().map(|x| (*x.clone()))
            }
            TypeKind::Composite(_, _) => None,
            TypeKind::Unit => None,
            TypeKind::Unknown => None,
        }
    }
}

// --------------
// Primitive
// --------------
#[derive(Clone, PartialEq)]
pub enum Primitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    USize,
    Bool,
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            Primitive::U8 => "Primitive::U8",
            Primitive::I8 => "Primitive::I8",
            Primitive::U16 => "Primitive::U16",
            Primitive::I16 => "Primitive::I16",
            Primitive::U32 => "Primitive::U32",
            Primitive::I32 => "Primitive::I32",
            Primitive::U64 => "Primitive::U64",
            Primitive::I64 => "Primitive::I64",
            Primitive::USize => "Primitive::Usize",
            Primitive::Bool => "Primitive::Bool",
        };
        write!(f, "{}", ty)
    }
}

// --------------
// Value
// --------------
#[derive(Clone, PartialEq)]
pub enum Value {
    CString,
    String,
    Str,
    Custom(TypeInfo, String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::CString => write!(f, "Value::CString"),
            Value::Str => write!(f, "Value::Str"),
            Value::String => write!(f, "Value::String"),
            Value::Custom(type_info, name) => {
                write!(f, "Value::Custom({:?}, \"{}\")", type_info, name)
            }
        }
    }
}

impl Value {
    fn self_unaliased(self, owner_name: String) -> Self {
        match self {
            Value::Custom(type_info, name) if name == "Self" => {
                Self::Custom(type_info, owner_name)
            }
            _ => self,
        }
    }

    fn is_enum(&self) -> bool {
        use Value::*;
        match self {
            CString | String | Str => false,
            Custom(type_info, _) => type_info.cat == Category::Enum,
        }
    }

    fn is_struct(&self) -> bool {
        use Value::*;
        match self {
            CString | String | Str => false,
            Custom(type_info, _) => type_info.cat == Category::Struct,
        }
    }
}

// --------------
// Composite
// --------------
#[derive(Clone, PartialEq)]
pub enum Composite {
    Vec,
    Option,
    Custom(TypeInfo, String),
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Composite::Vec => write!(f, "Composite::Vec"),
            Composite::Option => write!(f, "Composite::Option"),
            Composite::Custom(type_info, name) => {
                write!(f, "Composite::Custom({:?}, \"{}\")", type_info, name)
            }
        }
    }
}

// --------------
// RustType Impl
// --------------
impl RustType {
    pub fn from_boxed_type(
        ty: Box<Type>,
        type_infos: &TypeInfoMap,
    ) -> Option<RustType> {
        resolve_rust_ty(ty.as_ref(), type_infos)
    }

    pub fn from_type(ty: &Type, type_infos: &TypeInfoMap) -> Option<RustType> {
        resolve_rust_ty(ty, type_infos)
    }

    pub fn from_plain_type(ty: &Type) -> Option<RustType> {
        resolve_rust_ty(ty, &TypeInfoMap::default())
    }
}

fn resolve_rust_ty(ty: &Type, type_infos: &TypeInfoMap) -> Option<RustType> {
    let (ty, reference) = match ty {
        Type::Reference(r) => {
            let pr = ParsedReference::from(r);
            (r.elem.as_ref(), pr)
        }
        Type::Path(_) => (ty, ParsedReference::Owned),
        _ => return None,
    };

    let (ident, kind) = match ty {
        Type::Path(TypePath { path, .. }) => {
            let PathSegment {
                ident, arguments, ..
            } = path.segments.first().unwrap();
            (ident, ident_to_kind(ident, arguments, type_infos))
        }
        _ => return None,
    };

    Some(RustType::new(ident.clone(), kind, reference))
}

fn ident_to_kind(
    ident: &Ident,
    arguments: &PathArguments,
    type_infos: &TypeInfoMap,
) -> TypeKind {
    let ident_str = ident.to_string();

    match arguments {
        // Non Composite Types
        PathArguments::None => {
            // primitives
            match ident_str.as_str() {
                "u8" => return TypeKind::Primitive(Primitive::U8),
                "i8" => return TypeKind::Primitive(Primitive::I8),
                "u16" => return TypeKind::Primitive(Primitive::U16),
                "i16" => return TypeKind::Primitive(Primitive::I16),
                "u32" => return TypeKind::Primitive(Primitive::U32),
                "i32" => return TypeKind::Primitive(Primitive::I32),
                "u64" => return TypeKind::Primitive(Primitive::U64),
                "i64" => return TypeKind::Primitive(Primitive::I64),
                "usize" => return TypeKind::Primitive(Primitive::USize),
                "bool" => return TypeKind::Primitive(Primitive::Bool),
                _ => {}
            };

            // known value types
            match ident_str.as_str() {
                "String" => return TypeKind::Value(Value::String),
                "CString" => return TypeKind::Value(Value::CString),
                "str" => return TypeKind::Value(Value::Str),
                _ => {}
            }

            // custom value types
            if let Some(type_info) = type_infos.get(&ident_str) {
                return TypeKind::Value(Value::Custom(
                    type_info.clone(),
                    ident_str.clone(),
                ));
            }

            TypeKind::Unknown
        }

        // Composite Types
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            args,
            ..
        }) => {
            // For now assuming one arg
            match &args[0] {
                GenericArgument::Type(ty) => {
                    let inner =
                        resolve_rust_ty(ty, type_infos).map(|x| Box::new(x));
                    match ident_str.as_str() {
                        "Vec" => TypeKind::Composite(Composite::Vec, inner),
                        "Option" => {
                            TypeKind::Composite(Composite::Option, inner)
                        }
                        _ => {
                            if let Some(type_info) = type_infos.get(&ident_str)
                            {
                                TypeKind::Composite(
                                    Composite::Custom(
                                        type_info.clone(),
                                        ident_str.clone(),
                                    ),
                                    inner,
                                )
                            } else {
                                TypeKind::Unknown
                            }
                        }
                    }
                }
                _ => TypeKind::Unknown,
            }
        }
        PathArguments::Parenthesized(args) => {
            todo!(
                "rust_type::ident_to_kind PathArguments::Parenthesized {:#?}",
                args
            )
        }
    }
}
