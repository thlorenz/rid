import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart' as package_ffi;
import 'package:wasm/wasm.dart';
import 'package:wasm_example/generated/rid_api.dart';

const Utf8Codec utf8Codec = Utf8Codec();

String toDartString(ffi.Pointer<ffi.Int32> ptr, [int? len]) {
  final ffi.Pointer<package_ffi.Utf8> stringPtr = ptr.cast();
  return stringPtr.toDartString(length: len);
}

class WasmLibrary {
  final WasmInstance _wasmInstance;
  final dynamic Function(String name) _lookup;

  WasmLibrary(this._wasmInstance) : _lookup = _wasmInstance.lookupFunction;

  WasmMemory get memory {
    return _wasmInstance.memory;
  }

  Uint8List get memView {
    return _wasmInstance.memory.view;
  }

  WasmFunction lookupFunction<T extends Function, F extends Function>(
      String name) {
    return _wasmInstance.lookupFunction(name) as WasmFunction;
  }

  WasmFunction lookup<T extends ffi.NativeType>(String name) {
    return _wasmInstance.lookupFunction(name) as WasmFunction;
  }

  // -----------------
  // Method Wrappers
  // -----------------

  // --- create_store ---
  ffi.Pointer<RawStore> create_store() {
    final address = _create_store.apply([]);
    return ffi.Pointer<RawStore>.fromAddress(address);
  }

  late final WasmFunction _create_store = _lookup('create_store');

  // --- rid_store_count ---
  int rid_store_count(ffi.Pointer<RawStore> ptr) {
    return _rid_store_count.apply([ptr.address]);
  }

  late final WasmFunction _rid_store_count = _lookup('rid_store_count');

  // --- rid_msg_Dump ---
  void rid_msg_Dump(int req_id) {
    _rid_msg_Dump.apply([req_id]);
  }

  late final WasmFunction _rid_msg_Dump = _lookup('rid_msg_Dump');

  // --- rid_msg_Inc ---
  void rid_msg_Inc(int req_id) {
    _rid_msg_Inc.apply([req_id]);
  }

  late final WasmFunction _rid_msg_Inc = _lookup('rid_msg_Inc');

  // --- rid_cstring_free ---
  void rid_cstring_free(ffi.Pointer<ffi.Int8> ptr) {
    _rid_cstring_free.apply([ptr.address]);
  }

  late final WasmFunction _rid_cstring_free = _lookup('rid_cstring_free');

  // --- rid_rawstore_debug_pretty ---
  ffi.Pointer<ffi.Int8> rid_rawstore_debug_pretty(ffi.Pointer<RawStore> store) {
    final address = _rid_rawstore_debug_pretty.apply([store.address]);
    return ffi.Pointer.fromAddress(address);
  }

  late final WasmFunction _rid_rawstore_debug_pretty =
      _lookup('rid_rawstore_debug_pretty');

  // --- rid_rawstore_debug ---
  ffi.Pointer<ffi.Int8> rid_rawstore_debug(ffi.Pointer<RawStore> store) {
    final address = _rid_rawstore_debug.apply([store.address]);
    return ffi.Pointer.fromAddress(address);
  }

  late final WasmFunction _rid_rawstore_debug = _lookup('rid_rawstore_debug');

  // --- rid_store_lock ---
  void rid_store_lock() {
    return _rid_store_lock.apply([]);
  }

  late final WasmFunction _rid_store_lock = _lookup('rid_store_lock');

  // --- rid_store_unlock ---
  void rid_store_unlock() {
    return _rid_store_unlock.apply([]);
  }

  late final WasmFunction _rid_store_unlock = _lookup('rid_store_unlock');

  // --- rid_store_free ---
  void rid_store_free() {
    return _rid_store_free.apply([]);
  }

  late final WasmFunction _rid_store_free = _lookup('rid_store_free');

  // -----------------
  // Reply Polling Wrappers
  // -----------------
  // --- rid_rawreplystruct_debug_pretty ---
  ffi.Pointer<ffi.Int8> rid_rawreplystruct_debug_pretty(
      ffi.Pointer<RawReplyStruct> replystruct) {
    final address =
        _rid_rawreplystruct_debug_pretty.apply([replystruct.address]);
    return ffi.Pointer.fromAddress(address);
  }

  late final WasmFunction _rid_rawreplystruct_debug_pretty =
      _lookup('rid_rawreplystruct_debug_pretty');

  // --- rid_rawreplystruct_debug ---
  ffi.Pointer<ffi.Int8> rid_rawreplystruct_debug(
      ffi.Pointer<RawReplyStruct> replystruct) {
    final address = _rid_rawreplystruct_debug.apply([replystruct.address]);
    return ffi.Pointer.fromAddress(address);
  }

  late final WasmFunction _rid_rawreplystruct_debug =
      _lookup('rid_rawreplystruct_debug');

  // --- rid_replystruct_ty ---
  int rid_replystruct_ty(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_replystruct_ty.apply([replystruct.address]);
  }

  late final WasmFunction _rid_replystruct_ty = _lookup('rid_replystruct_ty');

  // --- rid_replystruct_req_id ---
  int rid_replystruct_req_id(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_replystruct_req_id.apply([replystruct.address]);
  }

  late final WasmFunction _rid_replystruct_req_id =
      _lookup('rid_replystruct_req_id');

  // --- rid_export_RawStore_poll_reply ---
  ffi.Pointer<RawReplyStruct> rid_export_RawStore_poll_reply(
      ffi.Pointer<RawStore> store) {
    final int address = _rid_export_RawStore_poll_reply.apply([store.address]);
    return ffi.Pointer.fromAddress(address);
  }

  late final WasmFunction _rid_export_RawStore_poll_reply =
      _lookup('rid_export_RawStore_poll_reply');

  // --- handled_reply ---
  void rid_handled_reply(int req_id) {
    return _rid_handled_reply.apply([req_id]);
  }

  late final WasmFunction _rid_handled_reply = _lookup('rid_handled_reply');

  // --- poll_reply ---
  ReplyStruct? rid_poll_reply() {
    final address = _rid_poll_reply.apply([]);
    if (address == 0x0) return null;
    final ffi.Pointer<RawReplyStruct> ptr = ffi.Pointer.fromAddress(address);
    return ptr.toDart();
  }

  late final WasmFunction _rid_poll_reply = _lookup('rid_poll_reply');

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

  static WasmLibrary init(String pathToWasm) {
    final file = File(pathToWasm);
    final moduleData = file.readAsBytesSync();
    final WasmModule module = WasmModule(moduleData);
    print(module.describe());

    final builder = module.builder()
      ..enableWasi(captureStdout: false, captureStderr: false);
    final instance = builder.build();

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
