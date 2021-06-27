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

  late final dynamic _create_store;
  late final dynamic _rid_store_count;
  late final dynamic _rid_store_free;
  late final dynamic _rid_cstring_free;
  late final dynamic _rid_msg_Dump;
  late final dynamic _rid_msg_Inc;
  late final dynamic _rid_rawstore_debug;
  late final dynamic _rid_rawstore_debug_pretty;
  late final dynamic _rid_store_lock;
  late final dynamic _rid_store_unlock;

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
  }

  WasmMemory get memory {
    return _wasmInstance.memory;
  }

  Uint8List get memView {
    return _wasmInstance.memory.view;
  }

  F lookupFunction<T extends Function, F extends Function>(String name) {
    return _wasmInstance.lookupFunction(name) as F;
  }

  Pointer<T> lookup<T extends ffi.NativeType>(String name) {
    return _wasmInstance.lookupFunction(name) as Pointer<T>;
  }

  // -----------------
  // Method Wrappers
  // -----------------
  ffi.Pointer<RawStore> create_store() {
    final address = _create_store();
    return ffi.Pointer<RawStore>.fromAddress(address);
  }

  int rid_store_count(ffi.Pointer<RawStore> ptr) {
    return _rid_store_count(ptr.address);
  }

  void rid_msg_Dump(int req_id) {
    _rid_msg_Dump(req_id);
  }

  void rid_msg_Inc(int req_id) {
    _rid_msg_Inc(req_id);
  }

  void rid_cstring_free(ffi.Pointer<ffi.Int8> ptr) {
    _rid_cstring_free(ptr.address);
  }

  ffi.Int8 rid_rawstore_debug_pretty(ffi.Pointer<RawStore> store) {
    return _rid_rawstore_debug_pretty(store.address);
  }

  ffi.Int8 rid_rawstore_debug(ffi.Pointer<RawStore> store) {
    return _rid_rawstore_debug(store.address);
  }

  void rid_store_lock() {
    return _rid_store_lock();
  }

  void rid_store_unlock() {
    return _rid_store_unlock();
  }

  void rid_store_free() {
    return _rid_store_free();
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

    final builder = module.builder();
    final instance = builder.build();

    return WasmLibrary(instance);
  }
}

// -----------------
// Util Methods
// -----------------
String decodeUtf8String(Uint8List codeUnits, ffi.Int8 address) {
  final end = _end(codeUnits, address as int);
  return utf8Codec.decode(codeUnits.sublist(address as int, end));
}

int _end(Uint8List codeUnits, int start) {
  int end = start;
  while (codeUnits[end] != 0) end++;
  return end;
}

// -----------------
// Extensions
// -----------------
extension Rid_ExtOnInt8 on ffi.Int8 {
  String toDartString([int? len]) {
    return decodeUtf8String(WasmLibrary.instance.memView, this);
  }

  void free() {
    final ffi.Pointer<ffi.Int8> ptr = ffi.Pointer.fromAddress(this as int);
    rid_ffi.rid_cstring_free(ptr);
  }
}
