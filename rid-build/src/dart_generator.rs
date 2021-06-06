use rid_common::{
    CSTRING_FREE, DART_COLLECTION, DART_FFI, FFI_GEN_BIND, RID_FFI,
    STRING_TO_NATIVE_INT8,
};

use crate::{parsed_bindings::ParsedBindings, FlutterConfig, Project};
const PACKAGE_FFI: &str = "package_ffi";
static RID_WIDGETS: &str = include_str!("../dart/_rid_widgets.dart");

#[derive(Clone, Copy)]
pub enum BuildTarget {
    Release,
    Debug,
}

fn dart_string_from_pointer() -> String {
    format!(
        r###"
extension Rid_ExtOnPointerInt8 on {dart_ffi}.Pointer<{dart_ffi}.Int8> {{
  String toDartString([int? len]) {{
    final {dart_ffi}.Pointer<{pack_ffi}.Utf8> stringPtr = this.cast();
    return stringPtr.toDartString(length: len);
  }}
  void free() {{
    {rid_ffi}.{string_free}(this);
  }}
}}
"###,
        dart_ffi = DART_FFI,
        pack_ffi = PACKAGE_FFI,
        rid_ffi = RID_FFI,
        string_free = CSTRING_FREE
    )
}

fn dart_string_pointer_from_string() -> String {
    format!(
        r###"
extension Rid_ExtOnString on String {{
  {dart_ffi}.Pointer<{dart_ffi}.Int8> {toNativeInt8}() {{
    final {dart_ffi}.Pointer<{dart_ffi}.Int8> stringPtr =
        this.toNativeUtf8().cast();
    return stringPtr;
  }}
}}
        "###,
        dart_ffi = DART_FFI,
        toNativeInt8 = STRING_TO_NATIVE_INT8,
    )
}

fn load_dynamic_library(constructor: &str) -> String {
    format!(
        r###"final {dart_ffi}.DynamicLibrary _dl = _open();
final {rid_ffi} = {ffigen_bind}.{constructor}(_dl);"###,
        dart_ffi = DART_FFI,
        rid_ffi = RID_FFI,
        ffigen_bind = FFI_GEN_BIND,
        constructor = constructor,
    )
}

fn dart_ffi_reexports() -> String {
    format!("export 'dart:ffi' show Pointer;\n")
}

/// Generates Dart code from the provided cbindgen artifact, taking config into account.
pub(crate) struct DartGenerator<'a> {
    /// Relative path to the bindings generated by Dart ffigen from where we will put the code
    /// generated here.
    pub(crate) ffigen_binding: &'a str,

    /// Relative path to the reply_channel Dart implementation
    pub(crate) reply_channel: &'a str,

    /// Path to the 'target' directory of Rust binaries where we load the dynamic library from.
    pub(crate) path_to_target: &'a str,

    /// Name of the Rust lib, i.e. libapp_todo
    pub(crate) lib_name: &'a str,

    /// Code sections to render inside the generated Dart and Swift modules respectively.
    pub(crate) code_sections: &'a ParsedBindings,

    /// Rust library to load, Release or Debug.
    pub(crate) target: &'a BuildTarget,

    pub(crate) project: &'a Project,
}

impl<'a> DartGenerator<'a> {
    pub(crate) fn generate(&self) -> String {
        let extensions = &self.code_sections.dart_code;
        let structs = &self.code_sections.structs;
        let enums = &self.code_sections.enums;
        let dynamic_library_constructor = match self.project {
            Project::Dart => "NativeLibrary",
            Project::Flutter(FlutterConfig { plugin_name, .. }) => plugin_name,
        };
        let flutter_widget_overrides = match self.project {
            Project::Dart => "",
            Project::Flutter(_) => RID_WIDGETS,
        };

        format!(
            r###"{imports}
// Forwarding dart_ffi types essential to access Rust structs
{dart_ffi_exports}
// Forwarding Dart Types for Rust structs
{struct_exports}
// Forwarding Dart Types for Rust enums
{enum_exports}
//
// Open Dynamic Library
//
{open_dl}
{flutter_widget_overrides}
//
// Extensions to provide an API for FFI calls into Rust
//
{extensions}
{string_from_pointer_extension}
{string_pointer_from_string_extension}
//
// Exporting Native Library to call Rust functions directly
//
{native_export}"###,
            imports = self.dart_imports(),
            dart_ffi_exports = dart_ffi_reexports(),
            struct_exports = self.dart_rust_type_reexports(&structs),
            enum_exports = self.dart_rust_type_reexports(&enums),
            open_dl = self.dart_open_dl(),
            flutter_widget_overrides = flutter_widget_overrides,
            extensions = extensions,
            string_from_pointer_extension = dart_string_from_pointer(),
            string_pointer_from_string_extension =
                dart_string_pointer_from_string(),
            native_export = load_dynamic_library(dynamic_library_constructor),
        )
        .to_string()
    }

