import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  rid.debugLock = null;
  test('field_access: strings', () {
    final store = Store.instance;
    expect(store.title, "T-shirt Store", reason: 'String');
    expect(store.ctitle, "C-shirt Store", reason: 'CString');
  });
}
