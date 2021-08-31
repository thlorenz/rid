pub struct ExportConfig {
    pub render_dart_extension: bool,
    pub render_vec_access: bool,
    pub include_ffi: bool,
    pub render_utils_module: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            render_dart_extension: true,
            render_vec_access: true,
            include_ffi: true,
            render_utils_module: true,
        }
    }
}

impl ExportConfig {
    pub fn for_tests() -> Self {
        Self {
            render_dart_extension: false,
            render_vec_access: false,
            include_ffi: false,
            render_utils_module: false,
        }
    }
}
