use anyhow::{bail, Result};
use std::{env, fs, path, process::Command};

pub(crate) struct BindingsGenerator<'a> {
    pub(crate) cargo: &'a str,
    pub(crate) crate_dir: &'a str,
    pub(crate) crate_name: &'a str,
}

pub fn inject_rid_ffi_types(stdout: &[u8]) -> String {
    format!(
        "{stdout}\n{rid_ffi}",
        stdout = std::str::from_utf8(stdout).unwrap(),
        rid_ffi = rid_ffi::code_rid_vec()
    )
}

impl<'a> BindingsGenerator<'a> {
    pub(crate) fn generate(&self) -> Result<cbindgen::Bindings> {
        let expanded_rust_path = self.expand_crate()?;
        let bindings = self.cbindgen(&expanded_rust_path)?;
        Ok(bindings)
    }

    // Option that doesn't depend on cargo-expand to be installed
    // cargo rustc --lib -- -Zunstable-options --pretty=expanded
    fn expand_crate(&self) -> Result<String> {
        let output = Command::new(&self.cargo)
            .args(&["expand", "--lib"])
            .args(&["--color", "never"])
            .current_dir(&self.crate_dir)
            .output()?;

        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        // TODO: this needs to be less brittle, i.e. if any message contains 'error:' we'd bail
        if stderr.contains("error:") {
            bail!("\n'cargo expand' encountered error(s): \n\n{}\n\n", stderr);
        }

        let expanded_rust_path = self.expand_crate_path();
        let code = inject_rid_ffi_types(&output.stdout);
        fs::write(&expanded_rust_path, code)?;

        Ok(format!(
            "{}",
            &expanded_rust_path.as_path().to_str().unwrap()
        ))
    }

    fn cbindgen(&self, expanded_rust_path: &str) -> Result<cbindgen::Bindings> {
        let built = cbindgen::Builder::new()
            .with_src(expanded_rust_path)
            .with_language(cbindgen::Language::C)
            .with_include_version(true)
            .with_no_includes()
            .with_include("stdint.h")
            .with_parse_deps(false)
            .generate()?;
        Ok(built)
    }

    fn expand_crate_path(&self) -> path::PathBuf {
        let mut root = env::temp_dir();
        root.push(format!("rid_{}_expanded.rs", self.crate_name));
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bindings_path(crate_name: &str) -> String {
        let mut root = env::temp_dir();
        root.push(format!("rid_test_{}_binding.h", crate_name));
        root.to_str().unwrap().to_string()
    }

    #[test]
    fn binding_generator() {
        let crate_dir = "./fixtures/foo-bar-baz";
        let crate_name = "foo-bar-baz";
        let binding_h = bindings_path(&crate_name);
        let generator = BindingsGenerator {
            cargo: "cargo",
            crate_dir,
            crate_name,
        };
        let bindings = generator.generate().unwrap();
        bindings.write_to_file(&binding_h);

        eprintln!("binding.h written to '{}'", &binding_h);

        let binding_h = fs::read_to_string(&binding_h).unwrap();
        assert!(binding_h.len() > 0);
        assert!(binding_h.contains("typedef struct Foo Foo;"));
        assert!(
            binding_h.contains("FFI access methods generated for struct 'Baz'")
        );
    }
}
