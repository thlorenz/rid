use crate::parse::rust_type::{
    Composite, Primitive, RustType, TypeKind, Value,
};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Ident;

pub fn render_return_type(rust_type: &RustType) -> TokenStream {
    use crate::parse::ParsedReference::*;
    use TypeKind as K;
    let RustType {
        ident,
        kind,
        reference,
    } = rust_type;

    // NOTE: we will need a pointer version of this as well
    let ref_ident_str = match reference {
        Owned => "".to_string(),
        Ref(lifetime) => format!("&{} ", stringify_lifetime(lifetime)),
        RefMut(lifetime) => format!("&{} mut ", stringify_lifetime(lifetime)),
    };
    let type_str = match kind {
        K::Primitive(prim) => stringify_primitive_return(prim),
        K::Value(val) => stringify_value_return(val),
        K::Composite(Composite::Vec, rust_type) => match rust_type {
            Some(ty) => stringify_vec_return(ty.as_ref()),
            None => {
                todo!("blow up since a composite should include inner type")
            }
        },
        K::Composite(composite, rust_type) => todo!(
            "render_function_export::stringify_type_return::custom_composite"
        ),
        K::Unit => todo!("unit"),
        K::Unknown => todo!("unknown .. need error"),
    };
    let ref_ident: TokenStream = ref_ident_str.parse().unwrap();
    let type_ident: TokenStream = type_str.parse().unwrap();
    quote_spanned! { ident.span() => #ref_ident #type_ident }
}

// -----------------
// Stringify Return Types
// -----------------
fn stringify_primitive_return(prim: &Primitive) -> String {
    use Primitive::*;
    match prim {
        U8 => "u8",
        I8 => "i8",
        U16 => "u16",
        I16 => "i16",
        U32 => "u32",
        I32 => "i32",
        U64 => "u64",
        I64 => "i64",
        USize => "usize",
        Bool => "bool",
    }
    .to_string()
}

fn stringify_value_return(val: &Value) -> String {
    use Value::*;
    match val {
        CString | String | Str => "*const ::std::os::raw::c_char",
        Custom(_, _) => todo!("stringify_export_value_type_signature::Custom"),
    }
    .to_string()
}

fn stringify_vec_return(inner_type: &RustType) -> String {
    use TypeKind as K;
    match &inner_type.kind {
        K::Primitive(prim) => {
            format!("rid::RidVec<{}>", stringify_primitive_return(prim))
        }
        K::Value(_) => todo!("stringify_vec_return::value"),
        K::Composite(_, _) => todo!("stringify_vec_return::value"),
        K::Unit => todo!("stringify_vec_return::unit -- should abort"),
        K::Unknown => {
            todo!("stringify_vec_return::unknown -- should abort")
        }
    }
}

fn stringify_lifetime(lifetime: &Option<Ident>) -> String {
    match lifetime {
        Some(lifetime) => format!("'{}", lifetime),
        None => "".to_string(),
    }
}
