import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  rid.debugLock = null;

  group("export-functions", () {
    test('string args', () {
      final len =
          rid_ffi.rid_export_get_string_len('Hello World'.toNativeInt8());
      expect(len, 11, reason: 'rid_export_get_string_len: len');
    });
  });

  group("export-methods", () {
    final store = Store.instance;

    test('direct: string args', () {
      final len = rid_ffi.rid_export_Store_get_string_len(
          store.raw, 'Hello World'.toNativeInt8());

      expect(len, 11, reason: 'rid_export_Store_get_string_len: len');
    });

    test('instance: string args', () {
      final len = store.getStringLen('Hello World');
      expect(len, 11, reason: 'store.getStringLen: len');
    });
  });
}
