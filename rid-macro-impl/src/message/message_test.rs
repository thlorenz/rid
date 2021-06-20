use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rid_common::STORE;
use syn::{parse_macro_input, Ident, Item};

use crate::{
    attrs::{self, RidAttr},
    common::{abort, dump_code, dump_tokens, normalize_code},
    message::{MessageEnumConfig, ParsedMessageEnum},
};

use super::render_message_enum::MessageRenderConfig;

fn render(
    input: TokenStream,
    config: &MessageRenderConfig,
) -> (TokenStream, String) {
    let item = syn::parse2::<syn::Item>(input).unwrap();
    let rid_args: Vec<Ident> = vec![format_ident!("Reply")];
    let rid_attrs: Vec<RidAttr> = vec![];
    match item {
        Item::Enum(item) => {
            let rid_attrs = attrs::parse_rid_attrs(&item.attrs);
            let enum_config = MessageEnumConfig::new(
                &rid_attrs,
                format_ident!("{}", STORE),
                &rid_args[0],
            );
            let parsed_enum = ParsedMessageEnum::new(
                &item.ident,
                item.variants.clone(),
                enum_config,
            );
            parsed_enum.render(&config)
        }
        _ => {
            abort!(item, "rid::message attribute can only be applied to enums")
        }
    }
}

fn render_rust(input: &TokenStream) -> TokenStream {
    render(
        input.clone(),
        &MessageRenderConfig {
            rust_only: true,
            ..MessageRenderConfig::bare()
        },
    )
    .0
}

fn render_dart(input: &TokenStream) -> String {
    render(
        input.clone(),
        &MessageRenderConfig {
            dart_code_only: true,
            ..MessageRenderConfig::bare()
        },
    )
    .1
}

// -----------------
// Messages without Fields
// -----------------
mod msg_variants_without_fields {
    use super::*;

    #[test]
    fn msg_init() {
        let msg = quote! {
            pub enum Msg {
                Init,
            }
        };

        let expected_rust = quote! {
            mod __rid_msg_ffi {
                use super::*;
                fn rid_msg_Init(__rid_req_id: u64) {
                    let __rid_msg = Msg::Init;
                    store::write().update(__rid_req_id, __rid_msg);
                }
            }
        };
        let expected_dart = r###"
            extension Rid_Message_ExtOnPointerStoreForMsg on dart_ffi.Pointer<ffigen_bind.RawStore> {
            Future<PostedReply> msgInit({Duration? timeout}) {
                final reqId = replyChannel.reqId;
                rid_ffi.rid_msg_Init(reqId, );

                final reply = _isDebugMode && RID_DEBUG_REPLY != null
                    ? replyChannel.reply(reqId).then((PostedReply reply) {
                        if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
                        return reply;
                    })
                    : replyChannel.reply(reqId);

                if (!_isDebugMode) return reply;

                timeout ??= RID_MSG_TIMEOUT;
                if (timeout == null) return reply;
                final msgCall = 'msgInit() with reqId: $reqId';
                return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
            }
            }
        "###;
        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        assert_eq!(rust.to_string().trim(), expected_rust.to_string().trim());
        assert_eq!(normalize_code(&dart), normalize_code(expected_dart));
    }

    #[test]
    fn msg_init_deinit() {
        let msg = quote! {
            pub enum Msg {
                Init,
                Deinit
            }
        };

        let expected_rust = quote! {
            mod __rid_msg_ffi {
                use super::*;
                fn rid_msg_Init(__rid_req_id: u64) {
                    let __rid_msg = Msg::Init;
                    store::write().update(__rid_req_id, __rid_msg);
                }
                fn rid_msg_Deinit(__rid_req_id: u64) {
                    let __rid_msg = Msg::Deinit;
                    store::write().update(__rid_req_id, __rid_msg);
                }
            }
        };
        let expected_dart = r###"
            extension Rid_Message_ExtOnPointerStoreForMsg on dart_ffi.Pointer<ffigen_bind.RawStore> {

              Future<PostedReply> msgInit({Duration? timeout}) {
                  final reqId = replyChannel.reqId;
                  rid_ffi.rid_msg_Init(reqId, );

                  final reply = _isDebugMode && RID_DEBUG_REPLY != null
                      ? replyChannel.reply(reqId).then((PostedReply reply) {
                          if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
                          return reply;
                      })
                      : replyChannel.reply(reqId);

                  if (!_isDebugMode) return reply;

                  timeout ??= RID_MSG_TIMEOUT;
                  if (timeout == null) return reply;
                  final msgCall = 'msgInit() with reqId: $reqId';
                  return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
              }

              Future<PostedReply> msgDeinit({Duration? timeout}) {
                  final reqId = replyChannel.reqId;
                  rid_ffi.rid_msg_Deinit(reqId, );

                  final reply = _isDebugMode && RID_DEBUG_REPLY != null
                      ? replyChannel.reply(reqId).then((PostedReply reply) {
                          if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
                          return reply;
                      })
                      : replyChannel.reply(reqId);

                  if (!_isDebugMode) return reply;

                  timeout ??= RID_MSG_TIMEOUT;
                  if (timeout == null) return reply;
                  final msgCall = 'msgDeinit() with reqId: $reqId';
                  return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
              }
            }
        "###;
        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        dump_code(&dart);
        assert_eq!(rust.to_string().trim(), expected_rust.to_string().trim());
        assert_eq!(normalize_code(&dart), normalize_code(expected_dart));
    }
}

