use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use crate::Project;

use super::{host_props::HostProps, yaml_config::YamlConfig};
use anyhow::{bail, Context, Result};
use tempfile::tempdir;

pub use log::{debug, error, info, trace, warn};

const FFIGEN_RUNNER: &str = "dart";

pub fn run_ffigen(
    p: &Project,
    host_props: &HostProps,
    project_root: &Path,
    structs_to_prefix_raw: &[String],
) -> Result<()> {
    let yaml_config =
        YamlConfig::new(p, host_props, project_root, structs_to_prefix_raw);
    let yaml_string = yaml_config.render();
    trace!("ffigen config:\n{}", yaml_string);

    let tmp_dir = tempdir().context("Failed to create tmpdir")?;
    let ffi_config_file_path: PathBuf =
        tmp_dir.path().join("ffigen_config.yaml");
    let mut ffi_config_file: File = File::create(&ffi_config_file_path)
        .context(format!(
            "Unable to create ffigen config file at {:?}",
            &ffi_config_file_path.as_path()
        ))?;

    ffi_config_file
        .write_all(yaml_string.as_bytes())
        .context("Failed to write ffigen yaml config")?;

    let output = {
        // For some reason, Dart doesn't want to run directly on Windows. As a
        // bandaid fix, Dart is run under Powershell in Windows which works as
        // expected.

        let mut cmd = Command::new(if cfg!(target_os = "windows") {
            "powershell"
        } else {
            FFIGEN_RUNNER
        });

        if cfg!(target_os = "windows") {
            cmd.args(&["-c", FFIGEN_RUNNER]);
        }

        cmd.args(&["run", "ffigen"])
            .args(&[
                "--config",
                ffi_config_file_path.as_path().to_str().unwrap(),
            ])
            .current_dir(project_root);

        debug!("Running '{:?}' from: '{:?}'", &cmd, &project_root);
        cmd.output()?
    };

    if !output.status.success() {
        bail!(
            "\n'{FFIGEN_RUNNER} run ffigen' failed to run successfully\nstderr: \"{stderr}\"\nstdout: \"{stdout}\"",
            stderr = std::str::from_utf8(&output.stderr).unwrap(),
            stdout = std::str::from_utf8(&output.stdout).unwrap()
        );
    }

    // ffigen logs to stdout
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    trace!("{}", &stdout);

    if stdout.contains("[SEVERE]") {
        bail!(
            "\n'{FFIGEN_RUNNER} run ffigen' encountered severe error(s): \n\n{}\n\n",
            stdout
        );
    }

    tmp_dir.close().context("Unable to properly clean temp dir")
}
