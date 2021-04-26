use proc_macro2::TokenStream;
use rid_common::{FFI_GEN_BIND, RID_FFI};
use syn::Ident;

use crate::parse::rust_type::RustType;

impl RustType {
    pub fn render_dart_dispose_extension(
        &self,
        fn_free_ident: Ident,
        type_name: &str,
        comment: &str,
    ) -> TokenStream {
        let pointer_type = self.render_dart_pointer_type();
        let extension = format!(
            r###"
{comment} Extension to free {pointer_type} via `this.dispose()`.
{comment}
{comment} ```dart
{comment} extension {fn_free_ident}_ExtOn{type_name} on {pointer_type} {{
{comment}   void dispose() {{
{comment}     {rid_ffi}.{fn_free_ident}(this);
{comment}   }}
{comment} }}
{comment} ```
"###,
            rid_ffi = RID_FFI,
            comment = comment,
            fn_free_ident = fn_free_ident,
            type_name = type_name,
            pointer_type = pointer_type
        )
        .trim()
        .to_string();
        extension.parse().unwrap()
    }
}
