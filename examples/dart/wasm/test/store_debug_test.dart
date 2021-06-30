@TestOn('js')
import 'package:test/test.dart';
import 'package:wasm_example/wasm/utils.dart';
import 'package:wasm_example/wasm_binding.dart';

int reqId = 1;

void main() {
  group('Store Interaction', () {
    test('init store and interacting with it works', () async {
      const WASM_FILE = 'target/wasm32-unknown-unknown/debug/wasm_example.wasm';
      final moduleData = await loadWasmFile(WASM_FILE);
      final lib = await WasmLibrary.init(moduleData);

      final store = lib.create_store();
      int count = lib.rid_store_count(store);
      print('store count: $count');

      lib.rid_msg_Inc(reqId++);
      count = lib.rid_store_count(store);
      print('store count: $count');

      final reply = lib.rid_poll_reply();
      print('reply: ${reply?.debug(true)}');

      final dbgAddr = lib.rid_rawstore_debug_pretty(store);
      final str = lib.decodeUtf8String(dbgAddr);
      print('debug: $str');
    });
  });
}
