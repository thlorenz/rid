import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  rid.debugLock = null;
  final store = Store.instance;

  test('export: Vec<&Todo>', () {
    final todos = store.todosRef();
    expect(todos.length, 2, reason: 'todos len');
    expect(todos[0].id, 1, reason: 'todos idx id');
  });

  test('export: Vec<&u8>', () {
    final u8s = store.u8sRef();
    expect(u8s.length, 2, reason: 'u8s len');
    expect(u8s[0], 3, reason: 'u8s[0]');
  });

  test('export: Vec<&u16>', () {
    final u16s = store.u16sRef();
    expect(u16s.length, 2, reason: 'u16s len');
    expect(u16s[0], 5, reason: 'u16s[0]');
  });

  test('export: Vec<&Filter>', () {
    final filters = store.filtersRef();
    expect(filters.length, 2, reason: 'filters len');
    expect(filters[0], Filter.Completed, reason: 'filters idx');
  });

  test('export: Vec<&string>', () {
    final strings = store.stringsRef();
    expect(strings.length, 2, reason: 'strings len');
    expect(strings[0], "hello", reason: 'strings[0]');
    expect(strings[1], "world", reason: 'strings[1]');
  });

  test('export: Vec<&cstring>', () {
    final cstrings = store.cstringsRef();
    expect(cstrings.length, 2, reason: 'cstrings len');
    expect(cstrings[0], "hello", reason: 'cstrings[0]');
    expect(cstrings[1], "world", reason: 'cstrings[1]');
  });

  test('export: Vec<&str>', () {
    final strs = store.strRef();
    expect(strs.length, 2, reason: 'strs len');
    expect(strs[0], "hello", reason: 'strs[0]');
    expect(strs[1], "world", reason: 'strs[1]');
  });
}
