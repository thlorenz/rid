use rid_common::{DART_FFI, FFI_GEN_BIND, RID_CREATE_STORE};
use syn::Ident;

use crate::{
    attrs::Derive, common::prefixes::store_field_ident, parse::ParsedStruct,
};

impl ParsedStruct {
    pub fn render_store_api(&self, derive: &Derive, comment: &str) -> String {
        let store_ident: &Ident = &self.ident;
        let raw_store_ident: &Ident = &self.raw_ident;

        let store_field = store_field_ident(store_ident);

        let debug_api = if derive.debug {
            format!("{comment}   String debug([bool pretty = false]) => _store.debug(pretty);", comment = comment)
        } else {
            "".to_string()
        };

        format!(
            r###"
{comment} // rid API that provides memory safety and which is recommended to use.
{comment} // Use the lower level API (via `{Store}.raw`) only when you need more control,
{comment} // i.e. if you run into performance issues with this higher level API.
{comment} class {Store} {{
{comment}   final {Pointer}<{RawStore}> {_store};
{comment}
{comment}   /// Provides direct access to the underlying Rust store.
{comment}   /// You should not need to work with this lower level API except for cases
{comment}   /// where you want more fine grained control over how data is retrieved from
{comment}   /// Rust and converted into Dart, i.e. to tweak performance.
{comment}   /// In all other cases you should use the higher level API which is much
{comment}   /// easier to use and also guarantees memory safety.
{comment}   {Pointer}<{RawStore}> get raw => {_store};
{comment}
{comment}   const {Store}(this.{_store});
{comment}
{comment}   T _read<T>(T Function({Pointer}<{RawStore}> store) accessor, String? request) {{
{comment}     return {_store}.runLocked(accessor, request: request);
{comment}   }}
{comment}
{comment}   {Store}State toDartState() => _store.toDart();
{debug_api}
{comment}
{comment}   /// Disposes the store and closes the Rust reply channel in order to allow the app
{comment}   /// to exit properly. This needs to be called when exiting a Dart application.
{comment}   Future<void> dispose() => {_store}.dispose();
{comment}
{comment}   static {Store}? _instance;
{comment}   static {Store} get instance {{
{comment}     if (_instance == null) {{
{comment}       _instance = {Store}({createStore}());
{comment}     }}
{comment}     return _instance!;
{comment}   }}
{comment} }}
"###,
            Store = store_ident,
            RawStore = format!(
                "{ffigen_bind}.{raw_store_ident}",
                ffigen_bind = FFI_GEN_BIND,
                raw_store_ident = raw_store_ident
            ),
            Pointer = format!("{dart_ffi}.Pointer", dart_ffi = DART_FFI),
            _store = store_field,
            createStore = RID_CREATE_STORE,
            debug_api = debug_api,
            comment = comment
        )
    }
}
