#[derive(Clone)]
pub enum BuildTarget {
    Release,
    Debug,
    DebugExample(String),
}

impl BuildTarget {
    pub(crate) fn cargo_expand_args(&self) -> Vec<String> {
        use BuildTarget::*;

        let mut target = match self {
            Release | Debug => vec!["--lib".to_string()],
            DebugExample(example_name) => {
                vec!["--example".to_string(), example_name.to_string()]
            }
        };
        let mut cmd = vec!["+nightly".to_string(), "rustc".to_string()];
        cmd.append(&mut target);
        cmd.append(&mut vec![
            "--".to_string(),
            "-Zunpretty=expanded".to_string(),
        ]);
        cmd
    }
}
