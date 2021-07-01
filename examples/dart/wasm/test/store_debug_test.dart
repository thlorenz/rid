@TestOn('js')
import 'package:test/test.dart';
import 'package:wasm_example/wasm_binding.dart';

int reqId = 1;

void main() {
  group('Store Interaction', () {
    test('init store and interacting with it works', () async {
      final store = await Store.instance;
      print('store count: ${store.count}');

      PostedReply reply = await store.msgInc();
      print('reply: $reply');
      print('store count: ${store.count}');

      reply = await store.msgInc();
      print('reply: $reply');

      print('store: ${store.debug(true)}');
    });
  });
}
