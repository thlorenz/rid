use proc_macro2::TokenStream;
use quote::quote;
use rid_common::RID_FFI;

use crate::parse::rust_type::RustType;

impl RustType {
    pub fn render_dart_debug_extension(
        &self,
        debug_method_name: &str,
        debug_pretty_method_name: &str,
        comment: &str,
    ) -> String {
        let type_name = self.dart_ident(true).to_string();
        let rust_type_name = self.ident().to_string();
        let (extension_target, method_arg) = if self.is_enum() {
            (type_name.clone(), "this.index")
        } else {
            (self.render_dart_pointer_type(), "this")
        };

        format!(
            r###"
{comment} Extension to expose Debug<{rust_type_name}> via `this.debug([pretty])` on {extension_target}.
{comment}
{comment} ```dart
{comment} extension {debug_method_name}_ExtOn{type_name} on {extension_target} {{
{comment}   String debug([bool pretty = false]) {{
{comment}     final ptr = pretty
{comment}       ? {rid_ffi}.{debug_pretty_method_name}({method_arg})
{comment}       : {rid_ffi}.{debug_method_name}({method_arg});
{comment}     final s = ptr.toDartString();
{comment}     ptr.free();
{comment}     return s;
{comment}   }}
{comment} }}
{comment} ```
"###,
            comment = comment,
            rid_ffi = RID_FFI,
            debug_method_name = debug_method_name,
            debug_pretty_method_name = debug_pretty_method_name,
            type_name = type_name,
            rust_type_name = rust_type_name,
            extension_target = extension_target,
            method_arg = method_arg
        )
    }
}
