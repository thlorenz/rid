use heck::MixedCase;
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_CREATE_STORE};
use syn::Ident;

use crate::{
    common::prefixes::{reply_class_name_for_enum, store_field_ident},
    render_dart::RenderDartTypeOpts,
};

use super::{parsed_variant::ParsedMessageVariant, ParsedMessageEnum};

impl ParsedMessageEnum {
    pub fn render_store_api(&self, comment: &str) -> String {
        let store_ident: &Ident = &self.struct_ident;
        let raw_store_ident: &Ident = &self.raw_struct_ident;

        let store_field = store_field_ident(store_ident);
        let msg_methods = self
            .parsed_variants
            .iter()
            .map(|variant| {
                self.render_dart_wrapper_method(variant, &store_field, comment)
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r###"
{comment} extension MsgApiFor_{Store} on {Store} {{
{msg_methods}
{comment} }}
"###,
            Store = store_ident,
            msg_methods = msg_methods,
            comment = comment
        )
    }

    fn render_dart_wrapper_method(
        &self,
        variant: &ParsedMessageVariant,
        store_field: &Ident,
        comment: &str,
    ) -> String {
        let fn_ident = &variant.method_ident;
        let method_name = self.dart_method_name(&fn_ident.to_string());
        let api_method_name = method_name.to_mixed_case();

        struct DartArg {
            arg: String,
            ty: String,
        }
        let args_info: Vec<(usize, DartArg)> = variant
            .fields
            .iter()
            .map(|f| DartArg {
                arg: format!("arg{}", f.slot),
                ty: f.rust_ty.render_dart_type(
                    &self.type_infos(),
                    RenderDartTypeOpts::attr(),
                ),
            })
            .enumerate()
            .collect();

        let (args_decl, args_call) = args_info.iter().fold(
            ("".to_string(), "".to_string()),
            |(decl_acc, args_acc), (idx, DartArg { arg, ty })| {
                let field = &variant.fields[*idx];
                let to_raw = if field.is_enum() { ".index" } else { "" };
                (
                    format!(
                        "{acc}{ty} {arg}, ",
                        acc = decl_acc,
                        ty = ty,
                        arg = arg
                    ),
                    format!(
                        "{acc}{arg}{to_raw}, ",
                        acc = args_acc,
                        arg = arg,
                        to_raw = to_raw
                    ),
                )
            },
        );
        let posted_reply_type =
            reply_class_name_for_enum(&self.reply_dart_enum_name);

        format!(
            r###"
{comment}  Future<{PostedReply}> {msgApiMethod}({args_decl}{{Duration? timeout}}) {{
{comment}    return {_store}.{msgMethod}({args}timeout: timeout);
{comment}  }}
"###,
            _store = store_field,
            PostedReply = posted_reply_type,
            msgApiMethod = api_method_name,
            msgMethod = method_name,
            args = args_call,
            args_decl = args_decl,
            comment = comment
        )
    }
}
