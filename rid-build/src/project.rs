use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq)]
pub enum Project {
    Dart,
    Flutter,
}

impl Project {
    fn path_to_generated_dir(&self, project_root: &Path) -> PathBuf {
        match self {
            Project::Dart | Project::Flutter => {
                project_root.join("lib").join("generated").to_path_buf()
            }
        }
    }
    fn path_to_ios_lib_dir(&self, project_root: &Path) -> PathBuf {
        debug_assert_eq!(
            *self,
            Project::Flutter,
            "ios libs only exist in flutter apps"
        );
        project_root.join("ios").join("Classes").join("binding.h")
    }

    pub(crate) fn path_to_generated_ffigen(&self, project_root: &Path) -> PathBuf {
        // TODO: assuming lib/generated/ffigen_binding.dart for now, however we'll
        // need to read the pubspec.yaml and get the ffigen config from there.
        // @see: https://github.com/dart-lang/ffigen#configurations
        self.path_to_generated_dir(project_root)
            .join("ffigen_binding.dart")
            .to_path_buf()
    }

    pub(crate) fn path_to_generated_rid(&self, project_root: &Path) -> PathBuf {
        self.path_to_generated_dir(project_root)
            .join("rid_generated.dart")
            .to_path_buf()
    }

    pub(crate) fn path_to_generated_bindings(
        &self,
        project_root: &Path,
        crate_name: &str,
    ) -> PathBuf {
        match self {
            Project::Dart => tmp_bindings_path(crate_name),
            Project::Flutter => self.path_to_ios_lib_dir(project_root),
        }
    }

    pub(crate) fn path_to_target(&self, project_root: &Path) -> PathBuf {
        self.path_to_generated_dir(project_root)
            .join("target")
            .to_path_buf()
    }
}

fn tmp_bindings_path(crate_name: &str) -> PathBuf {
    let mut root = env::temp_dir();
    root.push(format!("rid_test_{}_binding.h", crate_name));
    root
}
