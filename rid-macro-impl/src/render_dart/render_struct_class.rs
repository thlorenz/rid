use rid_common::{DART_FFI, FFI_GEN_BIND};

use crate::{
    attrs::TypeInfoMap,
    parse::{ParsedStruct, ParsedStructField},
};

pub struct ParsedStructRenderConfig {
    pub comment: String,
    pub dart_class_only: bool,
    pub include_equality: bool,
    pub include_to_string: bool,
}

impl ParsedStruct {
    pub fn render_struct_pointer_to_class_extension(
        &self,
        config: &ParsedStructRenderConfig,
    ) -> String {
        let class_name = self.ident.to_string();
        let raw_class_name = format!(
            "{dart_ffi}.Pointer<{ffigen_bind}.{ident}>",
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            ident = self.raw_ident
        );
        let constructor_fields = self.render_constructor_fields(config);
        let constructor_args = self.render_constructor_args(config);

        let dart_class =
            self.render_dart_class(config, &constructor_fields, &class_name);
        if config.dart_class_only {
            dart_class
        } else {
            format!(
                r###"{comment}
{comment} ```dart
{comment} // Dart class representation of {ident}.
{dart_class}
{comment}
{comment} // Extension method `toDart` to instantiate a Dart {ident} by resolving all fields from Rust
{comment} extension Rid_ToDart_ExtOn{ident} on {raw_class_name} {{
{comment}   {class_name} toDart() {{
{comment}      ridStoreLock();
{comment}      final instance = {class_name}._({constructor_args});
{comment}      ridStoreUnlock();
{comment}      return instance;
{comment}   }}
{comment} }}
{comment} ```"###,
                ident = self.ident,
                dart_class = dart_class,
                class_name = class_name,
                raw_class_name = raw_class_name,
                constructor_args = constructor_args,
                comment = config.comment
            )
        }
    }

