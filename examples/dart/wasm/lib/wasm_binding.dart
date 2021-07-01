import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:wasm_example/wasm/utils.dart';
import 'package:wasm_example/wasm_reply_channel.dart';
import 'package:wasm_interop/wasm_interop.dart';

typedef JSBigInt = String;
const Utf8Codec utf8Codec = Utf8Codec();

// -----------------
// FFI Shims
// -----------------
abstract class Opaque {
  final int _address;

  Opaque(this._address);

  int get address => _address;
}

class RawReplyStruct extends Opaque {
  RawReplyStruct(int address) : super(address);
}

class RawStore extends Opaque {
  RawStore(int address) : super(address);
}

class Pointer<T extends Opaque> {
  final T _opaque;

  Pointer._(this._opaque);
  factory Pointer.fromAddress(T opaque) {
    return Pointer._(opaque);
  }

  int get address => _opaque.address;
}

// -----------------
// Reply Struct
// -----------------
class ReplyStruct {
  final int ty;
  final int reqId;
  const ReplyStruct._(this.ty, this.reqId);
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        other is ReplyStruct && ty == other.ty && reqId == other.reqId;
  }

  @override
  int get hashCode {
    return ty.hashCode ^ reqId.hashCode;
  }

  @override
  String toString() {
    return 'ReplyStruct{ty: $ty, reqId: $reqId}';
  }
}

extension Rid_ToDart_ExtOnReplyStruct on Pointer<RawReplyStruct> {
  ReplyStruct toDart() {
    ridStoreLock();
    final instance = ReplyStruct._(this.ty, this.req_id);
    ridStoreUnlock();
    return instance;
  }
}

extension Rid_Model_ExtOnPointerRawReplyStruct on Pointer<RawReplyStruct> {
  int get ty => WasmLibrary.instance.rid_replystruct_ty(this);
  int get req_id => WasmLibrary.instance.rid_replystruct_req_id(this);
}

extension rid_rawreplystruct_debug_ExtOnReplyStruct on Pointer<RawReplyStruct> {
  String debug([bool pretty = false]) {
    final ptr = pretty
        ? WasmLibrary.instance.rid_rawreplystruct_debug_pretty(this)
        : WasmLibrary.instance.rid_rawreplystruct_debug(this);
    final s = WasmLibrary.instance.decodeUtf8String(ptr);
    WasmLibrary.instance.rid_cstring_free(ptr);
    return s;
  }
}

// -----------------
// Posted Reply
// -----------------
void Function(PostedReply)? RID_DEBUG_REPLY = (PostedReply reply) {
  print('$reply');
};
final Duration? RID_MSG_TIMEOUT = const Duration(milliseconds: 100);

Future<PostedReply> _replyWithTimeout(
  Future<PostedReply> reply,
  String msgCall,
  StackTrace applicationStack,
  Duration timeout,
) {
  final failureMsg = '''$msgCall timed out\n
  ---- Application Stack ----\n
  $applicationStack\n
  ---- Internal Stack ----
  ''';
  return reply.timeout(timeout,
      onTimeout: () => throw TimeoutException(failureMsg, timeout));
}

enum Reply { Inced, Added }

class PostedReply extends IReply {
  final Reply type;
  final int? reqId;
  final String? data;
  PostedReply._(this.type, this.reqId, this.data);
  @override
  String toString() {
    return '''PostedReply {
    type:  ${this.type.toString().substring('Reply.'.length)}
    reqId: $reqId
    data:  $data
  }
  ''';
  }
}

PostedReply wasmDecode(ReplyStruct reply) {
  return PostedReply._(Reply.values[reply.ty], reply.reqId, null);
}

final _isDebugMode = true;
late final ReplyChannel<PostedReply> replyChannel;

// -----------------
// RawStore
// -----------------
extension rid_rawstore_debug_ExtOnStore on Pointer<RawStore> {
  String debug([bool pretty = false]) {
    final ptr = pretty
        ? WasmLibrary.instance.rid_rawstore_debug_pretty(this)
        : WasmLibrary.instance.rid_rawstore_debug(this);
    final s = WasmLibrary.instance.decodeUtf8String(ptr);
    WasmLibrary.instance.rid_cstring_free(ptr);
    return s;
  }
}

extension rid_store_specific_extension on Pointer<RawStore> {
  T runLocked<T>(T Function(Pointer<RawStore>) fn, {String? request}) {
    try {
      ridStoreLock(request: request);
      return fn(this);
    } finally {
      ridStoreUnlock();
    }
  }

  Future<void> dispose() {
    return replyChannel.dispose();
  }
}

extension Rid_Model_ExtOnPointerRawStore on Pointer<RawStore> {
  int get count => WasmLibrary.instance.rid_store_count(this);
}

extension FieldAccessWrappersOn_Store on Store {
  int get count => _read((store) => store.count, 'store.count');
}

