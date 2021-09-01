use quote::{format_ident, quote_spanned};
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Ident, Path,
    PathArguments, PathSegment, Type, TypePath,
};

use std::fmt::Debug;

use crate::attrs::{raw_typedef_ident, Category, TypeInfo, TypeInfoMap};

use super::ParsedReference;

#[derive(Debug, Clone, PartialEq)]
pub enum RustTypeContext {
    Default,
    CollectionItem,
    OptionItem,
    CustomItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustType {
    ident: Ident,
    raw_ident: Ident,

    /// If a type alias is needed, namely if the ident differs from the rust ident
    /// This is currently the case for structs and vecs
    pub needs_type_alias: bool,

    pub kind: TypeKind,
    pub reference: ParsedReference,

    /// The context of the type, i.e. is it an inner type of `Vec<ty>` and thus a
    /// CollectionInnerType
    pub context: RustTypeContext,
}

impl RustType {
    pub fn new(
        ident: Ident,
        kind: TypeKind,
        reference: ParsedReference,
        context: RustTypeContext,
    ) -> Self {
        let raw_ident = raw_typedef_ident(&ident);
        let needs_type_alias = kind.is_struct() || kind.is_vec();
        Self {
            ident,
            raw_ident,
            needs_type_alias,
            kind,
            reference,
            context,
        }
    }

    /// Ident that should be used inside generated Dart wrapper methods
    /// For structs this is `ident` prefixed with `Raw` and equal to `ident` for all
    /// else.
    pub fn dart_wrapper_rust_string(&self) -> String {
        if self.needs_type_alias {
            &self.raw_ident
        } else {
            &self.ident
        }
        .to_string()
    }

    /// Ident that came directly from the annotated Rust code
    pub fn rust_ident(&self) -> &Ident {
        &self.ident
    }

    /// Same as [RustType::rust_ident()] but qualifies namespace where necessary
    pub fn fully_qualified_rust_ident(&self) -> String {
        match self.rust_ident().to_string().as_str() {
            "CString" => "::std::ffi::CString".to_string(),
            _ => self.rust_ident().to_string(),
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
        Self::new(ident.clone(), kind, reference, RustTypeContext::Default)
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
        Self::new(ident.clone(), kind, reference, RustTypeContext::Default)
    }

    pub fn self_unaliased(self, owner_name: String) -> Self {
        RustType::new(
            self.ident,
            self.kind.self_unaliased(owner_name),
            self.reference,
            self.context,
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
            self.context,
        )
    }

    pub fn ensured_lifetime(self, lifetime: Ident) -> Self {
        RustType::new(
            self.ident,
            self.kind,
            self.reference.ensured_lifetime(lifetime),
            self.context,
        )
    }

    pub fn from_vec_with_pointer_alias(
        inner_poiner_alias_ident: &Ident,
        inner_reference: ParsedReference,
    ) -> Self {
        let inner_type = Self::from_pointer_alias(
            inner_poiner_alias_ident,
            inner_reference,
            RustTypeContext::CollectionItem,
        );
        Self::from_vec(inner_type, ParsedReference::Owned)
    }

    pub fn from_pointer_alias(
        ident: &Ident,
        reference: ParsedReference,
        context: RustTypeContext,
    ) -> Self {
        // TODO(thlorenz): should be Category::Pointer and/or do we need TypeKind::Pointer?
        let type_info = TypeInfo {
            cat: Category::Prim,
            key: ident.clone(),
            typedef: Some(ident.clone()),
        };

        let value = Value::Custom(type_info, ident.to_string());
        let kind = TypeKind::Value(value);

        Self::new(ident.clone(), kind, reference, context)
    }

    pub fn from_vec(inner_type: RustType, reference: ParsedReference) -> Self {
        let ident = format_ident!("Vec");
        let kind = TypeKind::Composite(
            Composite::Vec,
            Some(Box::new(inner_type)),
            None,
        );
        let context = RustTypeContext::Default;
        Self::new(ident, kind, reference, context)
    }

    pub fn is_primitive(&self) -> bool {
        self.kind.is_primitive()
    }

    pub fn is_string(&self) -> bool {
        self.kind.is_string()
    }

    pub fn is_cstring(&self) -> bool {
        self.kind.is_cstring()
    }

    pub fn is_str(&self) -> bool {
        self.kind.is_str()
    }

    pub fn is_string_like(&self) -> bool {
        self.kind.is_string_like()
    }

    pub fn is_composite(&self) -> bool {
        self.kind.is_composite()
    }

    pub fn is_struct(&self) -> bool {
        self.kind.is_struct()
    }

    pub fn is_vec(&self) -> bool {
        self.kind.is_vec()
    }

    pub fn is_collection_item(&self) -> bool {
        self.context == RustTypeContext::CollectionItem
    }

    pub fn inner_composite_type(&self) -> Option<RustType> {
        self.kind.inner_composite_rust_type()
    }

    pub fn key_val_composite_types(&self) -> Option<(RustType, RustType)> {
        self.kind.key_val_composite_rust_types()
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
    Composite(Composite, Option<Box<RustType>>, Option<Box<RustType>>),
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
                TypeKind::Composite(com1, first_ty1, second_ty1),
                TypeKind::Composite(com2, first_ty2, second_ty2),
            ) => {
                com1 == com2
                    && first_ty1 == first_ty2
                    && second_ty1 == second_ty2
            }
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
            TypeKind::Composite(com, fst_inner, snd_inner) => {
                format!(
                    "TypeKind::Composite({:?}, {:?}, {:?})",
                    com, fst_inner, snd_inner
                )
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

    pub fn is_string(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_string()
        } else {
            false
        }
    }

    pub fn is_cstring(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_cstring()
        } else {
            false
        }
    }

