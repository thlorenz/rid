@TestOn('js')
import 'package:test/test.dart';
import 'package:wasm_example/wasm/utils.dart';
import 'package:wasm_example/wasm_binding.dart';

int reqId = 1;

void main() {
  ROOT_URL = 'http://localhost:8080';
  group('Store Interaction', () {
    test('init store and interacting with it works', () async {
      final store = await Store.instance;
      expect(store.count, 0);

      {
        final reply = await store.msgInc();
        expect(reply.type, Reply.Inced);
        expect(store.count, 1);
      }

      {
        final reply = await store.msgAdd(9);
        expect(reply.type, Reply.Added);
        expect(store.count, 10);
      }

      {
        final reply = await store.msgAdd(100);
        expect(reply.type, Reply.Added);
        expect(store.count, 110);

        expect(store.debug(), 'Store { count: 110 }');
      }
    });
  });
}
