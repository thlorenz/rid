pub struct RenderFunctionExportConfig {
    pub include_ffi: bool,
    pub comment_dart_code: bool,
    pub comment_swift_code: bool,
}

impl Default for RenderFunctionExportConfig {
    fn default() -> Self {
        Self {
            include_ffi: true,
            comment_dart_code: true,
            comment_swift_code: true,
        }
    }
}

impl RenderFunctionExportConfig {
    pub fn bare() -> Self {
        Self {
            include_ffi: false,
            comment_dart_code: false,
            comment_swift_code: false,
        }
    }
}
