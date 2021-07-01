import 'dart:async';

import 'package:wasm_example/wasm_binding.dart';

Future<void> main() async {
  final store = await Store.instance;
  print('create_store: ${store.debug(true)}');

  await store.msgInc();
  print('store count: ${store.count}');

  await store.msgInc();

  print('store: ${store.debug(true)}');

  await store.msgAdd(10);
  print('store: ${store.debug(true)}');
  await store.msgAdd(100);
  print('store: ${store.debug(true)}');
}
