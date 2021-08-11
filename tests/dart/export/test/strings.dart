import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('export: strings owned', () {
    final store = Store.instance;
    expect(store.titleOwned(), "T-shirt Store", reason: 'String');
    expect(store.ctitleOwned(), "C-shirt Store", reason: 'CString');
  });
}