extension Rid_Message_ExtOnPointerStoreForMsg on Pointer<RawStore> {
  Future<PostedReply> msgInc({Duration? timeout}) {
    final reqId = replyChannel.reqId;
    rid_ffi.rid_msg_Inc(reqId);
    final reply = _isDebugMode && RID_DEBUG_REPLY != null
        ? replyChannel.reply(reqId).then((PostedReply reply) {
            if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
            return reply;
          })
        : replyChannel.reply(reqId);
    if (!_isDebugMode) return reply;
    timeout ??= RID_MSG_TIMEOUT;
    if (timeout == null) return reply;

    final msgCall = 'msgInc() with reqId: $reqId';
    return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
  }

  Future<PostedReply> msgAdd(int n, {Duration? timeout}) {
    final reqId = replyChannel.reqId;
    rid_ffi.rid_msg_Add(reqId, n);
    final reply = _isDebugMode && RID_DEBUG_REPLY != null
        ? replyChannel.reply(reqId).then((PostedReply reply) {
            if (RID_DEBUG_REPLY != null) RID_DEBUG_REPLY!(reply);
            return reply;
          })
        : replyChannel.reply(reqId);
    if (!_isDebugMode) return reply;
    timeout ??= RID_MSG_TIMEOUT;
    if (timeout == null) return reply;
    final msgCall = 'msgAdd() with reqId: $reqId';
    return _replyWithTimeout(reply, msgCall, StackTrace.current, timeout);
  }
}

// -----------------
// Store
// -----------------

// TODO: this will be determined at build time and appended to the file
const WASM_FILE = 'target/wasm32-unknown-unknown/debug/wasm_example.wasm';

class Store {
  final Pointer<RawStore> _store;
  Pointer<RawStore> get raw => _store;

  const Store(this._store);
  T _read<T>(T Function(Pointer<RawStore> store) accessor, String? request) {
    return _store.runLocked(accessor, request: request);
  }

  // StoreState toDartState() => _store.toDart();
  String debug([bool pretty = false]) => _store.debug(pretty);

  // Future<void> dispose() => _store.dispose();
  static Store? _instance;
  static Future<Store> get instance async {
    if (_instance == null) {
      final moduleData = await loadWasmFile(WASM_FILE);
      final wasmLib = await WasmLibrary.init(moduleData);
      _instance = Store(wasmLib.create_store());
    }
    return _instance!;
  }
}

extension MsgApiFor_Store on Store {
  Future<PostedReply> msgInc({Duration? timeout}) {
    return _store.msgInc(timeout: timeout);
  }

  Future<PostedReply> msgAdd(int n, {Duration? timeout}) {
    return _store.msgAdd(n, timeout: timeout);
  }
}

// -----------------
// Wasm lib
// -----------------
class WasmLibrary {
  final Instance _wasmInstance;

  WasmLibrary(this._wasmInstance);

  T _lookup<T>(String name) {
    return _wasmInstance.functions[name]! as T;
  }

  Memory get memory {
    return _wasmInstance.memories['memory']!;
  }

  Uint8List get memView {
    return _wasmInstance.memories['memory']!.buffer.asUint8List();
  }

  // -----------------
  // Method Wrappers
  // -----------------

  // --- create_store ---
  Pointer<RawStore> create_store() {
    final addr = _create_store();
    return Pointer.fromAddress(RawStore(addr));
  }

  late final int Function() _create_store = _lookup('create_store');

  // --- rid_store_count ---
  int rid_store_count(Pointer<RawStore> ptr) {
    return _rid_store_count(ptr.address);
  }

  late final int Function(int) _rid_store_count = _lookup('rid_store_count');

  // --- rid_msg_Add ---
  void rid_msg_Add(int req_id, int n) {
    _rid_msg_Add(req_id.toString(), n);
  }

  late final void Function(JSBigInt, int) _rid_msg_Add = _lookup('rid_msg_Add');

  // --- rid_msg_Inc ---
  void rid_msg_Inc(int req_id) {
    _rid_msg_Inc(req_id.toString());
  }

  late final void Function(JSBigInt) _rid_msg_Inc = _lookup('rid_msg_Inc');

  // --- rid_cstring_free ---
  void rid_cstring_free(int strAddr) {
    _rid_cstring_free(strAddr);
  }

  late final void Function(int) _rid_cstring_free = _lookup('rid_cstring_free');

  // --- rid_rawstore_debug_pretty ---
  int rid_rawstore_debug_pretty(Pointer<RawStore> ptr) {
    return _rid_rawstore_debug_pretty(ptr.address);
  }

  late final int Function(int) _rid_rawstore_debug_pretty =
      _lookup('rid_rawstore_debug_pretty');

  String decodeUtf8String(int address) {
    return decodeUtf8ListString(memView, address);
  }

  // --- rid_rawstore_debug ---
  int rid_rawstore_debug(Pointer<RawStore> ptr) {
    return _rid_rawstore_debug(ptr.address);
  }

  late final int Function(int) _rid_rawstore_debug =
      _lookup('rid_rawstore_debug');

  // --- rid_store_lock ---
  void rid_store_lock() {
    return _rid_store_lock();
  }

