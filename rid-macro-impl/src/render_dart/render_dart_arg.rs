use crate::parse::rust_type::RustType;

#[derive(Debug)]
pub struct DartArg {
    pub arg: String,
    pub ty: String,
}

impl DartArg {
    pub fn from(ty: &RustType, slot: usize) -> Self {
        let dart_ty = ty.render_dart_type(true);
        let arg = format!("arg{}", slot);
        Self { arg, ty: dart_ty }
    }

    pub fn render_typed_parameter(&self) -> String {
        let DartArg { arg, ty, .. } = self;
        format!("{ty} {arg}", ty = ty, arg = arg)
    }

    pub fn render_parameter(&self) -> String {
        format!("{arg}", arg = self.arg)
    }
}
