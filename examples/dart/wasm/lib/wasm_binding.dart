import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:wasm_interop/wasm_interop.dart';

const Utf8Codec utf8Codec = Utf8Codec();

abstract class Opaque {
  final int _address;

  Opaque(this._address);

  int get address => _address;
}

// For the native API we could type alias RawReplyStructPointer to Pointer<RawReplyStruct>
class RawReplyStruct extends Opaque {
  RawReplyStruct(int address) : super(address);
}

class Pointer<T extends Opaque> {
  final T _opaque;

  Pointer._(this._opaque);
  factory Pointer.fromAddress(T opaque) {
    return Pointer._(opaque);
  }

  int get address => _opaque.address;
}

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
    WasmLibrary.instance.rid_store_lock();
    final instance = ReplyStruct._(this.ty, this.req_id);
    WasmLibrary.instance.rid_store_unlock();
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
  int create_store() {
    return _create_store();
  }

  late final int Function() _create_store = _lookup('create_store');

  // --- rid_store_count ---
  int rid_store_count(int storeAddr) {
    return _rid_store_count(storeAddr);
  }

  late final int Function(int) _rid_store_count = _lookup('rid_store_count');

  // --- rid_msg_Dump ---
  void rid_msg_Dump(int req_id) {
    _rid_msg_Dump(req_id);
  }

  late final void Function(int) _rid_msg_Dump = _lookup('rid_msg_Dump');

  // --- rid_msg_Inc ---
  void rid_msg_Inc(int req_id) {
    _rid_msg_Inc(req_id.toString());
  }

  late final void Function(/* BigInt */ String) _rid_msg_Inc =
      _lookup('rid_msg_Inc');

  // --- rid_cstring_free ---
  void rid_cstring_free(int strAddr) {
    _rid_cstring_free(strAddr);
  }

  late final void Function(int) _rid_cstring_free = _lookup('rid_cstring_free');

  // --- rid_rawstore_debug_pretty ---
  int rid_rawstore_debug_pretty(int storeAddr) {
    return _rid_rawstore_debug_pretty(storeAddr);
  }

  late final int Function(int) _rid_rawstore_debug_pretty =
      _lookup('rid_rawstore_debug_pretty');

  String decodeUtf8String(int address) {
    return decodeUtf8ListString(memView, address);
  }

  // --- rid_rawstore_debug ---
  int rid_rawstore_debug(int storeAddr) {
    return _rid_rawstore_debug(storeAddr);
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
    return _rid_replystruct_req_id(ptr.address);
  }

  late final int Function(int) _rid_replystruct_req_id =
      _lookup('rid_replystruct_req_id');

  // --- handled_reply ---
  void rid_handled_reply(int req_id) {
    return _rid_handled_reply(req_id);
  }

  late final void Function(int) _rid_handled_reply =
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
    return WasmLibrary.instance;
  }

  static Future<WasmLibrary> initFromFile(String pathToWasm) async {
    final file = File(pathToWasm);
    return init(file.readAsBytesSync());
  }
}

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
