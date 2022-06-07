use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, IdentFragment};
use rid_common::{
    DART_ASYNC, DART_FFI, FFI_GEN_BIND, RID_DEBUG_REPLY, RID_FFI,
    RID_MSG_TIMEOUT, STRING_TO_NATIVE_INT8, _RID_REPLY_CHANNEL,
};
use syn::Ident;

use crate::{
    attrs::TypeInfoMap,
    common::{
        derive_error, prefixes::reply_class_name_for_enum, tokens::resolve_ptr,
    },
    parse::rust_type::{RustType, TypeKind},
    render_dart::RenderDartTypeOpts,
    render_rust::{ffi_prelude, RustArg},
    reply,
};

use super::{parsed_variant::ParsedMessageVariant, ParsedMessageEnum};

pub struct MessageRenderConfig {
    pub include_ffi: bool,
    pub render_reply_check: bool,
    pub dart_code_only: bool,
    pub rust_only: bool,
    pub render_utils_module: bool,
}

impl Default for MessageRenderConfig {
    fn default() -> Self {
        Self {
            include_ffi: true,
            render_reply_check: true,
            dart_code_only: false,
            rust_only: false,
            render_utils_module: true,
        }
    }
}

impl MessageRenderConfig {
    pub fn bare() -> Self {
        Self {
            include_ffi: false,
            render_reply_check: false,
            dart_code_only: false,
            rust_only: false,
            render_utils_module: false,
        }
    }
}

