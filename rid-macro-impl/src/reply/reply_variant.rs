use syn::{Field, Variant};

use crate::{
    common::abort,
    parse::rust_type::{self, Primitive, RustType, Value},
};

pub struct ReplyVariant {
    pub ident: syn::Ident,
    pub slot: usize,
    pub has_req_id: bool,
    pub has_data: bool,
}

// There are only three versions of a reply variant which simplifies parsing a lot.
// No fields:                         Foo
// With RequestId field:              Foo(u64)
// With RequestId and data fields:    Foo(u64, String)
impl ReplyVariant {
    pub fn new(slot: usize, variant: &Variant) -> Self {
        let field_vec: Vec<&Field> = variant.fields.iter().collect();
        let ident = variant.ident.clone();
        if field_vec.is_empty() {
            ReplyVariant {
                ident,
                slot,
                has_req_id: false,
                has_data: false,
            }
        } else if field_vec.len() == 1 {
            verify_req_id_type(field_vec[0], &ident);
            ReplyVariant {
                ident,
                slot,
                has_req_id: true,
                has_data: false,
            }
        } else if field_vec.len() == 2 {
            verify_req_id_type(field_vec[0], &ident);
            verify_data_type(field_vec[1], &ident);
            ReplyVariant {
                ident,
                slot,
                has_req_id: true,
                has_data: true,
            }
        } else {
            abort!(ident.span(), "Can only have reqId and data inside a reply, i.e. 'Started(u64, String)'")
        }
    }
}

fn verify_req_id_type(req_id: &Field, variant_ident: &syn::Ident) {
    let rust_type = RustType::from_plain_type(&req_id.ty);
    match rust_type {
        Some(RustType {
            kind: rust_type::TypeKind::Primitive(p),
            ..
        }) if p == Primitive::U64 => {}
        Some(RustType { ident, .. }) => abort!(
            ident,
            "First reply field needs to be a u64, i.e. 'Started(u64)'"
        ),
        None => abort!(
            variant_ident,
            "First reply field needs to be a u64, i.e. 'Started(u64)'"
        ),
    };
}

fn verify_data_type(data: &Field, variant_ident: &syn::Ident) {
    let rust_type = RustType::from_plain_type(&data.ty);
    match rust_type {
        Some(RustType {
            kind: rust_type::TypeKind::Value(v),
            ..
        }) if v == Value::String => {}
        Some (RustType { ident, .. }) => abort!(
            ident,
            "Second reply field needs to be a String, i.e. 'Started(u64, String)'"
        ),
        None => abort!(
            variant_ident,
            "Second reply field needs to be a String, i.e. 'Started(u64, String)'"
        ),
    };
}
