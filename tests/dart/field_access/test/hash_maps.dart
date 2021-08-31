import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('field_access: raw HashMap<u8, u8>', () {
    final store = Store.instance;

    expect(store.raw.u8s.length, 3, reason: 'raw u8s len');
    expect(store.raw.u8s.contains(1), true,
        reason: 'raw u8s contains 1 -> true');
    expect(store.raw.u8s.contains(2), true,
        reason: 'raw u8s contains 2 -> true');
    expect(store.raw.u8s.contains(5), false,
        reason: 'raw u8s contains 5 -> false');

    expect(store.raw.u8s.get(1), 11, reason: 'raw u8s get(1)');
    expect(store.raw.u8s.get(3), 33, reason: 'raw u8s get(3)');
    expect(store.raw.u8s.get(5), null, reason: 'raw u8s get(5) -> null');
  });

  test('field_access: HashMap<u8, u8>', () {
    final store = Store.instance;

    expect(store.u8s.length, 3, reason: 'u8s len');
    expect(store.u8s.containsKey(1), true, reason: 'u8s containsKey 1 -> true');
    expect(store.u8s.containsKey(2), true, reason: 'u8s containsKey 2 -> true');
    expect(store.u8s.containsKey(5), false,
        reason: 'u8s containsKey 5 -> false');

    final u8s = store.u8s;
    expect(u8s[1], 11, reason: 'u8s get(1)');
    expect(u8s[3], 33, reason: 'u8s get(3)');
    expect(u8s[5], null, reason: 'u8s get(5) -> null');
  });
}
