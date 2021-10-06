use rid_build::{build, BuildConfig, BuildTarget, Project};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Missing CARGO_MANIFEST_DIR, please run this via 'cargo run'");

    let test = env::var("TEST").expect("Please provide the test to run");

    let workspace_dir = &crate_dir;

    let crate_name = &env::var("CARGO_PKG_NAME")
        .expect("Missing CARGO_PKG_NAME, please run this via 'cargo run'");
    let lib_name = &if cfg!(target_os = "windows") {
        format!("{}", &test)
    } else {
        format!("lib{}", &test)
    };

    let build_config = BuildConfig {
        target: BuildTarget::DebugExample(test),
        project: Project::Dart,
        lib_name,
        crate_name,
        project_root: &crate_dir,
        workspace_root: Some(&workspace_dir),
    };
    build(&build_config).expect("Build failed");
}