    // -----------------
    // Args to Class constructor
    // -----------------
    fn render_constructor_args(
        &self,
        config: &ParsedStructRenderConfig,
    ) -> String {
        if self.fields.is_empty() {
            "".to_string()
        } else {
            let last_slot = self.fields.len() - 1;
            self.fields
                .iter()
                .map(|x| {
                    ParsedStructField::render_constructor_arg(
                        x,
                        self.type_infos(),
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        }
    }

    // -----------------
    // Dart Class
    // -----------------
    /// Renders a Dart class for a specific Rust struct that can be instantiated by passing a pointer
    /// to a Rust struct instance.
    /// It includes a private constructor and overrides for equality and toString.
    fn render_dart_class(
        &self,
        config: &ParsedStructRenderConfig,
        constructor_fields: &str,
        class_name: &str,
    ) -> String {
        let field_declarations = self.render_field_declarations(config);
        let constructor = self.render_private_constructor(
            &config,
            constructor_fields,
            class_name,
        );
        let equality_overrides =
            self.render_equality_overrides(config, class_name);
        let to_string_override =
            self.render_to_string_override(config, class_name);

        format!(
            r###"{comment} class {class_name} {{
{field_declarations}
{constructor}
{equality_overrides}
{to_string_override}
{comment} }}"###,
            class_name = class_name,
            field_declarations = field_declarations,
            constructor = constructor,
            equality_overrides = equality_overrides,
            to_string_override = to_string_override,
            comment = config.comment
        )
    }

    // -----------------
    // Class Fields
    // -----------------
    fn render_field_declarations(
        &self,
        config: &ParsedStructRenderConfig,
    ) -> String {
        return self
            .fields
            .iter()
            .map(|x| {
                let (ty, ffi_ty) =
                    x.rust_type.render_dart_and_ffi_type(self.type_infos());
                match ffi_ty {
                    Some(ffi_ty) => format!(
                        "{comment}    {ffi_ty}\n{comment}   final {ty} {name};",
                        ffi_ty = ffi_ty,
                        ty = ty,
                        name = x.ident,
                        comment = config.comment
                    ),
                    None => format!(
                        "{comment}   final {ty} {name};",
                        ty = ty,
                        name = x.ident,
                        comment = config.comment
                    ),
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
    }

    // -----------------
    // Class Constructor
    // -----------------
    fn render_private_constructor(
        &self,
        config: &ParsedStructRenderConfig,
        constructor_fields: &str,
        class_name: &str,
    ) -> String {
        format!(
            r###"{comment}
{comment}   const {class_name}._({constructor_fields});
"###,
            class_name = class_name,
            comment = config.comment,
            constructor_fields = constructor_fields,
        )
    }

    fn render_constructor_fields(
        &self,
        config: &ParsedStructRenderConfig,
    ) -> String {
        if self.fields.is_empty() {
            "".to_string()
        } else {
            let last_slot = self.fields.len() - 1;
            self.fields
                .iter()
                .map(|x| format!("this.{name}", name = x.ident,))
                .collect::<Vec<String>>()
                .join(", ")
        }
    }

    // -----------------
    // Class Equality overrides
    // -----------------
    fn render_equality_overrides(
        &self,
        config: &ParsedStructRenderConfig,
        class_name: &str,
    ) -> String {
        if config.include_equality {
            format!(
                "{}\n{}",
                self.render_equals_operator(config, class_name),
                self.render_hash_code(config)
            )
        } else {
            "".to_string()
        }
    }

    /// Renders hashCode override for the provided class.
    ///
    /// Example:
    ///
    /// ```dart
    /// @override
    /// int get hashCode {
    ///   return
    ///     id.hashCode ^
    ///     title.hashCode ^
    ///     completed.hashCode;
    /// }
    /// ```
    fn render_hash_code(&self, config: &ParsedStructRenderConfig) -> String {
        if self.fields.is_empty() {
            "".to_string()
        } else {
            let field_xors = self
                .fields
                .iter()
                .map(|x| {
                    format!(
                        "{comment}      {field}.hashCode",
                        field = x.ident,
                        comment = config.comment
                    )
                })
                .collect::<Vec<String>>()
                .join(" ^\n");

            format!(
                r###"{comment}  @override
{comment}  int get hashCode {{
{comment}    return
{field_xors};
{comment}  }}"###,
                field_xors = field_xors,
                comment = config.comment,
            )
        }
    }

    /// Renders equals override for the provided class.
    ///
    /// Example:
    ///
    /// ```dart
    /// @override
    /// bool operator ==(Object other) {
    ///   return identical(this, other) ||
    ///     other is Todo &&
    ///         runtimeType == other.runtimeType &&
    ///         id == other.id &&
    ///         title == other.title &&
    ///         completed == other.completed;
    /// }
    /// ```
    fn render_equals_operator(
        &self,
        config: &ParsedStructRenderConfig,
        class_name: &str,
    ) -> String {
        if self.fields.is_empty() {
            "".to_string()
        } else {
            let field_comparisons = self
                .fields
                .iter()
                .map(|x| {
                    format!(
                        "{comment}          {field} == other.{field}",
                        field = x.ident,
                        comment = config.comment
                    )
                })
                .collect::<Vec<String>>()
                .join(" &&\n");

            format!(
                r###"{comment}  @override
{comment}  bool operator ==(Object other) {{
{comment}    return identical(this, other) ||
{comment}      other is {class_name} &&
{field_comparisons};
{comment}  }}"###,
                field_comparisons = field_comparisons,
                class_name = class_name,
                comment = config.comment,
            )
        }
    }

    // -----------------
    // Class toString override
    // -----------------

    /// Renders toString() override for Dart instance
    ///
    /// Example:
    /// ```dart
    /// @override
    /// String toString() {
    ///   return 'Todo{id: $id, title: $title, completed: $completed}';
    /// }
    /// ```
    fn render_to_string_override(
        &self,
        config: &ParsedStructRenderConfig,
        class_name: &str,
    ) -> String {
        if !config.include_to_string || self.fields.is_empty() {
            "".to_string()
        } else {
            let multi_line = self.fields.len() > 6;
            let field_separator = if multi_line {
                format!("\n{comment}   ", comment = config.comment)
            } else {
                ", ".to_string()
            };
            let quote = if multi_line { "'''" } else { "'" };
            let fields = self
                .fields
                .iter()
                .map(|x| format!("{field}: ${field}", field = x.ident,))
                .collect::<Vec<String>>()
                .join(&field_separator);

            let pre_fields = if multi_line {
                format!(" {{\n{comment}   ", comment = config.comment)
            } else {
                "{".to_string()
            };
            let post_fields = if multi_line {
                format!(
                    "\n{comment}{closing_brace}",
                    comment = config.comment,
                    closing_brace = "}}"
                )
            } else {
                "}".to_string()
            };
            format!(
                r###"{comment}  @override
    {comment}  String toString() {{
    {comment}    return {quote}{class_name}{pre_fields}{fields}{post_fields}{quote};
    {comment}  }}"###,
                class_name = class_name,
                pre_fields = pre_fields,
                post_fields = post_fields,
                fields = fields,
                quote = quote,
                comment = config.comment,
            )
        }
    }
}

// -----------------
// Render impls of Struct parts
// -----------------
impl ParsedStructField {
    pub fn render_constructor_arg(&self, type_infos: &TypeInfoMap) -> String {
        format!(
            "this.{resolution}",
            resolution = self
                .rust_type
                .render_to_dart_for_arg(type_infos, &self.ident)
        )
    }
}
