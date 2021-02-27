use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};
const PACKAGE_FFI: &str = "package_ffi";

const TYPEDEF_STRUCT: &str = "typedef struct ";
const TYPEDEF_STRUCT_LEN: usize = TYPEDEF_STRUCT.len();

#[derive(Clone, Copy)]
pub(crate) enum BuildTarget {
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
}}
"###,
        dart_ffi = DART_FFI,
        pack_ffi = PACKAGE_FFI
    )
}

fn dart_rid_native() -> String {
    format!(
        r###"final {dart_ffi}.DynamicLibrary _dl = _open();
final {rid_ffi} = {ffigen_bind}.NativeLibrary(_dl);"###,
        dart_ffi = DART_FFI,
        rid_ffi = RID_FFI,
        ffigen_bind = FFI_GEN_BIND
    )
}

/// Generates Dart code from the provided cbindgen artifact, taking config into account.
pub(crate) struct DartGenerator<'a> {
    /// Relative path to the bindings generated by Dart ffigen from where we will put the code
    /// generated here.
    pub(crate) ffigen_binding: &'a str,

    /// Path to the 'target' directory of Rust binaries where we load the dynamic library from.
    pub(crate) path_to_target: &'a str,

    /// Name of the Rust lib, i.e. libapp_todo
    pub(crate) lib_name: &'a str,

    /// Content of binding.h generated by cbindgen with functions and structs expanded via
    /// rid-derive.
    pub(crate) binding: &'a str,

    /// Rust library to load, Release or Debug.
    pub(crate) target: BuildTarget,
}

impl<'a> DartGenerator<'a> {
    pub(crate) fn generate(&self) -> String {
        let (extensions, structs) = self.parse_binding();
        format!(
            r###"{imports}
// Forwarding Dart Types for Rust structs
{exports}
//
// Open Dynamic Library
//
{open_dl}
//
// Extensions to provide an API into FFI calls to Rust
//
{extensions}
{string_from_pointer_extension}
//
// Exporting Native Library to call Rust functions directly
//
{native_export}"###,
            imports = self.dart_imports(),
            exports = self.dart_reexports(structs),
            open_dl = self.dart_open_dl(),
            extensions = extensions,
            string_from_pointer_extension = dart_string_from_pointer(),
            native_export = dart_rid_native()
        )
        .to_string()
    }

    fn dart_imports(&self) -> String {
        format!(
            r###"import 'dart:ffi' as {dart_ffi};
import 'dart:io' as dart_io;
import 'package:ffi/ffi.dart' as {pack_ffi};
import '{ffigen_binding}' as {ffigen_bind};
"###,
            dart_ffi = DART_FFI,
            ffigen_binding = self.ffigen_binding,
            pack_ffi = PACKAGE_FFI,
            ffigen_bind = FFI_GEN_BIND
        )
        .to_string()
    }

    fn dart_open_dl(&self) -> String {
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
  throw UnsupportedError('This platform is not supported.');
}}
"###,
            dart_ffi = DART_FFI,
            path_to_target = self.path_to_target,
            sub = sub_target_folder,
            lib_name = self.lib_name
        )
    }

    fn dart_reexports(&self, structs: Vec<String>) -> String {
        let structs = structs.join(", ");
        format!(
            "export '{ffigen_binding}' show {structs};\n",
            ffigen_binding = self.ffigen_binding,
            structs = structs
        )
    }

    fn parse_binding(&self) -> (String, Vec<String>) {
        let mut extension_sections: Vec<Vec<String>> = vec![];
        let mut inside_extension = false;
        let mut current_extension = vec![];

        let mut structs: Vec<String> = vec![];

        for line in self.binding.lines() {
            if !inside_extension {
                let trimmed = line.trim_start();
                if trimmed.starts_with("* ```dart") {
                    inside_extension = true;
                } else if trimmed.starts_with(TYPEDEF_STRUCT) {
                    let (struct_name, _) = trimmed[TYPEDEF_STRUCT_LEN..]
                        .split_once(" ")
                        .expect(&format!("Invalid struct definition {}", &trimmed));
                    structs.push(struct_name.to_string());
                }
                continue;
            }
            if line.trim_start().starts_with("* ```") {
                extension_sections.push(current_extension);
                current_extension = vec![];
                inside_extension = false;
                continue;
            }
            let trimmed_line = line.trim();
            let without_comment = &trimmed_line[2..];
            current_extension.push(without_comment.to_string());
        }

        let extensions_code = extension_sections
            .into_iter()
            .map(|section| {
                let last_line = section.len() - 1;
                section
                    .into_iter()
                    .enumerate()
                    .fold("".to_string(), |acc, (idx, ext)| {
                        if idx == 0 || idx == last_line {
                            let new_line = if idx == 0 { "" } else { "\n" };
                            format!(
                                "{acc}{new_line}{ext}",
                                acc = acc,
                                new_line = new_line,
                                ext = ext
                            )
                        } else {
                            format!("{acc}\n  {ext}", acc = acc, ext = ext)
                        }
                    })
            })
            .fold("".to_string(), |acc, ref section| {
                let new_line = if acc == "" { "" } else { "\n\n" };
                format!(
                    "{acc}{new_line}{section}",
                    acc = acc,
                    new_line = new_line,
                    section = section
                )
            });

        structs.sort();

        (extensions_code, structs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup<'a>(binding: &'a str, target: BuildTarget) -> DartGenerator<'a> {
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

    fn _setup2<'a>(binding: &'a str, target: BuildTarget) -> DartGenerator<'a> {
        let ffigen_binding = "../generated/binding.dart";
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
        let binding_dart = include_str!("../fixtures/prims+strings_binding.dart");
        let (extensions, _) = setup(&binding_h, BuildTarget::Debug).parse_binding();
        assert_eq!(extensions, binding_dart.trim_end())
    }

    #[test]
    fn test_dart_extensions_three_structs() {
        let binding_h = include_str!("../fixtures/three_structs_binding.h");
        let binding_dart = include_str!("../fixtures/three_structs_binding.dart");
        let (extensions, _) = setup(&binding_h, BuildTarget::Debug).parse_binding();
        assert_eq!(extensions, binding_dart.trim_end())
    }

    #[test]
    fn test_dart_generation() {
        let binding_h = include_str!("../fixtures/two_structs_binding.h");
        let binding_dart = include_str!("../fixtures/two_structs_binding.dart");
        let dart = setup(&binding_h, BuildTarget::Debug).generate();
        assert_eq!(dart, binding_dart.trim())
    }
}
