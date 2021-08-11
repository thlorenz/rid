import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('field_access: enums', () {
    final store = Store.instance;

    expect(store.filterOwned(), Filter.Completed, reason: 'enum: filter owned');
  });
}
