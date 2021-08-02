import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('field_access: structs', () {
    final store = Store.instance;
    expect(store.raw.todo.id, 1, reason: 'raw todo id');
    expect(store.todo.id, 1, reason: 'todo id');
  });
}
