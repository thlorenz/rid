use super::{parsed_variant::ParsedVariant, store::code_store_module};
use crate::{
    attrs::{self, EnumConfig, TypeInfoMap},
    common::{
        errors::derive_error,
        rust::RustType,
        tokens::{instance_ident, resolve_ptr, resolve_string_ptr},
    },
    parse::rust_type,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use rid_common::{DART_FFI, FFI_GEN_BIND, RID_FFI, STRING_TO_NATIVE_INT8};
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
        Self {
            ident: ident.clone(),
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

        quote_spanned! { self.ident.span() =>
            #store_module
            mod #module_ident {
              use super::*;
              #dart_comment
              #(#method_tokens)*
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

        struct Arg {
            arg: syn::Ident,
            ty: Tokens,
            resolver: Tokens,
        }

        let arg_idents: Vec<Arg> = variant
            .fields
            .iter()
            .map(|f| match &f.rust_ty {
                RustType::Value(RString) => {
                    let arg = format_ident!("arg{}", f.slot);
                    let ty = quote_spanned! { arg.span() =>  *mut ::std::os::raw::c_char };
                    let resolver = resolve_string_ptr(&arg, true);
                    Arg { arg, ty, resolver }
                }
                RustType::Value(CString) => {
                    todo!("ParsedEnum::rust_method::RustType::Value(CString)")
                }
                RustType::Value(RVec(_)) => {
                    todo!("ParsedEnum::rust_method::RustType::Value(RVec(_))")
                }
                RustType::Value(RCustom(_, p)) => {
                    let arg = format_ident!("arg{}", f.slot);
                    let ty = format_ident!("{}", p);
                    let ty = quote_spanned! { arg.span() => #ty };
                    Arg {
                        arg,
                        ty,
                        resolver: Tokens::new(),
                    }
                }
                RustType::Primitive(p) => {
                    let arg = format_ident!("arg{}", f.slot);
                    let ty = format_ident!("{}", p.to_string());
                    let ty = quote_spanned! { arg.span() => #ty };
                    Arg {
                        arg,
                        ty,
                        resolver: Tokens::new(),
                    }
                }
                RustType::Unit => todo!("ParsedEnum::rust_method::RustType::Unit"),
                RustType::Unknown => unimplemented!("RustType::Unknown"),
            })
            .collect();

        let args = arg_idents
            .iter()
            .map(|Arg { arg, ty, .. }| quote_spanned! { fn_ident.span() => #arg: #ty });

        let args_resolvers =
            arg_idents.iter().map(|Arg { resolver, .. }| resolver);

        let msg_args = arg_idents
            .iter()
            .map(|Arg { arg, .. }| quote_spanned! { fn_ident.span() => #arg });

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
            pub extern "C" fn #fn_ident(#req_id_ident: u64, #(#args,)* ) {
                #(#args_resolvers)*
                #msg
                #update_method
            }
        }
    }

    //
    // Dart Methods
    //
    fn dart_extension(&self) -> Tokens {
        let methods: Vec<String> = self
            .parsed_variants
            .iter()
            .map(|x| self.dart_method(x))
            .collect();

        // TODO: generate via #rid::post
        let reply_impl = r###"
/// 
/// enum Post {
///   Started,
///   Stopped,
///   Reset,
///   Tick,
/// }
/// 
/// class Reply extends IReply {
///   final Post post;
///   final int? reqId;
///   final String? data; 
/// 
///   Reply(this.post, this.reqId, this.data);
/// 
///   @override
///   String toString() {
///     return '''Reply {
///   post:  ${this.post.toString().substring('Post.'.length)}
///   reqId: $reqId
///   data:  $data
/// }
/// ''';
///   }
/// }
/// 
/// const int _POST_MASK = 0x000000000000ffff;
/// const int _I64_MIN = -9223372036854775808;
/// 
/// Reply decode(int packed, String? data) {
///   final npost = packed & _POST_MASK;
///   final id = (packed - _I64_MIN) >> 16;
///   final reqId = id > 0 ? id : null;
/// 
///   final post = Post.values[npost];
///   return Reply(post, reqId, data);
/// }
/// 
/// final ReplyChannel<Reply> replyChannel =
///     ReplyChannel.instance(_dl, decode);
        "###;

        let s = format!(
            r###"
/// The below extension provides convenience methods to send messages to rust.
///
/// ```dart
{reply_impl}
/// extension Rid_Message_ExtOnPointer{struct_ident}For{enum_ident} on {dart_ffi}.Pointer<{ffigen_bind}.{struct_ident}> {{
  {methods}
/// }}
/// ```
        "###,
            reply_impl = reply_impl,
            enum_ident = self.ident,
            struct_ident = self.struct_ident,
            dart_ffi = DART_FFI,
            ffigen_bind = FFI_GEN_BIND,
            methods = methods.join("\n")
        );
        s.parse().unwrap()
    }

    fn dart_method(&self, variant: &ParsedVariant) -> String {
        use crate::common::rust::ValueType::*;
        let fn_ident = &variant.method_ident;
        struct Arg {
            arg: String,
            ty: String,
            ffi_arg: String,
        }

        let args_info: Vec<(usize, Arg)> = variant
            .fields
            .iter()
            .map(|f| {
                let ffi_arg = match f.rust_ty {
                    RustType::Value(RString) => format!(
                        "arg{slot}.{toNativeInt8}()",
                        slot = f.slot,
                        toNativeInt8 = STRING_TO_NATIVE_INT8
                    ),
                    RustType::Value(CString) => {
                        todo!(
                            "ParsedEnum::dart_method::RustType::Value(CString)"
                        )
                    }
                    RustType::Value(RVec(_)) => {
                        todo!(
                            "ParsedEnum::dart_method::RustType::Value(RVec(_))"
                        )
                    }
                    RustType::Primitive(_) => format!("arg{}", f.slot),
                    RustType::Value(RCustom(_, _)) => format!("arg{}", f.slot),

                    RustType::Unit => {
                        todo!("ParsedEnum::dart_method::RustType::Unit")
                    }
                    RustType::Unknown => {
                        unimplemented!(
                            "ParsedEnum::dart_method::RustType::Unknown"
                        )
                    }
                };
                let ty = if let RustType::Value(RCustom(info, _)) = &f.rust_ty {
                    use attrs::Category::*;
                    match info.cat {
                        Enum => "int".to_string(),
                        Struct => todo!("parsed_enum::rust_method Struct"),
                        Prim => todo!("parsed_enum::rust_method Prim"),
                    }
                } else {
                    f.dart_ty.to_string()
                };
                Arg {
                    arg: format!("arg{}", f.slot),
                    ty,
                    ffi_arg,
                }
            })
            .enumerate()
            .collect();

        let args_decl = args_info.iter().fold(
            "".to_string(),
            |acc, (idx, Arg { arg, ty, .. })| {
                let comma = if *idx == 0 { "" } else { ", " };
                format!(
                    "{acc}{comma}{ty} {arg}",
                    acc = acc,
                    comma = comma,
                    ty = ty,
                    arg = arg
                )
            },
        );

        let args_call = args_info.iter().fold(
            "".to_string(),
            |acc, (idx, Arg { ffi_arg, .. })| {
                let comma = if *idx == 0 { "" } else { ", " };
                format!(
                    "{acc}{comma}{arg}",
                    acc = acc,
                    comma = comma,
                    arg = ffi_arg
                )
            },
        );

        format!(
            r###"
///   Future<Reply> {dart_method_name}({args_decl}) {{
///     final reqId = replyChannel.reqId;
///     {rid_ffi}.{method_name}(reqId, {args_call});
///     return replyChannel.reply(reqId);
///   }}
/// "###,
            dart_method_name = self.dart_method_name(&fn_ident.to_string()),
            method_name = fn_ident.to_string(),
            args_decl = args_decl,
            args_call = args_call,
            rid_ffi = RID_FFI,
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
