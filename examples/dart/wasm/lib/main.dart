import 'dart:async';
import 'package:wasm_example/generated/rid_api.dart';

Future<void> main() async {
  final store = Store.instance;
  print('store: ${store.debug(true)}');

  final reply = await store.msgInc();
  print('reply: $reply');
  print('store: ${store.debug(true)}');
  final storeState = store.toDartState();
  print('$storeState');
  store.dispose();
}
