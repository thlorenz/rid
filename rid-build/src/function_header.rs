#[derive(Debug, PartialEq)]
pub enum FunctionArg {
    Pointer,
    Int,
    Struct(String),
}

impl FunctionArg {
    fn render_swift(&self) -> String {
        use FunctionArg::*;
        match self {
            Pointer => "nil",
            Int => "0",
            Struct(_) => "nil",
        }
        .to_string()
    }
}

/// Parsed C function header. Does not include return type since this is not needed for
/// the current use, which is to render dummy Swift calls.
#[derive(Debug, PartialEq)]
pub struct FunctionHeader {
    /// Name of the function
    pub name: String,
    /// Arguments to call the function
    pub args: Vec<FunctionArg>,
}

impl FunctionHeader {
    pub fn render_swift_call(&self) -> String {
        let args: Vec<String> =
            self.args.iter().map(|x| x.render_swift()).collect();
        format!("{}({})", self.name, args.join(","))
    }
}
