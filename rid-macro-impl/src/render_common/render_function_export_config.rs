pub struct RenderFunctionExportConfig {
    pub include_ffi: bool,
    pub include_free: bool,
    pub include_access_item: bool,
    pub comment_dart_code: bool,
}

impl Default for RenderFunctionExportConfig {
    fn default() -> Self {
        Self {
            include_ffi: true,
            include_free: true,
            include_access_item: true,
            comment_dart_code: true,
        }
    }
}

impl RenderFunctionExportConfig {
    pub fn bare() -> Self {
        Self {
            include_ffi: false,
            include_free: false,
            include_access_item: false,
            comment_dart_code: false,
        }
    }
}
