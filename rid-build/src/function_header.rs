#[derive(Debug, PartialEq)]
pub enum FunctionArg {
    Pointer,
    Int,
    Struct(String),
    Enum(String),
    RidVec(String),
}

impl FunctionArg {
    fn render_swift(&self) -> String {
        use FunctionArg::*;
        match self {
            Pointer => "nil".to_string(),
            Int => "0".to_string(),
            Struct(_) => "nil".to_string(),
            Enum(name) => format!("{}(rawValue: 0)", name),
            RidVec(name) => format!("{}()", name),
        }
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
        format!("{}({})", self.name, args.join(", "))
    }
}
