import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as package_ffi;

import 'package:wasm/wasm.dart';
import 'package:wasm_example/generated/ffigen_binding.dart';
import 'package:wasm_example/generated/rid_api.dart';

// import 'package:clock/generated/rid_api.dart';

const WIP_WASM = 'target/wasm32-unknown-unknown/debug/wasm_example.wasm';
const WIP_WASM_WASI = 'target/wasm32-wasi/debug/wasm_example.wasm';

const Utf8Codec utf8Codec = Utf8Codec();

String toDartString(ffi.Pointer<ffi.Int32> ptr, [int? len]) {
  final ffi.Pointer<package_ffi.Utf8> stringPtr = ptr.cast();
  return stringPtr.toDartString(length: len);
}

class WasmWrapper {
  final WasmInstance instance;

  late final dynamic _create_store;
  late final dynamic _rid_store_count;
  late final dynamic _rid_rawstore_debug;
  late final dynamic _rid_rawstore_debug_pretty;

  WasmWrapper(this.instance) {
    _create_store = instance.lookupFunction('create_store');
    _rid_store_count = instance.lookupFunction('rid_store_count');
    _rid_rawstore_debug = instance.lookupFunction('rid_rawstore_debug');
    _rid_rawstore_debug_pretty =
        instance.lookupFunction('rid_rawstore_debug_pretty');
  }

  ffi.Pointer<RawStore> create_store() {
    final address = _create_store();
    return ffi.Pointer<RawStore>.fromAddress(address);
  }

  int rid_store_count(ffi.Pointer<RawStore> ptr) {
    return _rid_store_count(ptr.address);
  }

  String rid_rawstore_debug(ffi.Pointer<RawStore> ptr, [bool pretty = false]) {
    final int address = pretty
        ? _rid_rawstore_debug_pretty(ptr.address)
        : _rid_rawstore_debug(ptr.address);
    final strPtr = ffi.Pointer<ffi.Int8>.fromAddress(address);

    // -----------------
    // Decode Utf8String
    // -----------------
    final codeUnits = instance.memory.view;
    final end = _end(codeUnits, strPtr.address);
    return utf8Codec.decode(codeUnits.sublist(strPtr.address, end));
  }

  static int _end(Uint8List codeUnits, int start) {
    int end = start;
    while (codeUnits[end] != 0) end++;
    return end;
  }
}

void initWasm() {
  final wipPath = WIP_WASM;
  ;
  print('\nLoading wasm module from $wipPath');

  final file = File(wipPath);
  final moduleData = file.readAsBytesSync();
  final WasmModule module = WasmModule(moduleData);
  print(module.describe());

  final builder = module.builder();
  final instance = builder.build();

  final wrapper = WasmWrapper(instance);
  final store = wrapper.create_store();
  print('store: $store');

  print(wrapper.rid_store_count(store));
  print(wrapper.rid_rawstore_debug(store, true));
}

Future<void> main() async {
  initWasm();
}
