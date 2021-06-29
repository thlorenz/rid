import 'dart:async';
import 'package:wasm_example/generated/rid_api.dart';

Future<void> main() async {
  await initRid();
  final store = Store.instance;
  print('store: ${store.debug(true)}');

  await store.msgInc();
  print('store: ${store.debug(true)}');
  await store.msgInc();
  print('store: ${store.debug(true)}');
  await store.msgInc();
  print('store: ${store.debug(true)}');
  await store.msgInc();
  print('store: ${store.debug(true)}');

  store.dispose();
}
