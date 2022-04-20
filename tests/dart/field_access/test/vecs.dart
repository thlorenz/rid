import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  final store = Store.instance;
  rid.debugLock = null;

  test('field_access: Vec<struct>', () {
    expect(store.raw.todos.length, 2, reason: 'raw todos len');
    expect(store.raw.todos.iter().first.id, 1,
        reason: 'raw todos iter first id');
    expect(store.raw.todos[1].id, 2, reason: 'raw todos idx id');

    expect(store.todos.length, 2, reason: 'todos len');
    expect(store.todos[0].id, 1, reason: 'todos idx id');
  });

  test('field_access: Vec<u8>', () {
    expect(store.raw.u8s.length, 2, reason: 'raw u8s len');
    expect(store.raw.u8s.iter().first, 3, reason: 'raw u8s iter first');
    expect(store.raw.u8s[1], 4, reason: 'raw u8s idx');

    expect(store.u8s.length, 2, reason: 'u8s len');
    expect(store.u8s[0], 3, reason: 'u8s idx');
  });

  test('field_access: Vec<enum>', () {
    expect(store.raw.filters.length, 2, reason: 'raw filters len');
    expect(store.raw.filters.iter().first, 1, reason: 'raw filters iter first');
    expect(store.raw.filters[1], 0, reason: 'raw filters idx');

    expect(store.filters.length, 2, reason: 'filters len');
    expect(store.filters[0], Filter.Completed, reason: 'filters idx');
  });

  test('field_access: Vec<String>', () {
    expect(store.raw.strings.length, 2, reason: 'raw strings len');
    expect(store.raw.strings.iter().first, 'hello',
        reason: 'raw strings iter first');
    expect(store.raw.strings[1], 'world', reason: 'raw strings idx');

    expect(store.strings.length, 2, reason: 'strings len');
    expect(store.strings[0], 'hello', reason: 'strings idx');
  });

  test('field_access: Vec<CString>', () {
    expect(store.raw.cstrings.length, 2, reason: 'raw cstrings len');
    expect(store.raw.cstrings.iter().first, 'hello',
        reason: 'raw cstrings iter first');
    expect(store.raw.cstrings[1], 'world', reason: 'raw cstrings idx');

    expect(store.cstrings.length, 2, reason: 'cstrings len');
    expect(store.cstrings[0], 'hello', reason: 'cstrings idx');
  });
}
