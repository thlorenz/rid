use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, ItemEnum, Token, Variant};

use crate::{
    common::prefixes::reply_class_name_for_enum,
    parse::{
        rust_type::{RustType, TypeKind, Value},
        ParsedEnum, ParsedReference,
    },
};

pub fn render_reply_dart(
    parsed_enum: &ParsedEnum,
    comment: &str,
) -> TokenStream {
    let rust_type = RustType::from_owned_enum(&parsed_enum.ident);
    let rendered_enum = parsed_enum.render_dart(comment);
    let dart_enum_name = rust_type.rust_ident().to_string();

    let class_name = reply_class_name_for_enum(&dart_enum_name);
    let posted_reply_type = reply_class_name_for_enum(&dart_enum_name);

    let rendered_reply_class = format!(
        r###"
{comment} ```dart
{comment}
{comment} class {class_name} extends IReply {{
{comment}   final {enum} type;
{comment}   final int? reqId;
{comment}   final String? data; 
{comment} 
{comment}   {class_name}._(this.type, this.reqId, this.data);
{comment} 
{comment}   @override
{comment}   String toString() {{
{comment}     return '''{class_name} {{
{comment}   type:  ${{this.type.toString().substring('{enum}.'.length)}}
{comment}   reqId: $reqId
{comment}   data:  $data
{comment} }}
{comment} ''';
{comment}   }}
{comment} }}
{comment} 
{comment} void Function({PostedReply})? RID_DEBUG_REPLY = (PostedReply reply) {{
{comment}   print('$reply');
{comment} }};
{comment}
{comment} const int _TYPE_MASK= 0x000000000000ffff;
{comment} const int _I64_MIN = -9223372036854775808;
{comment} 
{comment} {class_name} decode(int packed, String? data) {{
{comment}   final ntype = packed & _TYPE_MASK;
{comment}   final id = (packed - _I64_MIN) >> 16;
{comment}   final reqId = id > 0 ? id : null;
{comment} 
{comment}   final type = {enum}.values[ntype];
{comment}   return {class_name}._(type, reqId, data);
{comment} }}
{comment} 
{comment} final ReplyChannel<{class_name}> replyChannel = ReplyChannel.instance(_dl, decode, _isDebugMode);
{comment} ```
    "###,
        PostedReply = posted_reply_type,
        comment = comment,
        enum = dart_enum_name,
        class_name = class_name,
    );

    format!(
        r###"
{rendered_enum}
{comment}
{rendered_reply_class}
    "###,
        comment = comment,
        rendered_enum = rendered_enum,
        rendered_reply_class = rendered_reply_class
    )
    .parse()
    .unwrap()
}
