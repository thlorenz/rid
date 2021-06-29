import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:typed_data';

import 'package:wasm_interop/wasm_interop.dart';
import 'package:ffi/ffi.dart' as package_ffi;
import 'package:wasm_example/generated/rid_api.dart';

const Utf8Codec utf8Codec = Utf8Codec();

String toDartString(ffi.Pointer<ffi.Int32> ptr, [int? len]) {
  final ffi.Pointer<package_ffi.Utf8> stringPtr = ptr.cast();
  return stringPtr.toDartString(length: len);
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
  ffi.Pointer<RawStore> create_store() {
    final address = _create_store();
    return ffi.Pointer<RawStore>.fromAddress(address);
  }

  late final int Function() _create_store = _lookup('create_store');

  // --- rid_store_count ---
  int rid_store_count(ffi.Pointer<RawStore> ptr) {
    return _rid_store_count(ptr.address);
  }

  late final int Function(int) _rid_store_count = _lookup('rid_store_count');

  // --- rid_msg_Dump ---
  void rid_msg_Dump(int req_id) {
    _rid_msg_Dump(req_id);
  }

  late final void Function(int) _rid_msg_Dump = _lookup('rid_msg_Dump');

  // --- rid_msg_Inc ---
  void rid_msg_Inc(int req_id) {
    _rid_msg_Inc(req_id);
  }

  late final void Function(int) _rid_msg_Inc = _lookup('rid_msg_Inc');

  // --- rid_cstring_free ---
  void rid_cstring_free(ffi.Pointer<ffi.Int8> ptr) {
    _rid_cstring_free(ptr.address);
  }

  late final void Function(int) _rid_cstring_free = _lookup('rid_cstring_free');

  // --- rid_rawstore_debug_pretty ---
  ffi.Pointer<ffi.Int8> rid_rawstore_debug_pretty(ffi.Pointer<RawStore> store) {
    final address = _rid_rawstore_debug_pretty(store.address);
    return ffi.Pointer.fromAddress(address);
  }

  late final int Function(int) _rid_rawstore_debug_pretty =
      _lookup('rid_rawstore_debug_pretty');

  // --- rid_rawstore_debug ---
  ffi.Pointer<ffi.Int8> rid_rawstore_debug(ffi.Pointer<RawStore> store) {
    final address = _rid_rawstore_debug(store.address);
    return ffi.Pointer.fromAddress(address);
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
  ffi.Pointer<ffi.Int8> rid_rawreplystruct_debug_pretty(
      ffi.Pointer<RawReplyStruct> replystruct) {
    final address = _rid_rawreplystruct_debug_pretty(replystruct.address);
    return ffi.Pointer.fromAddress(address);
  }

  late final int Function(int) _rid_rawreplystruct_debug_pretty =
      _lookup('rid_rawreplystruct_debug_pretty');

  // --- rid_rawreplystruct_debug ---
  ffi.Pointer<ffi.Int8> rid_rawreplystruct_debug(
      ffi.Pointer<RawReplyStruct> replystruct) {
    final address = _rid_rawreplystruct_debug(replystruct.address);
    return ffi.Pointer.fromAddress(address);
  }

  late final int Function(int) _rid_rawreplystruct_debug =
      _lookup('rid_rawreplystruct_debug');

  // --- rid_replystruct_ty ---
  int rid_replystruct_ty(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_replystruct_ty(replystruct.address);
  }

  late final int Function(int) _rid_replystruct_ty =
      _lookup('rid_replystruct_ty');

  // --- rid_replystruct_req_id ---
  int rid_replystruct_req_id(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_replystruct_req_id(replystruct.address);
  }

  late final int Function(int) _rid_replystruct_req_id =
      _lookup('rid_replystruct_req_id');

  // --- rid_export_RawStore_poll_reply ---
  ffi.Pointer<RawReplyStruct> rid_export_RawStore_poll_reply(
      ffi.Pointer<RawStore> store) {
    final int address = _rid_export_RawStore_poll_reply(store.address);
    return ffi.Pointer.fromAddress(address);
  }

  late final int Function(int) _rid_export_RawStore_poll_reply =
      _lookup('rid_export_RawStore_poll_reply');

  // --- handled_reply ---
  void rid_handled_reply(int req_id) {
    return _rid_handled_reply(req_id);
  }

  late final void Function(int) _rid_handled_reply =
      _lookup('rid_handled_reply');

  // --- poll_reply ---
  ReplyStruct? rid_poll_reply() {
    final address = _rid_poll_reply();
    if (address == 0x0) return null;
    final ffi.Pointer<RawReplyStruct> ptr = ffi.Pointer.fromAddress(address);
    return ptr.toDart();
  }

  late final int Function() _rid_poll_reply = _lookup('rid_poll_reply');

  // -----------------
  // Extension Method Delegates
  // -----------------
  String ptrInt8ToDartString(ffi.Pointer<ffi.Int8> ptr, [int? len]) {
    return decodeUtf8String(WasmLibrary.instance.memView, ptr.address);
  }

  void ptrInt8Free(ffi.Pointer<ffi.Int8> ptr) {
    rid_cstring_free(ptr);
  }

  // -----------------
  // Initializers
  // -----------------
  static WasmLibrary? _instance;
  static WasmLibrary get instance {
    assert(_instance != null,
        "need to WasmLibrary.init() before accessing instance");
    return _instance!;
  }

  static Future<WasmLibrary> init(String pathToWasm) async {
    final file = File(pathToWasm);
    final moduleData = file.readAsBytesSync();
    final Instance instance = await Instance.fromBytesAsync(moduleData);
    // print(module.describe());

    _instance = WasmLibrary(instance);
    return WasmLibrary.instance;
  }
}

// -----------------
// Util Methods
// -----------------
String decodeUtf8String(Uint8List codeUnits, int address) {
  final end = _end(codeUnits, address);
  return utf8Codec.decode(codeUnits.sublist(address, end));
}

int _end(Uint8List codeUnits, int start) {
  int end = start;
  while (codeUnits[end] != 0) end++;
  return end;
}
