import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart' as package_ffi;
import 'package:wasm/wasm.dart';
import 'package:wasm_example/generated/ffigen_binding.dart';
import 'package:wasm_example/generated/rid_api.dart';

const Utf8Codec utf8Codec = Utf8Codec();

String toDartString(ffi.Pointer<ffi.Int32> ptr, [int? len]) {
  final ffi.Pointer<package_ffi.Utf8> stringPtr = ptr.cast();
  return stringPtr.toDartString(length: len);
}

class WasmLibrary {
  final WasmInstance _wasmInstance;

  late final WasmFunction _create_store;
  late final WasmFunction _rid_store_count;
  late final WasmFunction _rid_store_free;
  late final WasmFunction _rid_cstring_free;
  late final WasmFunction _rid_msg_Dump;
  late final WasmFunction _rid_msg_Inc;
  late final WasmFunction _rid_rawstore_debug;
  late final WasmFunction _rid_rawstore_debug_pretty;
  late final WasmFunction _rid_store_lock;
  late final WasmFunction _rid_store_unlock;

  // Reply Polling
  late final WasmFunction _rid_rawreplystruct_debug;
  late final WasmFunction _rid_rawreplystruct_debug_pretty;
  late final WasmFunction _rid_replystruct_ty;
  late final WasmFunction _rid_replystruct_req_id;
  late final WasmFunction _rid_store_posted_replies;
  late final WasmFunction _rid_export_RawStore_poll_reply;

  late final WasmFunction _handled_reply;

  WasmLibrary(this._wasmInstance) {
    _create_store = _wasmInstance.lookupFunction('create_store');
    _rid_store_count = _wasmInstance.lookupFunction('rid_store_count');
    _rid_store_free = _wasmInstance.lookupFunction('rid_store_free');
    _rid_cstring_free = _wasmInstance.lookupFunction('rid_cstring_free');
    _rid_msg_Dump = _wasmInstance.lookupFunction("rid_msg_Dump");
    _rid_msg_Inc = _wasmInstance.lookupFunction("rid_msg_Inc");
    _rid_rawstore_debug = _wasmInstance.lookupFunction("rid_rawstore_debug");
    _rid_rawstore_debug_pretty =
        _wasmInstance.lookupFunction("rid_rawstore_debug_pretty");
    _rid_store_lock = _wasmInstance.lookupFunction("rid_store_lock");
    _rid_store_unlock = _wasmInstance.lookupFunction("rid_store_unlock");

    // Reply Polling
    _rid_rawreplystruct_debug =
        _wasmInstance.lookupFunction("rid_rawreplystruct_debug");
    _rid_rawreplystruct_debug_pretty =
        _wasmInstance.lookupFunction("rid_rawreplystruct_debug_pretty");
    _rid_replystruct_ty = _wasmInstance.lookupFunction('rid_replystruct_ty');
    _rid_replystruct_req_id =
        _wasmInstance.lookupFunction('rid_replystruct_req_id');

    _rid_store_posted_replies =
        _wasmInstance.lookupFunction("rid_store_posted_replies");
    _rid_export_RawStore_poll_reply =
        _wasmInstance.lookupFunction("rid_export_RawStore_poll_reply");
    _handled_reply = _wasmInstance.lookupFunction("handled_reply");
  }

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
  ffi.Pointer<RawStore> create_store() {
    final address = _create_store.apply([]);
    return ffi.Pointer<RawStore>.fromAddress(address);
  }

  int rid_store_count(ffi.Pointer<RawStore> ptr) {
    return _rid_store_count.apply([ptr.address]);
  }

  void rid_msg_Dump(int req_id) {
    _rid_msg_Dump.apply([req_id]);
  }

  void rid_msg_Inc(int req_id) {
    _rid_msg_Inc.apply([req_id]);
  }

  void rid_cstring_free(ffi.Pointer<ffi.Int8> ptr) {
    _rid_cstring_free.apply([ptr.address]);
  }

  int rid_rawstore_debug_pretty(ffi.Pointer<RawStore> store) {
    return _rid_rawstore_debug_pretty.apply([store.address]);
  }

  int rid_rawstore_debug(ffi.Pointer<RawStore> store) {
    return _rid_rawstore_debug.apply([store.address]);
  }

  void rid_store_lock() {
    return _rid_store_lock.apply([]);
  }

  void rid_store_unlock() {
    return _rid_store_unlock.apply([]);
  }

  void rid_store_free() {
    return _rid_store_free.apply([]);
  }

  // Not needed
  ffi.Pointer<Vec_RawReplyStruct> rid_store_posted_replies(
      ffi.Pointer<RawStore> store) {
    return _rid_store_posted_replies.apply([store]);
  }

  int rid_vec_ReplyStruct_len(ffi.Pointer<Vec_RawReplyStruct> _) {
    throw UnimplementedError("rid_vec_ReplyStruct_len");
  }

  ffi.Pointer<RawReplyStruct> rid_vec_ReplyStruct_get(
      ffi.Pointer<Vec_RawReplyStruct> _, int _idx) {
    throw UnimplementedError("rid_vec_ReplyStruct_get");
  }

  // -----------------
  // Reply Polling Wrappers
  // -----------------
  int rid_rawreplystruct_debug_pretty(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_rawreplystruct_debug_pretty.apply([replystruct.address]);
  }

  int rid_rawreplystruct_debug(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_rawreplystruct_debug.apply([replystruct.address]);
  }

  int rid_replystruct_ty(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_replystruct_ty.apply([replystruct.address]);
  }

  int rid_replystruct_req_id(ffi.Pointer<RawReplyStruct> replystruct) {
    return _rid_replystruct_req_id.apply([replystruct.address]);
  }

  ffi.Pointer<RawReplyStruct> rid_export_RawStore_poll_reply(
      ffi.Pointer<RawStore> store) {
    final int address = _rid_export_RawStore_poll_reply.apply([store.address]);
    return ffi.Pointer.fromAddress(address);
  }

  void handled_reply(int req_id) {
    return _handled_reply.apply([req_id]);
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

    final builder = module.builder();
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

// -----------------
// Extensions
// -----------------
extension Rid_ExtOnInt on int {
  String toDartString([int? len]) {
    return decodeUtf8String(WasmLibrary.instance.memView, this);
  }

  void free() {
    final ffi.Pointer<ffi.Int8> ptr = ffi.Pointer.fromAddress(this);
    rid_ffi.rid_cstring_free(ptr);
  }
}
