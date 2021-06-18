use crate::parse::ParsedEnum;

impl ParsedEnum {
    pub fn render_dart(&self, comment: &str) -> String {
        let variants = self
            .variants
            .iter()
            .map(|x| x.ident.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            r###"{comment}
{comment} ```dart
{comment} /// Dart enum implementation for Rust {enum_ident} enum.
{comment} enum {enum_ident} {{ {variants} }}
{comment} ```
"###,
            comment = comment,
            enum_ident = self.ident,
            variants = variants,
        )
    }
}
