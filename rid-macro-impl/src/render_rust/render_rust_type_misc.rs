use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::parse::rust_type::RustType;

impl RustType {
    /// Type Alias that needs to be inserted in case the ident used in wrapper methods differs from
    /// the actual Rust ident. At this point this is only the case for structs.
    /// If no type alias is needed an empty token stream is returned.
    pub fn typealias_tokens(&self) -> TokenStream {
        if self.needs_type_alias {
            let raw_ident = &self.ident();
            let rust_ident = &self.rust_ident();
            quote_spanned! { rust_ident.span() => type #raw_ident = #rust_ident; }
        } else {
            TokenStream::new()
        }
    }
}
