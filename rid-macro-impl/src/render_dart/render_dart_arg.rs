use crate::{
    attrs::TypeInfoMap, parse::rust_type::RustType,
    render_dart::RenderDartTypeOpts,
};

#[derive(Debug)]
pub struct DartArg {
    pub raw_arg: String,
    pub arg: String,
    pub ty: String,
}

impl DartArg {
    pub fn from(ty: &RustType, type_infos: &TypeInfoMap, slot: usize) -> Self {
        let dart_ty =
            ty.render_dart_type(type_infos, RenderDartTypeOpts::attr_raw());
        let raw_arg = if ty.is_string_like() {
            format!("arg{}.toNativeInt8()", slot)
        } else {
            format!("arg{}", slot)
        };
        let arg = format!("arg{}", slot);
        Self {
            arg,
            raw_arg,
            ty: dart_ty,
        }
    }

    pub fn render_typed_parameter(&self) -> String {
        let DartArg { arg, ty, .. } = self;
        format!("{ty} {arg}", ty = ty, arg = arg)
    }

    pub fn render_parameter(&self) -> String {
        format!("{arg}", arg = self.arg)
    }

    pub fn render_raw_parameter(&self) -> String {
        format!("{raw_arg}", raw_arg = self.raw_arg)
    }
}
