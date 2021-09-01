use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};

use crate::attrs::TypeInfoMap;

use super::{HashMapAccess, PointerTypeAlias, VecAccess};

/// Distinguishes between accesses that are references to fields on structs or enums vs.
/// instances created during a method call and returned to Dart without keeping a reference
/// on the Rust side.
#[derive(PartialEq)]
pub enum AccessKind {
    /// Type is a reference to a field held onto by Rust
    FieldReference,
    /// Type is instantiated inside a method and returned i.e. as RidVec for Vecs, not held onto by
    /// Rust. This is the case for [rid::export]
    MethodReturn,
}

pub enum AccessRender {
    Force,
    Omit,
    Default,
}

pub struct RenderedAccessRust {
    /// Rendered access
    pub tokens: TokenStream,

    /// Type aliases used to render access for which a typedef needs to be rendered
    pub type_aliases: HashMap<String, PointerTypeAlias>,
}

// -----------------
// Renderable Access trait and implmentations
// -----------------
pub trait RenderableAccess {
    fn render_rust(&self) -> RenderedAccessRust;
    fn render_dart(&self, type_infos: &TypeInfoMap, comment: &str) -> String;
    fn key(&self) -> String;
    fn span(&self) -> Span;
}

impl RenderableAccess for VecAccess {
    fn render_rust(&self) -> RenderedAccessRust {
        match self.kind {
            AccessKind::FieldReference => self.render_rust_field_access(),
            AccessKind::MethodReturn => self.render_rust_method_return(),
        }
    }

    fn render_dart(&self, type_infos: &TypeInfoMap, comment: &str) -> String {
        match self.kind {
            AccessKind::FieldReference => {
                self.render_dart_for_field_reference(type_infos, comment)
            }
            AccessKind::MethodReturn => {
                self.render_dart_return_from_method(type_infos, comment)
            }
        }
    }

    fn key(&self) -> String {
        VecAccess::key_from_item_rust_ident(
            self.item_type.rust_ident(),
            &self.kind,
        )
    }

    fn span(&self) -> Span {
        self.vec_type_ident.span()
    }
}

impl RenderableAccess for HashMapAccess {
    fn render_rust(&self) -> RenderedAccessRust {
        match self.kind {
            AccessKind::FieldReference => self.render_rust_field_access(),
            AccessKind::MethodReturn => self.render_rust_method_return(),
        }
    }

    fn render_dart(&self, type_infos: &TypeInfoMap, comment: &str) -> String {
        match self.kind {
            AccessKind::FieldReference => {
                self.render_dart_for_field_reference(type_infos, comment)
            }
            AccessKind::MethodReturn => {
                self.render_dart_return_from_method(type_infos, comment)
            }
        }
    }

    fn key(&self) -> String {
        Self::key_from_item_rust_ident(
            self.key_type.rust_ident(),
            self.val_type.rust_ident(),
            &self.kind,
        )
    }

    fn span(&self) -> Span {
        self.hash_map_type_ident.span()
    }
}
