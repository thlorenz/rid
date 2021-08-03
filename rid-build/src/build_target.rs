#[derive(Clone)]
pub enum BuildTarget {
    Release,
    Debug,
    DebugExample(String),
}

impl BuildTarget {
    pub(crate) fn cargo_expand_args(&self) -> Vec<String> {
        use BuildTarget::*;
        match self {
            Release | Debug => {
                vec!["expand".to_string(), "--lib".to_string()]
            }
            DebugExample(example_name) => vec![
                "expand".to_string(),
                "--example".to_string(),
                example_name.to_string(),
            ],
        }
    }
}