    pub fn is_str(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_str()
        } else {
            false
        }
    }

    pub fn is_string_like(&self) -> bool {
        if let TypeKind::Value(val) = self {
            val.is_string_like()
        } else {
            false
        }
    }

    pub fn is_composite(&self) -> bool {
        if let TypeKind::Composite(_, _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_vec(&self) -> bool {
        if let TypeKind::Composite(Composite::Vec, _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_option(&self) -> bool {
        if let TypeKind::Composite(Composite::Option, _, _) = self {
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
            TypeKind::Composite(Composite::Vec, inner, _) => {
                inner.as_ref().map(|x| (*x.clone()))
            }
            TypeKind::Composite(_, _, _) => None,
            TypeKind::Unit => None,
            TypeKind::Unknown => None,
        }
    }

    pub fn key_val_composite_rust_types(&self) -> Option<(RustType, RustType)> {
        match self {
            TypeKind::Primitive(_) => None,
            TypeKind::Value(_) => None,
            TypeKind::Composite(Composite::HashMap, key_ty, val_ty) => {
                let key = key_ty
                    .as_ref()
                    .map(|x| *x.clone())
                    .expect("hashmap should have key type");
                let val = val_ty
                    .as_ref()
                    .map(|x| *x.clone())
                    .expect("hashmap should have val type");

                Some((key, val))
            }
            TypeKind::Composite(_, _, _) => None,
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

    fn is_string_like(&self) -> bool {
        use Value::*;
        match self {
            CString | String | Str => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        use Value::*;
        match self {
            String => true,
            _ => false,
        }
    }

    fn is_cstring(&self) -> bool {
        use Value::*;
        match self {
            CString => true,
            _ => false,
        }
    }

    fn is_str(&self) -> bool {
        use Value::*;
        match self {
            Str => true,
            _ => false,
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
    HashMap,
    Custom(TypeInfo, String),
}

impl Debug for Composite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Composite::Vec => write!(f, "Composite::Vec"),
            Composite::Option => write!(f, "Composite::Option"),
            Composite::HashMap => write!(f, "Composite::HashMap"),
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
        resolve_rust_ty(ty.as_ref(), type_infos, RustTypeContext::Default)
    }

    pub fn from_type(ty: &Type, type_infos: &TypeInfoMap) -> Option<RustType> {
        resolve_rust_ty(ty, type_infos, RustTypeContext::Default)
    }

    pub fn from_plain_type(ty: &Type) -> Option<RustType> {
        resolve_rust_ty(ty, &TypeInfoMap::default(), RustTypeContext::Default)
    }
}

fn resolve_rust_ty(
    ty: &Type,
    type_infos: &TypeInfoMap,
    context: RustTypeContext,
) -> Option<RustType> {
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

    Some(RustType::new(ident.clone(), kind, reference, context))
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
                    match ident_str.as_str() {
                        "Vec" => {
                            let inner = resolve_rust_ty(
                                ty,
                                type_infos,
                                RustTypeContext::CollectionItem,
                            )
                            .map(|x| Box::new(x));
                            TypeKind::Composite(Composite::Vec, inner, None)
                        }
                        "Option" => {
                            let inner = resolve_rust_ty(
                                ty,
                                type_infos,
                                RustTypeContext::OptionItem,
                            )
                            .map(|x| Box::new(x));
                            TypeKind::Composite(Composite::Option, inner, None)
                        }
                        "HashMap" => {
                            let inner = resolve_rust_ty(
                                ty,
                                type_infos,
                                RustTypeContext::CollectionItem,
                            )
                            .map(|x| Box::new(x));
                            // TODO(thlorenz): HashMap needs two inner types
                            TypeKind::Composite(
                                Composite::HashMap,
                                inner.clone(),
                                inner,
                            )
                        }
                        _ => {
                            if let Some(type_info) = type_infos.get(&ident_str)
                            {
                                let inner = resolve_rust_ty(
                                    ty,
                                    type_infos,
                                    RustTypeContext::CustomItem,
                                )
                                .map(|x| Box::new(x));

                                TypeKind::Composite(
                                    Composite::Custom(
                                        type_info.clone(),
                                        ident_str.clone(),
                                    ),
                                    inner,
                                    None,
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
