use std::env;

#[test]
pub fn export_impl() {
    macrotest::expand("tests/expand/export_impl.rs");
}
