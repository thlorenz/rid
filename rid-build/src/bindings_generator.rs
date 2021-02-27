use anyhow::{bail, Result};
use std::{env, fs, path, process::Command};

pub(crate) struct BindingsGenerator {
    cargo: String,
    crate_dir: String,
    crate_name: String,
}

impl BindingsGenerator {
    pub(crate) fn generate(&self) -> Result<cbindgen::Bindings> {
        let expanded_rust_path = self.expand_crate()?;
        let bindings = self.cbindgen(&expanded_rust_path)?;
        Ok(bindings)
    }

    fn expand_crate(&self) -> Result<String> {
        let output = Command::new(&self.cargo)
            .arg("expand")
            .args(&["--color", "never"])
            .current_dir(&self.crate_dir)
            .output()?;

        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        if stderr.contains("error:") {
            bail!("\n'cargo expand' encountered error(s): \n\n{}\n\n", stderr);
        }

        let expanded_rust_path = self.expand_crate_path();
        fs::write(&expanded_rust_path, output.stdout)?;

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
        let crate_dir = "./fixtures/foo-bar-baz".to_string();
        let crate_name = "foo-bar-baz".to_string();
        let binding_h = bindings_path(&crate_name);
        let generator = BindingsGenerator {
            cargo: "cargo".to_string(),
            crate_dir,
            crate_name,
        };
        let bindings = generator.generate().unwrap();
        bindings.write_to_file(&binding_h);

        eprintln!("binding.h written to '{}'", &binding_h);

        let binding_h = fs::read_to_string(&binding_h).unwrap();
        assert!(binding_h.len() > 0);
        assert!(binding_h.contains("typedef struct Foo Foo;"));
        assert!(binding_h.contains("FFI access methods generated for struct 'Baz'"));
    }
}
