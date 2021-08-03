use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Item;

use super::{render_enum::render_enum, render_struct::render_struct};
use crate::common::{abort, utils_module_tokens};

pub fn rid_ffi_model_impl(item: &Item, is_store: bool) -> TokenStream {
    match item {
        Item::Struct(struct_item) => {
            let tokens = render_struct(struct_item, is_store);
            let utils_module = utils_module_tokens();
            quote_spanned! { struct_item.ident.span() =>
                #item
                #tokens
                #utils_module
            }
        }
        Item::Enum(enum_item) => {
            let tokens = render_enum(enum_item);
            let utils_module = utils_module_tokens();
            quote_spanned! { enum_item.ident.span() =>
                #[repr(C)]
                #item
                #tokens
                #utils_module
            }
        }
        _ => {
            abort!(item, "rid::model attribute can only be applied to structs and c-style enums")
        }
    }
}
