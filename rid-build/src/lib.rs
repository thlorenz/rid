#![allow(dead_code)]
#![feature(str_split_once)]

use std::path::{Path, PathBuf};
mod bindings_generator;
mod dart_generator;

pub enum ProjectType {
    Dart,
    Flutter,
}

impl ProjectType {
    fn path_to_generated_dir(&self, project_root: &Path) -> PathBuf {
        match self {
            ProjectType::Dart | ProjectType::Flutter => {
                project_root.join("lib").join("generated").to_path_buf()
            }
        }
    }

    fn path_to_generated_ffigen(&self, project_root: &Path) -> PathBuf {
        // TODO: assuming lib/generated/ffigen_binding.dart for now, however we'll
        // need to read the pubspec.yaml and get the ffigen config from there.
        // @see: https://github.com/dart-lang/ffigen#configurations
        self.path_to_generated_dir(project_root)
            .join("ffigen_binding.dart")
            .to_path_buf()
    }

    fn path_to_generated_rid(&self, project_root: &Path) -> PathBuf {
        self.path_to_generated_dir(project_root)
            .join("rid_generated.dart")
            .to_path_buf()
    }
}

pub struct BuildConfig {
    pub project_root: String,
    pub project_type: ProjectType,
    pub lib_name: String,
}

pub fn build(config: &BuildConfig) {}
