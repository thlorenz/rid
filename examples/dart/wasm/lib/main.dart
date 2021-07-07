import 'dart:async';

import 'package:wasm_example/generated/rid_api.dart';

import 'wasm/utils.dart';

const WASM_FILE = 'target/wasm32-unknown-unknown/debug/wasm_example.wasm';
Future<void> main() async {
  HTTP_HOST = 'localhost:8080';
  await initWasm(WASM_FILE);
  print(rid_ffi.toString());

  final store = Store.instance;
  print('store: ${store.debug(true)}');

  rid_ffi.rid_diagnose_filtered_todos(store.raw);

  print('store: ${store.debug(true)}');
}
