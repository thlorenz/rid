use rid_common::RID_FFI;

use crate::parse::rust_type::RustType;

impl RustType {
    pub fn render_dart_struct_debug_impl(
        &self,
        fn_debug_method_name: &str,
        fn_debug_pretty_method_name: &str,
    ) -> String {
        format!(
            r###"
                /// String debug([bool pretty = false]) {{
                ///   final ptr = pretty
                ///     ? {rid_ffi}.{debug_pretty_method}(this)
                ///     : {rid_ffi}.{debug_method}(this);
                ///   final s = ptr.toDartString();
                ///   ptr.free();
                ///   return s;
                /// }}
                "###,
            rid_ffi = RID_FFI,
            debug_method = fn_debug_method_name,
            debug_pretty_method = fn_debug_pretty_method_name,
        )
    }
}
