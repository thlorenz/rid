import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('export: strings owned', () {
    final store = Store.instance;
    // owned
    expect(store.titleOwned(), "T-shirt Store", reason: 'owned String');
    expect(store.ctitleOwned(), "C-shirt Store", reason: 'owned CString');

    // references
    expect(store.titleRef(), "T-shirt Store", reason: 'ref String');
    expect(store.ctitleRef(), "C-shirt Store", reason: 'ref CString');
    expect(store.titleAsStr(), "T-shirt Store", reason: 'as_str String');
  });
}
