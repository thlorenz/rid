use std::path::{Path, PathBuf};

/// All paths are relative to the plugin folder, i.e. `ios/Classes`
#[derive(Debug)]
pub struct FlutterPlatform {
    binding_file: Option<String>,
    swift_plugin_file: Option<String>,
}

impl FlutterPlatform {
    pub fn ios() -> Self {
        Self {
            binding_file: Some("ios/Classes/bindings.h".to_string()),
            swift_plugin_file: Some(
                "ios/Classes/SwiftPlugin.swift".to_string(),
            ),
        }
    }
    pub fn macos() -> Self {
        Self {
            binding_file: Some("macos/Classes/bindings.h".to_string()),
            swift_plugin_file: Some("macos/Classes/Plugin.swift".to_string()),
        }
    }
    pub fn android() -> Self {
        Self {
            binding_file: None,
            swift_plugin_file: None,
        }
    }
}

#[derive(Debug)]
pub struct FlutterConfig {
    pub plugin_name: String,
    pub platforms: Vec<FlutterPlatform>,
}

#[derive(Debug)]
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
            Project::Flutter(FlutterConfig { plugin_name, .. }) => project_root
                .join(plugin_name)
                .join("lib")
                .join("generated")
                .to_path_buf(),
        }
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

    pub(crate) fn path_to_rid_generated_dart(
        &self,
        project_root: &Path,
    ) -> PathBuf {
        self.path_to_generated_dir(project_root)
            .join("rid_generated.dart")
            .to_path_buf()
    }

    pub(crate) fn path_to_isolate_binding_dart(
        &self,
        project_root: &Path,
    ) -> PathBuf {
        self.path_to_generated_dir(project_root)
            .join("isolate_binding.dart")
            .to_path_buf()
    }

    pub(crate) fn path_to_response_channel_dart(
        &self,
        project_root: &Path,
    ) -> PathBuf {
        self.path_to_generated_dir(project_root)
            .join("response_channel.dart")
            .to_path_buf()
    }

    pub(crate) fn paths_to_generated_c_bindings(
        &self,
        project_root: &Path,
    ) -> Vec<PathBuf> {
        match self {
            Project::Dart => vec![self
                .path_to_generated_dir(project_root)
                .join("bindings.h")
                .to_path_buf()],
            Project::Flutter(FlutterConfig {
                plugin_name,
                platforms,
                ..
            }) => {
                let mut vec = platforms
                    .iter()
                    .flat_map(|x| &x.binding_file)
                    .map(|x| project_root.join(plugin_name).join(x))
                    .collect::<Vec<PathBuf>>();
                vec.push(
                    self.path_to_generated_dir(project_root).join("bindings.h"),
                );
                vec
            }
        }
    }

    pub(crate) fn paths_to_swift_plugin_files(
        &self,
        project_root: &Path,
    ) -> Vec<PathBuf> {
        match self {
            Project::Dart => vec![],
            Project::Flutter(FlutterConfig {
                plugin_name,
                platforms,
                ..
            }) => platforms
                .iter()
                .flat_map(|x| &x.swift_plugin_file)
                .map(|x| project_root.join(plugin_name).join(x))
                .collect::<Vec<PathBuf>>(),
        }
    }

    pub(crate) fn path_to_target(&self, workspace_root: &Path) -> PathBuf {
        workspace_root.join("target").to_path_buf()
    }
}
