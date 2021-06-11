use super::{parsed_variant::ParsedVariant, store::code_store_module};
use crate::{
    attrs::{self, EnumConfig, TypeInfoMap},
    common::{
        errors::derive_error,
        prefixes::reply_class_name_for_enum,
        rust::RustType,
        tokens::{instance_ident, resolve_ptr, resolve_string_ptr},
    },
    parse::rust_type,
    render_rust::RustArg,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, IdentFragment};
use rid_common::{
    DART_ASYNC, DART_FFI, FFI_GEN_BIND, RID_DEBUG_REPLY, RID_FFI,
    RID_MSG_TIMEOUT, STRING_TO_NATIVE_INT8,
};
use std::collections::HashMap;
use syn::{punctuated::Punctuated, token::Comma, Ident, Variant};

type Tokens = proc_macro2::TokenStream;

pub struct ParsedEnum {
    /// The enum itself, i.e. Msg
    pub ident: syn::Ident,

    /// The enum variants, i.e. AddTodo(String)
    pub parsed_variants: Vec<ParsedVariant>,

    /// Prefix used for all message methods, i.e. rid_msg
    pub method_prefix: String,

    /// The identifier of the struct receiving the message, i.e. Store
    struct_ident: syn::Ident,

    /// Identifier of the module into which the hidden code is wrapped
    module_ident: syn::Ident,

    /// The name of the enum used to reply to messages
    reply_dart_enum_name: String,

    ident_lower_camel: String,
    config: EnumConfig,
}

impl ParsedEnum {
    pub fn new(
        ident: &Ident,
        variants: Punctuated<Variant, Comma>,
        config: EnumConfig,
    ) -> Self {
        let ident_str = ident.to_string();
        let ident_lower_camel = lower_camel_case(&ident_str);
        let ident_lower = ident_str.to_lowercase();
        let method_prefix = format!("rid_{}", ident_lower);
        let module_ident = format_ident!("__rid_{}_ffi", ident_lower);

        let parsed_variants =
            parse_variants(variants, &method_prefix, &config.type_infos);
        let struct_ident = format_ident!("{}", config.to);
        let reply_ident =
            crate::parse::rust_type::RustType::from_owned_enum(&config.reply);
        let reply_dart_enum_name = reply_ident.dart_ident(false).to_string();

        Self {
            ident: ident.clone(),
            reply_dart_enum_name,
            parsed_variants,
            method_prefix,
            struct_ident,
            module_ident,
            ident_lower_camel,
            config,
        }
    }