    fn dart_imports(&self) -> String {
        let import_flutter_widgets = match self.project {
            Project::Dart => "",
            Project::Flutter(_) => "import 'package:flutter/widgets.dart';",
        };
        format!(
            r###"import 'dart:ffi' as {dart_ffi};
import 'dart:io' as dart_io;
import 'dart:collection' as {dart_collection};
import 'package:ffi/ffi.dart' as {pack_ffi};
import '{ffigen_binding}' as {ffigen_bind};
import '{reply_channel}';
{import_flutter_widgets}
"###,
            dart_ffi = DART_FFI,
            dart_collection = DART_COLLECTION,
            ffigen_binding = self.ffigen_binding,
            reply_channel = self.reply_channel,
            pack_ffi = PACKAGE_FFI,
            ffigen_bind = FFI_GEN_BIND,
            import_flutter_widgets = import_flutter_widgets,
        )
        .to_string()
    }

    fn dart_open_dl(&self) -> String {
        match self.project {
            Project::Dart => {
                let sub_target_folder = match self.target {
                    BuildTarget::Release => "release",
                    BuildTarget::Debug => "debug",
                };
                format!(
                    r###"{dart_ffi}.DynamicLibrary _open() {{
  if (dart_io.Platform.isLinux)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}/{sub}/{lib_name}.so');
  if (dart_io.Platform.isMacOS)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}/{sub}/{lib_name}.dylib');
  throw UnsupportedError(
      'Platform "${{dart_io.Platform.operatingSystem}}" is not supported.');
}}
"###,
                    dart_ffi = DART_FFI,
                    path_to_target = self.path_to_target,
                    sub = sub_target_folder,
                    lib_name = self.lib_name
                )
            }
            Project::Flutter(_) => {
                format!(
                    r###"{dart_ffi}.DynamicLibrary _open() {{
  if (dart_io.Platform.isLinux || dart_io.Platform.isAndroid)
    return {dart_ffi}.DynamicLibrary.open('{lib_name}.so');
  if (dart_io.Platform.isMacOS || dart_io.Platform.isIOS)
    return {dart_ffi}.DynamicLibrary.executable();
  throw UnsupportedError(
      'Platform "${{dart_io.Platform.operatingSystem}}" is not supported.');
}}
"###,
                    dart_ffi = DART_FFI,
                    lib_name = self.lib_name
                )
            }
        }
    }

    fn dart_rust_type_reexports(&self, types: &Vec<String>) -> String {
        if types.len() == 0 {
            "".to_string()
        } else {
            let types = types.join(", ");
            format!(
                "export '{ffigen_binding}' show {types};\n",
                ffigen_binding = self.ffigen_binding,
                types = types
            )
        }
    }
}

// TODO: disabled due to getting stuck
#[cfg(test_disabled)]
mod tests {
    use super::*;

    fn setup<'a>(
        binding: &'a str,
        target: &'a BuildTarget,
    ) -> DartGenerator<'a> {
        let ffigen_binding = "./ffigen_binding.dart";
        let path_to_target = "target";
        let lib_name = "libapp_todo";

        DartGenerator {
            lib_name,
            target,
            path_to_target,
            ffigen_binding,
            binding,
        }
    }

    #[test]
    fn test_dart_extensions_single_struct_prims_and_strings() {
        let binding_h = include_str!("../fixtures/prims+strings_binding.h");
        let binding_dart =
            include_str!("../fixtures/prims+strings_binding.dart");
        let (extensions, _) =
            setup(&binding_h, &BuildTarget::Debug).parse_binding();
        assert_eq!(extensions, binding_dart.trim_end())
    }

    #[test]
    fn test_dart_extensions_three_structs() {
        let binding_h = include_str!("../fixtures/three_structs_binding.h");
        let binding_dart =
            include_str!("../fixtures/three_structs_binding.dart");
        let (extensions, _) =
            setup(&binding_h, &BuildTarget::Debug).parse_binding();
        assert_eq!(extensions, binding_dart.trim_end())
    }
}
