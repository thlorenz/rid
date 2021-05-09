use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::Project;

const PREVENT_TREESHAKE_START: &str = "// <rid:prevent_tree_shake Start>";
const PREVENT_TREESHAKE_END: &str = "// <rid:prevent_tree_shake End>";

/// Inject Swift code sections with dummy calls into Swift Class files to prevent Swift purging
/// it from the binary via tree shaking.
pub struct SwiftInjector<'a> {
    pub project: &'a Project,
}

impl<'a> SwiftInjector<'a> {
    pub fn inject(
        &self,
        project_root: &Path,
        inject_code: &str,
    ) -> Result<Vec<PathBuf>> {
        let swift_plugin_files =
            self.project.paths_to_swift_plugin_files(project_root);

        for file in &swift_plugin_files {
            let plugin_content = fs::read_to_string(&file)?;
            let content_injected = inject_into(&plugin_content, inject_code);
            fs::write(&file, content_injected)?;
        }

        Ok(swift_plugin_files)
    }
}

fn inject_into(plugin_content: &str, inject_code: &str) -> String {
    let mut lines: Vec<&str> = vec![];
    let mut inside_injection = false;
    for line in plugin_content.lines() {
        if line.trim() == PREVENT_TREESHAKE_START {
            inside_injection = true;
            continue;
        }
        if line.trim() == PREVENT_TREESHAKE_END {
            inside_injection = false;
            continue;
        }
        if !inside_injection {
            lines.push(line);
        }
    }
    let plugin_code = lines.join("\n");

    format!(
        "{plugin_code}\n{tree_shake_start}\n{inject_code}\n{tree_shake_end}",
        plugin_code = plugin_code,
        tree_shake_start = PREVENT_TREESHAKE_START,
        inject_code = inject_code,
        tree_shake_end = PREVENT_TREESHAKE_END,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUMMY_ONE: &str = "func dummyMethodOne() {
    someCall();
}";

    const DUMMY_TWO: &str = "func dummyMethodTwo() {
    someCall();
    someOtherCall();
}";

    #[test]
    fn first_time_inject() {
        let plugin_content = "import Flutter

public class SwiftPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
  }
}
";
        let res = inject_into(&plugin_content, DUMMY_ONE);
        assert_eq!(
            res,
            "import Flutter

public class SwiftPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
  }
}
// <rid:prevent_tree_shake Start>
func dummyMethodOne() {
    someCall();
}
// <rid:prevent_tree_shake End>"
        );
    }

    #[test]
    fn second_time_inject() {
        let plugin_with_injection_content = "import Flutter

public class SwiftPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
  }
}
// <rid:prevent_tree_shake Start>
func dummyMethodOne() {
    someCall();
}
// <rid:prevent_tree_shake End>";

        let res = inject_into(&plugin_with_injection_content, DUMMY_TWO);
        assert_eq!(
            res,
            "import Flutter

public class SwiftPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
  }
}
// <rid:prevent_tree_shake Start>
func dummyMethodTwo() {
    someCall();
    someOtherCall();
}
// <rid:prevent_tree_shake End>"
        );
    }
}
