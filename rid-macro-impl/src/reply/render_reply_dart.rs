use proc_macro2::TokenStream;
use rid_common::{RID_DEBUG_REPLY, _RID_REPLY_CHANNEL};
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
{comment}   final {en} type;
{comment}   final int? reqId;
{comment}   final Uint8List? data; 
{comment} 
{comment}   {class_name}._(this.type, this.reqId, this.data);
{comment} 
{comment}   @override
{comment}   String toString() {{
{comment}     return '''{class_name} {{
{comment}   type:  ${{this.type.toString().substring('{en}.'.length)}}
{comment}   reqId: $reqId
{comment}   data:  $data
{comment} }}
{comment} ''';
{comment}   }}
{comment} String? asString() {{
{comment}     if (this.data != null) {{
{comment}         return utf8.decode(this.data!);
{comment}     }} else {{
{comment}         return null;
{comment}     }}
{comment} }}
{comment} int? asInt() {{
{comment}     if (this.data != null) {{
{comment}         int? number = null;
{comment}         if (this.data!.length == 4) {{
{comment}             number = ByteData.view(this.data!.buffer).getInt32(0, Endian.little);
{comment}         }}else if (this.data!.length == 8) {{
{comment}             number = ByteData.view(this.data!.buffer).getInt64(0, Endian.little);
{comment}         }}
{comment}         return number;
{comment}     }} else {{
{comment}         return null;
{comment}     }}
{comment} }}
{comment} int? asUint() {{
{comment}     if (this.data != null) {{
{comment}         int? number = null;
{comment}         if (this.data!.length == 4) {{
{comment}             number = ByteData.view(this.data!.buffer).getUint32(0, Endian.little);
{comment}         }}else if (this.data!.length == 8) {{
{comment}             number = ByteData.view(this.data!.buffer).getUint64(0, Endian.little);
{comment}         }}
{comment}         return number;
{comment}     }} else {{
{comment}         return null;
{comment}     }}
{comment} }}
{comment} double? asDouble() {{
{comment}     if (this.data != null) {{
{comment}         var number = ByteData.view(this.data!.buffer).getFloat64(0, Endian.little);
{comment}         return number;
{comment}     }} else {{
{comment}         return null;
{comment}     }}
{comment} }}
{comment} double? asFloat() {{
{comment}     if (this.data != null) {{
{comment}         var number = ByteData.view(this.data!.buffer).getFloat32(0, Endian.little);
{comment}         return number;
{comment}     }} else {{
{comment}         return null;
{comment}     }}
{comment} }}
{comment} List<String>? asStringArray() {{
{comment}     if (this.data != null) {{
{comment}         List<int> tmp = <int>[];
{comment}         List<String> ret = <String>[];
{comment}         for (var d in this.data!) {{
{comment}           if (d == 0){{
{comment}             var str = utf8.decode(tmp);
{comment}             ret.add(str);
{comment}             tmp.clear();
{comment}           }}else{{
{comment}             tmp.add(d);
{comment}           }}
{comment}         }}
{comment}         return ret;
{comment}     }} else {{
{comment}         return null;
{comment}     }}
{comment} }}
{comment} List<int>? asUint32Array() {{
{comment}     if (this.data != null) {{
{comment}       var length = this.data!.length;
{comment}       var ret = <int>[];
{comment}       const int INT_SIZE = 4;
{comment}       for (var i = 0; i < length/INT_SIZE; i++){{
{comment}         var number = ByteData.view(this.data!.buffer).getUint32(i*INT_SIZE, Endian.little);
{comment}         ret.add(number);
{comment}       }}
{comment}       return ret;
{comment}     }} else {{
{comment}       return null;
{comment}     }}
{comment} }}
{comment} List<int>? asUint64Array() {{
{comment}     if (this.data != null) {{
{comment}       var length = this.data!.length;
{comment}       var ret = <int>[];
{comment}       const int INT_SIZE = 8;
{comment}       for (var i = 0; i < length/INT_SIZE; i++){{
{comment}         var number = ByteData.view(this.data!.buffer).getUint64(i*INT_SIZE, Endian.little);
{comment}         ret.add(number);
{comment}       }}
{comment}       return ret;
{comment}     }} else {{
{comment}       return null;
{comment}     }}
{comment} }}
{comment} List<int>? asInt32Array() {{
{comment}     if (this.data != null) {{
{comment}       var length = this.data!.length;
{comment}       var ret = <int>[];
{comment}       const int INT_SIZE = 4;
{comment}       for (var i = 0; i < length/INT_SIZE; i++){{
{comment}         var number = ByteData.view(this.data!.buffer).getInt32(i*INT_SIZE, Endian.little);
{comment}         ret.add(number);
{comment}       }}
{comment}       return ret;
{comment}     }} else {{
{comment}       return null;
{comment}     }}
{comment} }}
{comment} List<int>? asInt64Array() {{
{comment}     if (this.data != null) {{
{comment}       var length = this.data!.length;
{comment}       var ret = <int>[];
{comment}       const int INT_SIZE = 8;
{comment}       for (var i = 0; i < length/INT_SIZE; i++){{
{comment}         var number = ByteData.view(this.data!.buffer).getInt64(i*INT_SIZE, Endian.little);
{comment}         ret.add(number);
{comment}       }}
{comment}       return ret;
{comment}     }} else {{
{comment}       return null;
{comment}     }}
{comment} }}
{comment} List<double>? asFloatArray() {{
{comment}     if (this.data != null) {{
{comment}       var length = this.data!.length;
{comment}       var ret = <double>[];
{comment}       const int FLOAT_SIZE = 4;
{comment}       for (var i = 0; i < length/FLOAT_SIZE; i++){{
{comment}         var number = ByteData.view(this.data!.buffer).getFloat32(i*FLOAT_SIZE, Endian.little);
{comment}         ret.add(number);
{comment}       }}
{comment}       return ret;
{comment}     }} else {{
{comment}       return null;
{comment}     }}
{comment} }}
{comment} List<double>? asDoubleArray() {{
{comment}     if (this.data != null) {{
{comment}       var length = this.data!.length;
{comment}       var ret = <double>[];
{comment}       const int DOUBLE_SIZE = 8;
{comment}       for (var i = 0; i < length/DOUBLE_SIZE; i++){{
{comment}         var number = ByteData.view(this.data!.buffer).getFloat64(i*DOUBLE_SIZE, Endian.little);
{comment}         ret.add(number);
{comment}       }}
{comment}       return ret;
{comment}     }} else {{
{comment}       return null;
{comment}     }}
{comment} }}
{comment} }}
{comment} 
{comment} void Function({PostedReply})? _RID_DEBUG_REPLY = ({PostedReply} reply) {{
{comment}   print('$reply');
{comment} }};
{comment}
{comment}
{comment} extension PostedReplyConfig on Rid {{
{comment}   void Function({PostedReply})? get debugReply => _RID_DEBUG_REPLY;
{comment}   void set debugReply(void Function({PostedReply})? val) => _RID_DEBUG_REPLY = val;
{comment} }}
{comment}
{comment} const int _TYPE_MASK= 0x000000000000ffff;
{comment} const int _I64_MIN = -9223372036854775808;
{comment} 
{comment} {class_name} decode(int packed, Uint8List? data) {{
{comment}   final ntype = packed & _TYPE_MASK;
{comment}   final id = (packed - _I64_MIN) >> 16;
{comment}   final reqId = id > 0 ? id : null;
{comment} 
{comment}   final type = {en}.values[ntype];
{comment}   return {class_name}._(type, reqId, data);
{comment} }}
{comment} 
{comment} final RidReplyChannelInternal<{class_name}> _replyChannel = RidReplyChannelInternal.instance(_dl, decode, _isDebugMode);
{comment}
{comment} extension ExposeRidReplyChannel on Rid {{
{comment}   RidReplyChannel<{PostedReply}> get replyChannel => {_RID_REPLY_CHANNEL};
{comment} }}
{comment} ```
    "###,
        PostedReply = posted_reply_type,
        _RID_REPLY_CHANNEL = _RID_REPLY_CHANNEL,
        comment = comment,
        en = dart_enum_name,
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
