use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

use crate::parse::ParsedStruct;

pub struct ParsedStructRenderConfig {
    pub comment: String,
    pub dart_class_only: bool,
    pub include_equality: bool,
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

        // TODO(thlorenz): `rid_store_{un}lock` is only present if we have a message unless we
        // create a stub otherwise
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
{comment}      {rid_ffi}.rid_store_lock();
{comment}      final instance = {class_name}._({constructor_fields});
{comment}      {rid_ffi}.rid_store_unlock();
{comment}      return instance;
{comment}   }}
{comment} }}
{comment} ```"###,
                ident = self.ident,
                dart_class = dart_class,
                class_name = class_name,
                raw_class_name = raw_class_name,
                constructor_fields = constructor_fields,
                rid_ffi = RID_FFI,
                comment = config.comment
            )
        }
    }

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
        format!(
            r###"{comment} class {class_name} {{
{field_declarations}
{constructor}
{equality_overrides}
{comment} }}"###,
            class_name = class_name,
            field_declarations = field_declarations,
            constructor = constructor,
            equality_overrides = equality_overrides,
            comment = config.comment
        )
    }

    fn render_field_declarations(
        &self,
        config: &ParsedStructRenderConfig,
    ) -> String {
        return self
            .fields
            .iter()
            .map(|x| {
                let (ty, ffi_ty) = x.rust_type.render_dart_and_ffi_type();
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
}
