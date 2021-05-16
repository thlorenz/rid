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

// There are only four versions of a reply variant which simplifies parsing a lot.
// No fields:                         Foo
// With RequestId field:              Foo(u64)
// With RequestId and data fields:    Foo(u64, String)
// With data field:                   Foo(String)
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
            if is_req_id_type(field_vec[0]) {
                ReplyVariant {
                    ident,
                    slot,
                    has_req_id: true,
                    has_data: false,
                }
            } else if is_data_type(field_vec[0]) {
                ReplyVariant {
                    ident,
                    slot,
                    has_req_id: false,
                    has_data: true,
                }
            } else {
                abort!(
                    ident,
                    "For replies with a single field it needs to be a u64 or String, i.e. 'Started(u64) or Started(String)'"
                )
            }
        } else if field_vec.len() == 2 {
            if !is_req_id_type(field_vec[0]) {
                abort!(
                    ident,
                    "For replies with two fields the first field needs to be a u64, i.e. 'Started(u64, String)'"
                )
            }
            if !is_data_type(field_vec[1]) {
                abort!(
                    ident,
                    "For reply with two fields the second field needs to be a String, i.e. 'Started(u64, String)'"
                )
            }

            ReplyVariant {
                ident,
                slot,
                has_req_id: true,
                has_data: true,
            }
        } else {
            abort!(ident.span(), "Only specific forms of reply are valid, i.e. 'Started | Started(u64) | Started(String) | Started(u64, String)'")
        }
    }
}

fn is_req_id_type(req_id: &Field) -> bool {
    let rust_type = RustType::from_plain_type(&req_id.ty);
    match rust_type {
        Some(RustType {
            kind: rust_type::TypeKind::Primitive(p),
            ..
        }) if p == Primitive::U64 => true,
        _ => false,
    }
}

fn is_data_type(data: &Field) -> bool {
    let rust_type = RustType::from_plain_type(&data.ty);
    match rust_type {
        Some(RustType {
            kind: rust_type::TypeKind::Value(v),
            ..
        }) if v == Value::String => true,
        _ => false,
    }
}
