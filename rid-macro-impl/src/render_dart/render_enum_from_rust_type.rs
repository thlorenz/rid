use crate::parse::rust_type::RustType;

//TODO (thlorenz): obsolete ASAP and use ./render-enum.rs instead
impl RustType {
    pub fn render_dart_enum(
        &self,
        variants: &[String],
        comment: &str,
    ) -> String {
        assert!(self.is_enum(), "Can only render dart enum for rust enum");

        let type_name = self.rust_ident().to_string();
        let rust_type_name = self.ident().to_string();
        let variants = variants.join(", ");

        format!(
            r###"
{comment} Dart enum implementation for Rust {rust_type_name} enum.
{comment}
{comment} ```dart
{comment} enum {type_name} {{ {variants} }}
{comment} ```
"###,
            comment = comment,
            type_name = type_name,
            rust_type_name = rust_type_name,
            variants = variants,
        )
    }
}