// -----------------
// Messages with primitive Fields
// -----------------
mod msg_variants_with_primitive_fields {
    use super::*;

    #[test]
    fn msg_add_id() {
        let msg = quote! {
            pub enum Msg {
                Add(u32)
            }
        };

        let expected_rust = quote! {
            mod __rid_msg_ffi {
                use super::*;
                fn rid_msg_Add(__rid_req_id: u64, arg0: u32) {
                    let __rid_msg = Msg::Add(arg0);
                    store::write().update(__rid_req_id, __rid_msg);
                }
            }
        };
        let expected_dart = r###"
            extension Rid_Message_ExtOnPointerStoreForMsg on dart_ffi.Pointer<ffigen_bind.RawStore> {

              Future<PostedReply> msgAdd(@dart_ffi.Int32() int arg0, {Duration? timeout}) {
                  final reqId = replyChannel.reqId;
                  rid_ffi.rid_msg_Add(reqId, arg0);

                  final reply = _isDebugMode && RID_DEBUG_REPLY != null
                      ? replyChannel.reply(reqId).then((PostedReply reply) {
                          if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
                          return reply;
                      })
                      : replyChannel.reply(reqId);

                  if (!_isDebugMode) return reply;

                  timeout ??= RID_MSG_TIMEOUT;
                  if (timeout == null) return reply;
                  final msgCall = 'msgAdd($arg0) with reqId: $reqId';
                  return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
              }
            }
        "###;
        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        assert_eq!(rust.to_string().trim(), expected_rust.to_string().trim());
        assert_eq!(normalize_code(&dart), normalize_code(expected_dart));
    }

    #[test]
    fn msg_add_id_title() {
        let msg = quote! {
            pub enum Msg {
                Add(u32, String),
            }
        };

        let expected_rust = quote! {
            mod __rid_msg_ffi {
                use super::*;
                fn rid_msg_Add(
                    __rid_req_id: u64,
                    arg0: u32,
                    arg1: *mut ::std::os::raw::c_char
                ) {
                    let arg1 = unsafe { ::std::ffi::CString::from_raw(arg1) }
                        .to_str()
                        .expect("Received String that wasn't valid UTF-8.")
                        .to_string();
                    let __rid_msg = Msg::Add(arg0, arg1);
                    store::write().update(__rid_req_id, __rid_msg);
                }
            }
        };
        let expected_dart = r###"
            extension Rid_Message_ExtOnPointerStoreForMsg on dart_ffi.Pointer<ffigen_bind.RawStore> {
           
              Future<PostedReply> msgAdd(@dart_ffi.Int32() int arg0, String arg1, {Duration? timeout}) {
                final reqId = replyChannel.reqId;
                rid_ffi.rid_msg_Add(reqId, arg0, arg1.toNativeInt8());
           
                final reply = _isDebugMode && RID_DEBUG_REPLY != null
                    ? replyChannel.reply(reqId).then((PostedReply reply) {
                        if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
                        return reply;
                      })
                    : replyChannel.reply(reqId);
           
                if (!_isDebugMode) return reply;
           
                timeout ??= RID_MSG_TIMEOUT;
                if (timeout == null) return reply;
                final msgCall = 'msgAdd($arg0, $arg1) with reqId: $reqId';
                return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
              }
            }
        "###;

        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        assert_eq!(rust.to_string().trim(), expected_rust.to_string().trim());
        assert_eq!(normalize_code(&dart), normalize_code(expected_dart));
    }
}

// -----------------
// Messages with enum Fields
// -----------------
mod msg_variants_with_enum_fields {
    use super::*;

    #[test]
    fn msg_set_filter() {
        let msg = quote! {
            #[rid::enums(Filter)]
            pub enum Msg {
                SetFilter(Filter),
            }
        };

        let expected_rust = quote! {
            mod __rid_msg_ffi {
                use super::*;
                fn rid_msg_SetFilter(__rid_req_id: u64, arg0: Filter) {
                    let __rid_msg = Msg::SetFilter(arg0);
                    store::write().update(__rid_req_id, __rid_msg);
                }
            }
        };
        let expected_dart = r###"
            extension Rid_Message_ExtOnPointerStoreForMsg on dart_ffi.Pointer<ffigen_bind.RawStore> {

              Future<PostedReply> msgSetFilter(int arg0, {Duration? timeout}) {
                final reqId = replyChannel.reqId;
                rid_ffi.rid_msg_SetFilter(reqId, arg0);

                final reply = _isDebugMode && RID_DEBUG_REPLY != null
                    ? replyChannel.reply(reqId).then((PostedReply reply) {
                        if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
                        return reply;
                      })
                    : replyChannel.reply(reqId);

                if (!_isDebugMode) return reply;

                timeout ??= RID_MSG_TIMEOUT;
                if (timeout == null) return reply;
                final msgCall = 'msgSetFilter($arg0) with reqId: $reqId';
                return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
              }
            }
        "###;

        let rust = render_rust(&msg);
        let dart = render_dart(&msg);

        assert_eq!(rust.to_string().trim(), expected_rust.to_string().trim());
        assert_eq!(normalize_code(&dart), normalize_code(expected_dart));
    }
}

// -----------------
// Messages with struct Fields
// -----------------
// These are not supported since when sending a pointer to a struct we cannot be sure if it is
// still valid or not due to store changes in between obtaining and sending it.
// Therefore it is recommended to send an id instead in order to have the matching instance be
// obtained fresh on the Rust side.