impl ParsedMessageEnum {
    /// Renders this message enum and returns a tuple of the fully rendered tokens and
    /// a separate copy of the dart string. The latter is mainly used when testing.
    /// When generating code the first one is what should be used.
    pub fn render(
        &self,
        config: &MessageRenderConfig,
    ) -> (TokenStream, String) {
        if self.parsed_variants.is_empty() {
            return (TokenStream::new(), "".to_string());
        }
        let method_tokens = self
            .parsed_variants
            .iter()
            .map(|v| self.render_rust_method(v, config));
        let dart_comment = self.render_dart_extension(config);
        let module_ident = &self.module_ident;
        let reply_check = if config.render_reply_check {
            self.render_reply_check()
        } else {
            TokenStream::new()
        };

        // Don't include dart in rust if we only want rust but also if the dart
        // comments contain code only which is not parseable as rust.
        let dart_tokens: TokenStream =
            if !config.rust_only && !config.dart_code_only {
                dart_comment.parse().unwrap()
            } else {
                TokenStream::new()
            };
        (
            quote_spanned! { self.ident.span() =>
                mod #module_ident {
                  use super::*;
                  #dart_tokens
                  #(#method_tokens)*
                }
                #reply_check
            },
            dart_comment,
        )
    }

    //
    // Rust Methods
    //

    fn render_rust_method(
        &self,
        variant: &ParsedMessageVariant,
        config: &MessageRenderConfig,
    ) -> TokenStream {
        let variant_ident = &variant.ident;

        let fn_ident = &variant.method_ident;
        let struct_ident = &self.struct_ident;
        let resolve_struct_ptr = resolve_ptr(&struct_ident);

        let enum_ident = &self.ident;

        let arg_idents: Vec<RustArg> = variant
            .fields
            .iter()
            .enumerate()
            .map(|(slot, f)| RustArg::from(&f.rust_ty, slot))
            .flatten()
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
                        slot == 0,
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

        let msg_args = if arg_idents.is_empty() {
            vec![]
        } else {
            let last_slot = arg_idents.len() - 1;
            arg_idents
                .iter()
                .enumerate()
                .map(|(slot, RustArg { arg_ident, virt, .. })| {
                    if !virt {
                        Some(if slot == last_slot {
                            quote_spanned! { fn_ident.span() => #arg_ident }
                        } else {
                            quote_spanned! { fn_ident.span() => #arg_ident, }
                        })
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        };

        let req_id_ident = format_ident!("__rid_req_id");
        let msg_ident = format_ident!("__rid_msg");

        // TODO: getting error in the right place if the model struct doesn't implement update at
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
                let #msg_ident = #enum_ident::#variant_ident(#(#msg_args)*);
            }
        };

        let ffi_prelude = if config.include_ffi {
            ffi_prelude()
        } else {
            TokenStream::new()
        };

        quote_spanned! { variant_ident.span() =>
             #ffi_prelude fn #fn_ident(#req_id_ident: u64#(#args)* ) {
                #(#args_resolvers_tokens)*
                #msg
                #update_method
            }
        }
    }

    fn render_reply_check(&self) -> TokenStream {
        let reply_ident = &self.config.reply;
        let reply_mod_ident =
            format_ident!("__rid_ensuring_{}_is_defined", reply_ident);
        quote! {
            #[allow(non_snake_case, unused_imports, unused_variables)]
            mod #reply_mod_ident {
                use super::*;
                fn __(verify_reply_enum_exists: #reply_ident) {}
            }
        }
    }

    //
    // Dart Methods
    //
    fn render_dart_extension(&self, config: &MessageRenderConfig) -> String {
        let class_name = reply_class_name_for_enum(&self.reply_dart_enum_name);
        let comment = if config.dart_code_only {
            "".to_string()
        } else {
            "///".to_string()
        };
        let methods: Vec<String> = self
            .parsed_variants
            .iter()
            .map(|x| self.render_dart_method(x, &class_name, &comment))
            .collect();

        let reply_with_timeout = if config.dart_code_only {
            "".to_string()
        } else {
            format!(
                r###"{comment}
{comment} Future<{class_name}> _replyWithTimeout(
{comment}   Future<{class_name}> reply,
{comment}   String msgCall,
{comment}   StackTrace applicationStack,
{comment}   Duration timeout,
{comment} ) {{
{comment}   final failureMsg = '''$msgCall timed out\n
{comment} ---- Application Stack ----\n
{comment} $applicationStack\n
{comment} ---- Internal Stack ----
{comment} ''';
{comment} 
{comment}   return reply.timeout(timeout,
{comment}       onTimeout: () => throw {dart_async}.TimeoutException(failureMsg, timeout));
{comment} }}
{comment}"###,
                class_name = class_name,
                dart_async = DART_ASYNC,
                comment = comment
            )
        };

        let raw_api = format!(
            r###"{reply_with_timeout}
{comment} extension Rid_Message_ExtOnPointer{struct_ident}For{enum_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{raw_struct_ident}> {{
{methods}
{comment} }}"###,
            reply_with_timeout = reply_with_timeout,
            enum_ident = self.ident,
            struct_ident = self.struct_ident,
            raw_struct_ident = self.raw_struct_ident,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            methods = methods.join("\n"),
            comment = comment
        );

        let store_api = self.render_store_api(&comment);

        if config.dart_code_only {
            format!(
                "{raw_api}\n{store_api}",
                raw_api = raw_api,
                store_api = store_api
            )
        } else {
            format!(
                r###"{comment}
{comment} The below extension provides convenience methods to send messages to rust.
{comment}
{comment} ```dart
{raw_api}
{store_api}
{comment} ```"###,
                raw_api = raw_api,
                store_api = store_api,
                comment = comment,
            )
        }
    }

    fn render_dart_method(
        &self,
        variant: &ParsedMessageVariant,
        class_name: &str,
        comment: &str,
    ) -> String {
        let fn_ident = &variant.method_ident;

        enum ArgType {
            Vector(Box<RustType>),
            Hashmap,
            Other,
        }

        struct DartArg {
            arg: String,
            arg_type: ArgType,
            ty: String,
            ffi_arg: String,
        }

        // NOTE: we don't support data of custom types inside message variants
        // Only primitives and Strings are allowed
        let type_infos = TypeInfoMap::default();

        let args_info: Vec<(usize, DartArg)> = variant
            .fields
            .iter()
            .map(|f| {
                let ffi_arg = f.dart_ty.render_resolved_ffi_arg(f.slot);
                DartArg {
                    arg: format!("arg{}", f.slot),
                    arg_type: match &f.rust_ty.kind {
                        TypeKind::Composite(_, Some(typ), None) => {
                            ArgType::Vector(typ.clone())
                        }
                        TypeKind::Composite(_, Some(_), Some(_)) => {
                            ArgType::Hashmap
                        }
                        _ => ArgType::Other,
                    },
                    ty: f.rust_ty.render_dart_type(
                        &type_infos,
                        RenderDartTypeOpts::attr_raw(),
                    ),
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

        let additional_processing = args_info.iter().fold(
            String::new(),
            |acc, (index, DartArg { arg_type, arg, .. })| {
                match arg_type {
                    ArgType::Vector(typ) if typ.kind.is_numeric() => {
                        let byte_size = typ.kind.get_numeric_size().expect("Numeric type without a bytesize?");
                        let code = format!(
                            "
{comment}    // Conversion into a C-compatible array.
{comment}    final {arg}_data = calloc<Uint8>({arg}.length * {byte_size});
{comment}    final {arg}_len = {arg}.length;
{comment}    var {arg}_counter = 0;
{comment}    for (int i = 0; i < {arg}_len; i++) {{
{comment}      for (int j = 0; j < {byte_size}; j++) {{
{comment}        {arg}_data[{arg}_counter] = ({arg}[i] >> j * 8) & 0xFF;
{comment}        {arg}_counter++;
{comment}      }}
{comment}    }}
                    "
                        );
                        format!("{acc}{code}\n")
                    }
                    ArgType::Vector(typ) if typ.kind.is_string_like() => {
                        let code = format!(
                            "
 {comment}      // Turn Dart strings into byte arrays.
 {comment}      final List<Pointer<Int8>> {arg}_utf8PointerList = [
 {comment}          for (final str in {arg}) 
 {comment}              str.toNativeUtf8().cast<Int8>()
 {comment}      ];
 {comment}      
 {comment}      // Reserve memory for string pointers
 {comment}      final Pointer<Pointer<Int8>> {arg}_data =
 {comment}          malloc.allocate(sizeOf<Pointer<Int8>>() * {arg}_utf8PointerList.length);
 {comment}
 {comment}      // Set string pointers to point to actual strings
 {comment}      for (int index = 0; index < {arg}.length; index++) {{
 {comment}        {arg}_data[index] = {arg}_utf8PointerList[index];
 {comment}      }}
 {comment}      
 {comment}      int {arg}_len = {arg}.length;
                            "
                        );
                        format!("{acc}{code}\n")
                    }
                    ArgType::Vector(a) => {
                        unimplemented!("Vector of type {:#?} currently isn't supported!", a);
                    }
                    ArgType::Hashmap => {
                        //TODO: Implement hashmap
                        acc
                    }
                    ArgType::Other => acc,
                }
            },
        );

        let deallocators = args_info.iter().fold(
            String::new(),
            |acc, (index, DartArg { arg_type, arg, .. })| {
                match arg_type {
                    ArgType::Vector(typ) if typ.kind.is_numeric() => {
                        //TODO: Fix memory leaks!
                        let byte_size = typ
                            .kind
                            .get_numeric_size()
                            .expect("Numeric type without a bytesize?");
                        let code = format!(
                            "
{comment}      calloc.free({arg}_data);
                            "
                        );
                        format!("{acc}{code}\n")
                    }
                    ArgType::Vector(typ) if typ.kind.is_string_like() => {
                        //TODO: Fix memory leaks!
                        let code = format!(
                            "
{comment}      for (int index = 0; index < {arg}.length; index++) {{
{comment}        calloc.free({arg}_data[index]);
{comment}      }}
{comment}      calloc.free({arg}_data);
                            "
                        );
                        format!("{acc}{code}\n")
                    }
                    ArgType::Vector(a) => {
                        unimplemented!(
                            "Vector of type {:#?} currently isn't supported!",
                            a
                        );
                    }
                    ArgType::Hashmap => {
                        //TODO: Implement hashmap
                        acc
                    }
                    ArgType::Other => acc,
                }
            },
        );

        let (args_call, args_string) = args_info.iter().fold(
            ("".to_string(), "".to_string()),
            |(args_acc, args_string_acc),
             (
                idx,
                DartArg {
                    ffi_arg,
                    arg,
                    arg_type,
                    ..
                },
            )| {
                let comma = if *idx == 0 { "" } else { ", " };

                match arg_type {
                    ArgType::Vector(_) => (
                        format!(
                            "{acc}{comma}{arg}_len, {arg}_data",
                            acc = args_acc,
                            comma = comma,
                            arg = ffi_arg
                        ),
                        format!(
                            "{acc}{comma}${arg}_len, ${arg}_data",
                            acc = args_string_acc,
                            comma = comma,
                            arg = arg
                        ),
                    ),
                    ArgType::Hashmap | ArgType::Other => (
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
                    ),
                }
            },
        );

        // NOTE: related code rendered via src/reply/render_reply_dart.rs, i.e. RID_DEBUG_REPLY
        format!(
            r###"
{comment}   Future<{class_name}> {dart_method_name}({args_decl}{{Duration? timeout}}) {{
{comment}     final reqId = {_RID_REPLY_CHANNEL}.reqId;
{comment}     {additional_processing}
{comment}     {rid_ffi}.{method_name}(reqId, {args_call});
{comment}     {deallocators}
{comment}     final reply = _isDebugMode && {rid_debug_reply} != null
{comment}         ? {_RID_REPLY_CHANNEL}.reply(reqId).then(({class_name} reply) {{
{comment}             if ({rid_debug_reply} != null) {rid_debug_reply}!(reply);
{comment}             return reply;
{comment}           }})
{comment}         : {_RID_REPLY_CHANNEL}.reply(reqId);
{comment}     
{comment}     if (!_isDebugMode) return reply;
{comment}
{comment}     timeout ??= {rid_msg_timeout};
{comment}     if (timeout == null) return reply;
{comment}     final msgCall = '{dart_method_name}({args_string}) with reqId: $reqId';
{comment}     return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
{comment}   }}"###,
            class_name = class_name,
            dart_method_name = self.dart_method_name(&fn_ident.to_string()),
            method_name = fn_ident.to_string(),
            args_decl = args_decl,
            args_call = args_call,
            args_string = args_string,
            rid_ffi = RID_FFI,
            _RID_REPLY_CHANNEL = _RID_REPLY_CHANNEL,
            rid_debug_reply = RID_DEBUG_REPLY,
            rid_msg_timeout = RID_MSG_TIMEOUT,
            comment = comment
        )
    }

    pub fn dart_method_name(&self, rust_method_name: &str) -> String {
        // Cut off "rid_msg_
        let shortened = rust_method_name[8..].to_string();
        // lowercase first char
        format!("{}{}", self.ident_lower_camel, shortened)
    }
}
