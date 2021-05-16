use crate::{
    attrs::TypeInfoMap, parse::rust_type::RustType,
    render_dart::RenderDartTypeOpts,
};

#[derive(Debug)]
pub struct DartArg {
    pub arg: String,
    pub ty: String,
}

impl DartArg {
    pub fn from(ty: &RustType, type_infos: &TypeInfoMap, slot: usize) -> Self {
        let dart_ty =
            ty.render_dart_type(type_infos, RenderDartTypeOpts::attr_raw());
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