  late final void Function() _rid_store_lock = _lookup('rid_store_lock');

  // --- rid_store_unlock ---
  void rid_store_unlock() {
    return _rid_store_unlock();
  }

  late final void Function() _rid_store_unlock = _lookup('rid_store_unlock');

  // --- rid_store_free ---
  void rid_store_free() {
    return _rid_store_free();
  }

  late final void Function() _rid_store_free = _lookup('rid_store_free');

  // -----------------
  // Reply Polling Wrappers
  // -----------------
  // --- rid_rawreplystruct_debug_pretty ---
  int rid_rawreplystruct_debug_pretty(Pointer<RawReplyStruct> ptr) {
    return _rid_rawreplystruct_debug_pretty(ptr.address);
  }

  late final int Function(int) _rid_rawreplystruct_debug_pretty =
      _lookup('rid_rawreplystruct_debug_pretty');

  int rid_rawreplystruct_debug(Pointer<RawReplyStruct> ptr) {
    return _rid_rawreplystruct_debug(ptr.address);
  }

  late final int Function(int) _rid_rawreplystruct_debug =
      _lookup('rid_rawreplystruct_debug');

  // --- rid_replystruct_ty ---
  int rid_replystruct_ty(Pointer<RawReplyStruct> ptr) {
    return _rid_replystruct_ty(ptr.address);
  }

  late final int Function(int) _rid_replystruct_ty =
      _lookup('rid_replystruct_ty');

  // --- rid_replystruct_req_id ---
  int rid_replystruct_req_id(Pointer<RawReplyStruct> ptr) {
    final req_id = _rid_replystruct_req_id(ptr.address);
    return jsBigIntToInt(req_id);
  }

  late final JSBigInt Function(int) _rid_replystruct_req_id =
      _lookup('rid_replystruct_req_id');

  // --- handled_reply ---
  void rid_handled_reply(int req_id) {
    return _rid_handled_reply(req_id.toString());
  }

  late final void Function(JSBigInt) _rid_handled_reply =
      _lookup('rid_handled_reply');

  // --- poll_reply ---
  Pointer<RawReplyStruct>? rid_poll_reply() {
    final address = _rid_poll_reply();
    if (address == 0x0) return null;
    return Pointer.fromAddress(RawReplyStruct(address));
  }

  late final int Function() _rid_poll_reply = _lookup('rid_poll_reply');

  /*
  // -----------------
  // Extension Method Delegates
  // -----------------
  String ptrInt8ToDartString(ffi.Pointer<ffi.Int8> ptr, [int? len]) {
    return decodeUtf8String(WasmLibrary.instance.memView, ptr.address);
  }

  void ptrInt8Free(ffi.Pointer<ffi.Int8> ptr) {
    rid_cstring_free(ptr);
  }
  */

  // -----------------
  // Initializers
  // -----------------
  static WasmLibrary? _instance;
  static WasmLibrary get instance {
    assert(_instance != null,
        "need to WasmLibrary.init() before accessing instance");
    return _instance!;
  }

  static Future<WasmLibrary> init(Uint8List moduleData) async {
    final Instance instance = await Instance.fromBytesAsync(moduleData);
    // print(module.describe());

    _instance = WasmLibrary(instance);

    replyChannel =
        ReplyChannel.instance(WasmLibrary.instance, wasmDecode, _isDebugMode);
    return WasmLibrary.instance;
  }

  static Future<WasmLibrary> initFromFile(String pathToWasm) async {
    final file = File(pathToWasm);
    return init(file.readAsBytesSync());
  }
}

WasmLibrary get rid_ffi => WasmLibrary.instance;

// -----------------
// Util Methods
// -----------------
String decodeUtf8ListString(Uint8List codeUnits, int address) {
  final end = _end(codeUnits, address);
  return utf8Codec.decode(codeUnits.sublist(address, end));
}

int _end(Uint8List codeUnits, int start) {
  int end = start;
  while (codeUnits[end] != 0) end++;
  return end;
}

// Only reliable way I found to convert JS BigInt to int
// Dart int is 64bit, but signed so a u64 may not exactly fit, but
// close enough for now.
int jsBigIntToInt(JSBigInt n) => int.parse(n.toString());

// -----------------
// Store lock
// -----------------
int _locks = 0;
void Function(bool, int, {String? request})? RID_DEBUG_LOCK =
    (bool locking, int locks, {String? request}) {
  if (locking) {
    if (locks == 1) print('🔐 {');
    if (request != null) print(' $request');
  } else {
    if (locks == 0) print('} 🔓');
  }
};
void ridStoreLock({String? request}) {
  if (_locks == 0) WasmLibrary.instance.rid_store_lock();
  _locks++;
  if (RID_DEBUG_LOCK != null) RID_DEBUG_LOCK!(true, _locks, request: request);
}

void ridStoreUnlock() {
  _locks--;
  if (RID_DEBUG_LOCK != null) RID_DEBUG_LOCK!(false, _locks);
  if (_locks == 0) WasmLibrary.instance.rid_store_unlock();
}
