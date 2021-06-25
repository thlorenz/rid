import 'dart:async';
import 'dart:io';
import 'dart:typed_data';

import 'package:wasm/wasm.dart';

// import 'package:clock/generated/rid_api.dart';

const WIP_WASM = 'target/wasm32-unknown-unknown/debug/wasm_example.wasm';
const WIP_WASM_WASI = 'target/wasm32-wasi/debug/wasm_example.wasm';

void initWasm() {
  final wipPath = WIP_WASM;
  ;
  print('\nLoading wasm module from $wipPath');

  final file = File(wipPath);
  final moduleData = file.readAsBytesSync();
  final WasmModule module = WasmModule(moduleData);
  print(module.describe());
}

Future<void> main() async {
  initWasm();
}
