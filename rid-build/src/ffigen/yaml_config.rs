use std::path::Path;

use crate::{constants::FFI_NATIVE_LIBRARY_NAME, Project};

use super::host_props::HostProps;
const IGNORE_FOR_FILE: [&str; 5] = [
    "non_constant_identifier_names",
    "unused_import",
    "unused_field",
    "unused_element",
    "camel_case_types",
];

// See https://github.com/dart-lang/ffigen#configurations
#[derive(Debug)]
pub struct YamlConfig {
    llvm_paths: Vec<String>,
    output: String,
    structs_to_prefix_raw: Vec<String>,
    header_entry_point: String,
}

impl YamlConfig {
    pub fn new(
        p: &Project,
        host_props: &HostProps,
        project_root: &Path,
        structs_to_prefix_raw: &[String],
    ) -> Self {
        let c_bindings = p.paths_to_generated_c_bindings(project_root);
        debug_assert!(
            !c_bindings.is_empty(),
            "Need at least one generated c-binding header file"
        );
        Self {
            llvm_paths: host_props.llvm_paths.clone(),
            output: p
                .path_to_generated_ffigen(project_root)
                .to_string_lossy()
                .to_string(),
            structs_to_prefix_raw: structs_to_prefix_raw.into(),
            header_entry_point: c_bindings[0].to_string_lossy().to_string(),
        }
    }

    pub fn render(&self) -> String {
        let mut s = String::from("# Generated ffigen config by rid-build\n");

        s.push_str(format!("name: '{}'\n", FFI_NATIVE_LIBRARY_NAME).as_str());
        s.push_str("description: 'Generated ffigen config by rid-build'\n");

        s.push_str("comments: false\n");

        // preamble: |
        //   // ignore_for_file: non_constant_identifier_names, unused_import, unused_field, unused_element, camel_case_types
        s.push_str("preamble: |\n");
        s.push_str("  // ignore_for_file: ");
        s.push_str(IGNORE_FOR_FILE.join(", ").as_str());
        s.push_str("\n");

        s.push_str("llvm-path:\n");
        for p in &self.llvm_paths {
            s.push_str(format!("  - {}\n", p).as_str());
        }

        // output: 'lib/generated/ffigen_binding.dart'
        s.push_str(format!("output: '{}'\n", self.output).as_str());
        // headers:
        //   entry-points:
        //     - 'lib/generated/bindings.h'
        s.push_str("headers:\n");
        s.push_str("  entry-points:\n");
        s.push_str(format!("    - '{}'\n", self.header_entry_point).as_str());

        // structs:
        //   rename:
        //     '(Todo|Store)': 'Raw$1'
        s.push_str("structs:\n");
        s.push_str("  rename:\n");
        s.push_str(
            format!(
                "    '({})': 'Raw$1'\n",
                self.structs_to_prefix_raw.join("|")
            )
            .as_str(),
        );

        s
    }
}
