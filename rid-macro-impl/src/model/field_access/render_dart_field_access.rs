use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    accesses::{AccessRender, RenderDartAccessConfig},
    attrs::TypeInfoMap,
    common::abort,
    parse::{dart_type::DartType, ParsedStruct, ParsedStructField},
    render_dart::RenderDartTypeOpts,
};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI};

impl ParsedStruct {
    pub fn render_dart_fields_access_extension(
        &self,
        render_config: &RenderDartAccessConfig,
    ) -> (TokenStream, String) {
        let field_accesses = self.render_dart_fields_access(render_config);
        let s = format!(
            r###"
{comment}```dart
{comment}extension Rid_Model_ExtOnPointer{struct_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{
{field_accesses}
{comment}}}
{comment}```
        "###,
            struct_ident = self.raw_ident,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            field_accesses = field_accesses,
            comment = render_config.comment
        );
        let tokens = if render_config.tokens {
            s.parse().unwrap()
        } else {
            TokenStream::new()
        };
        (tokens, s)
    }

    fn render_dart_fields_access(
        &self,
        render_config: &RenderDartAccessConfig,
    ) -> String {
        self.fields
            .iter()
            .map(move |field| {
                self.render_dart_field_access(field, render_config)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn render_dart_field_access(
        &self,
        field: &ParsedStructField,
        render_config: &RenderDartAccessConfig,
    ) -> String {
        let dart_ty = &field.dart_type;
        let dart_return_ty = field.rust_type.render_dart_field_return_type();
        let dart_ty_attr_str = match dart_ty.render_type_attribute() {
            Some(attr) => {
                format!(
                    "{comment}  {attr}\n",
                    comment = &render_config.comment,
                    attr = attr
                )
            }
            None => "".to_string(),
        };
        let getter_body = dart_ty.render_field_access_getter_body(
            &field.method_ident(&self.ident),
            &render_config.comment,
        );

        format!(
            "{dart_ty_attr}{comment}  {dart_return_ty} get {field_ident} {body}",
            dart_ty_attr = dart_ty_attr_str,
            dart_return_ty = dart_return_ty,
            field_ident = &field.ident,
            body = getter_body,
            comment = &render_config.comment
        )
    }
}

impl DartType {
    fn render_field_access_getter_body(
        &self,
        ffi_method_ident: &Ident,
        comment: &str,
    ) -> String {
        let indent = "    ";
        let half_indent = "  ";
        match self {
            // -----------------
            // Int
            // -----------------
            DartType::Int32(nullable) if *nullable => {
                todo!("DartType:render_field_access_getter_body:Int32:nullable")
            }
            DartType::Int64(nullable) if *nullable => {
                todo!("DartType:render_field_access_getter_body:Int64:nullable")
            }
            DartType::Int32(_) | DartType::Int64(_) => format!(
                "{{ return {rid_ffi}.{ffi_method}(this); }}",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident,
            ),
            // -----------------
            // Bool
            // -----------------
            DartType::Bool(nullable) if *nullable => {
                todo!("DartType:render_field_access_getter_body:Bool:nullable")
            }
            DartType::Bool(_) => format!(
                "{{ return {rid_ffi}.{ffi_method}(this) != 0; }}",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident,
            ),
            // -----------------
            // Double
            // -----------------
            DartType::Double(true) => todo!("DartType:render_field_access_getter_body:Double:nullable"),
            DartType::Double(false) => format!(
                "{{ return {rid_ffi}.{ffi_method}(this); }}",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident,
            ),
            // -----------------
            // String
            // -----------------
            DartType::String(nullable) if *nullable => format!(
                r###"{{
{comment}{indent}{dart_ffi}.Pointer<{dart_ffi}.Int8> ptr = {rid_ffi}.{ffi_method}(this);
{comment}{indent}if (ptr.address == 0x0) return null;
{string_resolution}
{comment}{half_indent}}}"###,
                string_resolution =
                    string_resolution(indent, comment, ffi_method_ident),
                dart_ffi = DART_FFI,
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident,
                indent = indent,
                half_indent = half_indent,
                comment = comment
            ),
            DartType::String(_) => format!(
                r###"{{
{comment}{indent}{dart_ffi}.Pointer<{dart_ffi}.Int8>? ptr = {rid_ffi}.{ffi_method}(this);
{string_resolution}
{comment}{half_indent}}}"###,
                string_resolution =
                    string_resolution(indent, comment, ffi_method_ident),
                dart_ffi = DART_FFI,
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident,
                indent = indent,
                half_indent = half_indent,
                comment = comment
            ),
            // -----------------
            // Custom
            // -----------------
            DartType::Custom(_, _, _) => format!(
                "{{ return {rid_ffi}.{ffi_method}(this); }}",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident
            ),
            // -----------------
            // Collection Types
            // -----------------
            DartType::Vec(_, _) => format!(
                "{{ return {rid_ffi}.{ffi_method}(this); }}",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident
            ),
            DartType::HashMap(_, _, _) => format!(
                "{{ return {rid_ffi}.{ffi_method}(this); }}",
                rid_ffi = RID_FFI,
                ffi_method = ffi_method_ident
            ),
            // -----------------
            // Invalid
            // -----------------
            DartType::Unit => {
                abort!(ffi_method_ident, "Dart get to void field not supported")
            }
        }
    }
}

fn string_resolution(
    indent: &str,
    comment: &str,
    ffi_method_ident: &Ident,
) -> String {
    format!(
        r###"{comment}{indent}int len = {rid_ffi}.{ffi_method}_len(this);
{comment}{indent}String s = ptr.toDartString(len);
{comment}{indent}ptr.free();
{comment}{indent}return s;"###,
        rid_ffi = RID_FFI,
        ffi_method = ffi_method_ident,
        indent = indent,
        comment = comment
    )
}
