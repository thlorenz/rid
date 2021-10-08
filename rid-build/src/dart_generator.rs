use rid_common::{
    CSTRING_FREE, DART_ASYNC, DART_COLLECTION, DART_FFI, FFI_GEN_BIND, RID_FFI,
    STRING_TO_NATIVE_INT8,
};

use crate::{
    build_target::BuildTarget, parsed_bindings::ParsedBindings, Project,
};
const PACKAGE_FFI: &str = "package_ffi";
static RID_WIDGETS: &str = include_str!("../dart/_rid_widgets.dart");
static RID_UTILS_FLUTTER: &str =
    include_str!("../dart/_rid_utils_flutter.dart");
static RID_UTILS_DART: &str = include_str!("../dart/_rid_utils_dart.dart");
static STORE_STUB_DART: &str = include_str!("../dart/_store_stub.dart");
static REPLY_CHANNEL_STUB_DART: &str =
    include_str!("../dart/_reply_channel_stub.dart");
static RID_CLASS_INSTANTIATION: &str = include_str!("../dart/_rid_rid.dart");

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

fn dart_ffi_reexports() -> String {
    format!("export 'dart:ffi' show Pointer;\n")
}

fn message_channel_reexports(message_channel: &str) -> String {
    format!(
        "export '{message_channel}' show RidMessageChannel, RidMessage, RidMessageType;\n",
        message_channel = message_channel
    )
}

/// Generates Dart code from the provided cbindgen artifact, taking config into account.
pub(crate) struct DartGenerator<'a> {
    /// Relative path to the bindings generated by Dart ffigen from where we will put the code
    /// generated here.
    pub(crate) ffigen_binding: &'a str,

    /// Relative path to the message_channel Dart implementation
    pub(crate) message_channel: &'a str,

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

    /// If `true` the user didn't implement a store yet and we need to stub some methods to make
    /// things work
    pub(crate) needs_store_stub: bool,

    /// If `true` the user didn't implement a #[rid::reply] enum yet and we need to stub a
    /// `dipsose`able `replyChannel` global var
    pub(crate) needs_reply_channel_stub: bool,
}

impl<'a> DartGenerator<'a> {
    pub(crate) fn generate(&self) -> String {
        let extensions = &self.code_sections.dart_code;
        let _enums = &self.code_sections.enums;
        let flutter_widget_overrides = match self.project {
            Project::Dart => "",
            Project::Flutter(_) => RID_WIDGETS,
        };
        let rid_utils = match self.project {
            Project::Dart => RID_UTILS_DART,
            Project::Flutter(_) => RID_UTILS_FLUTTER,
        };
        let store_stub = if self.needs_store_stub {
            STORE_STUB_DART
        } else {
            ""
        };

        let reply_channel_stub = if self.needs_reply_channel_stub {
            REPLY_CHANNEL_STUB_DART
        } else {
            ""
        };

        format!(
            r###"// ignore_for_file:unused_import, unused_element
{imports}
// Forwarding dart_ffi types essential to access raw Rust structs
{dart_ffi_exports}
// Forwarding Dart Types for raw Rust structs
{struct_exports}
// Forwarding MessageChannel Types
{message_channel_exports}

//
// Open Dynamic Library
//
{open_dl}
{flutter_widget_overrides}
//
// Rid internal Utils
// 
{rid_utils}
//
// Extensions to provide an API for FFI calls into Rust
//
{extensions}
{string_from_pointer_extension}
{string_pointer_from_string_extension}

{rid_class_instantiation}

{store_stub}
{reply_channel_stub}
"###,
            imports = self.dart_imports(),
            dart_ffi_exports = dart_ffi_reexports(),
            message_channel_exports =
                message_channel_reexports(self.message_channel),
            struct_exports = self.dart_rust_type_reexports(),
            open_dl = self.dart_open_dl(),
            flutter_widget_overrides = flutter_widget_overrides,
            rid_utils = rid_utils,
            extensions = extensions,
            string_from_pointer_extension = dart_string_from_pointer(),
            string_pointer_from_string_extension =
                dart_string_pointer_from_string(),
            rid_class_instantiation = RID_CLASS_INSTANTIATION,
            store_stub = store_stub,
            reply_channel_stub = reply_channel_stub
        )
        .to_string()
    }

    fn dart_imports(&self) -> String {
        let project_specific_imports = match self.project {
            Project::Dart => "",
            Project::Flutter(_) => {
                r###"
import 'package:flutter/widgets.dart';
import 'package:flutter/foundation.dart' as Foundation;"###
            }
        };
        format!(
            r###"import 'dart:ffi' as {dart_ffi};
import 'dart:async' as {dart_async};
import 'dart:io' as dart_io;
import 'dart:collection' as {dart_collection};
import 'package:ffi/ffi.dart' as {pack_ffi};
import '{ffigen_binding}' as {ffigen_bind};
import '{message_channel}';
import '{reply_channel}';
{project_specific_imports}
"###,
            dart_ffi = DART_FFI,
            dart_async = DART_ASYNC,
            dart_collection = DART_COLLECTION,
            ffigen_binding = self.ffigen_binding,
            message_channel = self.message_channel,
            reply_channel = self.reply_channel,
            pack_ffi = PACKAGE_FFI,
            ffigen_bind = FFI_GEN_BIND,
            project_specific_imports = project_specific_imports,
        )
        .to_string()
    }

    fn dart_open_dl(&self) -> String {
        let sub_target_folder = match self.target {
            BuildTarget::Release => "release",
            BuildTarget::Debug => "debug",
            BuildTarget::DebugExample(_) => "debug/examples",
        };
        match self.project {
            Project::Dart => {
                format!(
                    r###"{dart_ffi}.DynamicLibrary _open() {{
  if (dart_io.Platform.isLinux)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}/{sub}/{lib_name}.so');
  if (dart_io.Platform.isMacOS)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}/{sub}/{lib_name}.dylib');
  if (dart_io.Platform.isWindows)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}\\{sub}\\{lib_name}.dll');
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
  if (dart_io.Platform.isLinux)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}/{sub}/{lib_name}.so');
  if (dart_io.Platform.isMacOS)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}/{sub}/{lib_name}.dylib');
  if (dart_io.Platform.isWindows)
    return {dart_ffi}.DynamicLibrary.open('{path_to_target}\\{sub}\\{lib_name}.dll');
  throw UnsupportedError(
    'Platform "${{dart_io.Platform.operatingSystem}}" is not supported.');
}}
"###,
                    dart_ffi = DART_FFI,
                    lib_name = self.lib_name,
                    path_to_target = self.path_to_target,
                    sub = sub_target_folder,
                )
            }
        }
    }

    /// Reexports native struct/enum types generated by ffigen from the binding.
    /// They are static classes with int properties
    fn dart_rust_type_reexports(&self) -> String {
        let types = self.code_sections.renamed_structs();
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