    pub fn tokens(&self) -> Tokens {
        if self.parsed_variants.is_empty() {
            return Tokens::new();
        }
        let method_tokens =
            self.parsed_variants.iter().map(|v| self.rust_method(v));
        let dart_comment = self.dart_extension();
        let module_ident = &self.module_ident;

        let store_module = code_store_module(&self.ident, &self.struct_ident);
        let reply_ident = &self.config.reply;
        let reply_mod_ident =
            format_ident!("__rid_ensuring_{}_is_defined", reply_ident);

        quote_spanned! { self.ident.span() =>
            #store_module
            mod #module_ident {
              use super::*;
              #dart_comment
              #(#method_tokens)*
            }
            #[allow(non_snake_case, unused_imports, unused_variables)]
            mod #reply_mod_ident {
                use super::*;
                fn __(verify_reply_enum_exists: #reply_ident) {}
            }
        }
    }

    //
    // Rust Methods
    //

    fn rust_method(&self, variant: &ParsedVariant) -> Tokens {
        use crate::common::rust::ValueType::*;

        let variant_ident = &variant.ident;

        if variant.has_errors() {
            return variant
                .errors
                .iter()
                .map(|err| derive_error(variant_ident, err))
                .collect();
        }

        let fn_ident = &variant.method_ident;
        let struct_ident = &self.struct_ident;
        let resolve_struct_ptr = resolve_ptr(&struct_ident);

        let enum_ident = &self.ident;

        let arg_idents: Vec<RustArg> = variant
            .fields
            .iter()
            .enumerate()
            .map(|(slot, f)| RustArg::from(&f.rust_ty, slot))
            .collect();

        let args = if arg_idents.is_empty() {
            vec![]
        } else {
            let last_slot = arg_idents.len() - 1;
            arg_idents
                .iter()
                .enumerate()
                .map(|(slot, arg)| {
                    arg.render_typed_parameter(
                        Some(fn_ident.span()),
                        slot != last_slot,
                    )
                })
                .collect()
        };

        let args_resolvers_tokens = arg_idents.iter().map(
            |RustArg {
                 resolver_tokens, ..
             }| resolver_tokens,
        );

        let msg_args = arg_idents
            .iter()
            .map(|RustArg { arg_ident, .. }| quote_spanned! { fn_ident.span() => #arg_ident });

        let req_id_ident = format_ident!("__rid_req_id");
        let msg_ident = format_ident!("__rid_msg");

        // TODO: getting error in the right place if the model struct doesn't implement udpate at
        // all, however when it is implemented incorrectly then the error doesn't even mention the
        // method name
        let update_method = quote_spanned! { self.struct_ident.span() =>
            store::write().update(#req_id_ident, #msg_ident);
        };

        let msg = if msg_args.len() == 0 {
            quote_spanned! { variant_ident.span() =>
                let #msg_ident = #enum_ident::#variant_ident;
            }
        } else {
            quote_spanned! { variant_ident.span() =>
                let #msg_ident = #enum_ident::#variant_ident(#(#msg_args,)*);
            }
        };

        quote_spanned! { variant_ident.span() =>
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #fn_ident(#req_id_ident: u64, #(#args)* ) {
                #(#args_resolvers_tokens)*
                #msg
                #update_method
            }
        }
    }

    //
    // Dart Methods
    //
    fn dart_extension(&self) -> Tokens {
        let class_name = reply_class_name_for_enum(&self.reply_dart_enum_name);
        let methods: Vec<String> = self
            .parsed_variants
            .iter()
            .map(|x| self.dart_method(x, &class_name))
            .collect();

        let s = format!(
            r###"
/// The below extension provides convenience methods to send messages to rust.
///
/// ```dart
/// final Duration? RID_MSG_TIMEOUT = const Duration(milliseconds: 200);
/// 
/// Future<{class_name}> _replyWithTimeout(
///   Future<{class_name}> reply,
///   String msgCall,
///   StackTrace applicationStack,
///   Duration timeout,
/// ) {{
///   final failureMsg = '''$msgCall timed out\n
/// ---- Application Stack ----\n
/// $applicationStack\n
/// ---- Internal Stack ----
/// ''';
/// 
///   return reply.timeout(timeout,
///       onTimeout: () => throw {dart_async}.TimeoutException(failureMsg, timeout));
/// }}
/// 
/// extension Rid_Message_ExtOnPointer{struct_ident}For{enum_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{
  {methods}
/// }}
/// ```
        "###,
            class_name = class_name,
            enum_ident = self.ident,
            struct_ident = self.struct_ident,
            dart_async = DART_ASYNC,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            methods = methods.join("\n")
        );
        s.parse().unwrap()
    }

    fn dart_method(&self, variant: &ParsedVariant, class_name: &str) -> String {
        use crate::common::rust::ValueType::*;
        let fn_ident = &variant.method_ident;
        struct DartArg {
            arg: String,
            ty: String,
            ffi_arg: String,
        }

        let args_info: Vec<(usize, DartArg)> = variant
            .fields
            .iter()
            .map(|f| {
                let ffi_arg = f.dart_ty.render_resolved_ffi_arg(f.slot);
                DartArg {
                    arg: format!("arg{}", f.slot),
                    ty: f.rust_ty.render_dart_type(true),
                    ffi_arg,
                }
            })
            .enumerate()
            .collect();

        let args_decl = args_info.iter().fold(
            "".to_string(),
            |acc, (idx, DartArg { arg, ty, .. })| {
                format!("{acc}{ty} {arg}, ", acc = acc, ty = ty, arg = arg)
            },
        );

        let (args_call, args_string) = args_info.iter().fold(
            ("".to_string(), "".to_string()),
            |(args_acc, args_string_acc),
             (idx, DartArg { ffi_arg, arg, .. })| {
                let comma = if *idx == 0 { "" } else { ", " };
                (
                    format!(
                        "{acc}{comma}{arg}",
                        acc = args_acc,
                        comma = comma,
                        arg = ffi_arg
                    ),
                    format!(
                        "{acc}{comma}${arg}",
                        acc = args_string_acc,
                        comma = comma,
                        arg = arg
                    ),
                )
            },
        );

        // NOTE: related code rendered via src/reply/render_reply_dart.rs, i.e. RID_DEBUG_REPLY
        format!(
            r###"
///   Future<{class_name}> {dart_method_name}({args_decl}{{Duration? timeout}}) {{
///     final reqId = replyChannel.reqId;
///     {rid_ffi}.{method_name}(reqId, {args_call});
///
///     final reply = _isDebugMode && {rid_debug_reply} != null
///         ? replyChannel.reply(reqId).then(({class_name} reply) {{
///             if ({rid_debug_reply} != null) {rid_debug_reply}!(reply);
///             return reply;
///           }})
///         : replyChannel.reply(reqId);
///     
///     if (!_isDebugMode) return reply;
///
///     timeout ??= {rid_msg_timeout};
///     if (timeout == null) return reply;
///     final msgCall = 'msgInit({args_string}) with reqId: $reqId';
///     return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
///   }}
/// "###,
            class_name = class_name,
            dart_method_name = self.dart_method_name(&fn_ident.to_string()),
            method_name = fn_ident.to_string(),
            args_decl = args_decl,
            args_call = args_call,
            args_string = args_string,
            rid_ffi = RID_FFI,
            rid_debug_reply = RID_DEBUG_REPLY,
            rid_msg_timeout = RID_MSG_TIMEOUT,
        )
    }

    fn dart_method_name(&self, rust_method_name: &str) -> String {
        // Cut off "rid_msg_
        let shortened = rust_method_name[8..].to_string();
        // lowercase first char
        format!("{}{}", self.ident_lower_camel, shortened)
    }
}

fn parse_variants(
    variants: Punctuated<Variant, Comma>,
    method_prefix: &str,
    types: &TypeInfoMap,
) -> Vec<ParsedVariant> {
    variants
        .into_iter()
        .map(|v| ParsedVariant::new(v, &method_prefix, types))
        .collect()
}

fn lower_camel_case(s: &str) -> String {
    format!("{}{}", s[0..1].to_lowercase(), s[1..].to_string())
}
