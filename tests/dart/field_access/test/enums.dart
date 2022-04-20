import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  rid.debugLock = null;
  test('field_access: enums', () {
    final store = Store.instance;

    expect(store.raw.filter, 1, reason: 'raw enum: filter');
    expect(store.filter, Filter.Completed, reason: 'enum: filter');
  });
}
