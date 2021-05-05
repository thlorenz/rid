use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct FlutterConfig {
    pub plugin_name: String,
}

#[derive(Debug, PartialEq)]
pub enum Project {
    Dart,
    Flutter(FlutterConfig),
}

impl Project {
    fn path_to_generated_dir(&self, project_root: &Path) -> PathBuf {
        match self {
            Project::Dart => {
                project_root.join("lib").join("generated").to_path_buf()
            }
            Project::Flutter(FlutterConfig { plugin_name }) => project_root
                .join(plugin_name)
                .join("lib")
                .join("generated")
                .to_path_buf(),
        }
    }

    fn path_to_ios_lib_dir(
        &self,
        project_root: &Path,
        plugin_name: &str,
    ) -> PathBuf {
        let is_flutter_project = match self {
            Project::Dart => false,
            Project::Flutter(_) => true,
        };
        debug_assert!(
            is_flutter_project,
            "ios libs only exist in flutter apps"
        );
        project_root
            .join(plugin_name)
            .join("ios")
            .join("Classes")
            .join("binding.h")
    }

    pub(crate) fn path_to_generated_ffigen(
        &self,
        project_root: &Path,
    ) -> PathBuf {
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

    pub(crate) fn path_to_generated_c_bindings(
        &self,
        project_root: &Path,
    ) -> PathBuf {
        match self {
            Project::Dart => self
                .path_to_generated_dir(project_root)
                .join("bindings.h")
                .to_path_buf(),
            Project::Flutter(FlutterConfig { plugin_name }) => {
                self.path_to_ios_lib_dir(project_root, &plugin_name)
            }
        }
    }

    pub(crate) fn path_to_target(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join("target").to_path_buf()
    }
}
