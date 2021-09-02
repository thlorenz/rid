import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  RID_DEBUG_LOCK = null;
  RID_DEBUG_REPLY = null;

  // -----------------
  // Primitives key/val same type
  // -----------------

  // -----------------
  // unsigned ints
  // -----------------
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
    final u8s = store.u8s;

    expect(u8s.length, 3, reason: 'u8s len');
    expect(u8s.containsKey(1), true, reason: 'u8s containsKey 1 -> true');
    expect(u8s.containsKey(2), true, reason: 'u8s containsKey 2 -> true');
    expect(u8s.containsKey(5), false, reason: 'u8s containsKey 5 -> false');

    expect(u8s[1], 11, reason: 'u8s get(1)');
    expect(u8s[3], 33, reason: 'u8s get(3)');
    expect(u8s[5], null, reason: 'u8s get(5) -> null');
  });

  test('field_access: HashMap<u32, u32>', () {
    final store = Store.instance;
    final u32s = store.u32s;

    expect(u32s.length, 3, reason: 'u32s len');
    expect(u32s.containsKey(11), true, reason: 'u32s containsKey 11 -> true');
    expect(u32s.containsKey(22), true, reason: 'u32s containsKey 22 -> true');
    expect(u32s.containsKey(55), false, reason: 'u32s containsKey 55 -> false');

    expect(u32s[11], 111, reason: 'u32s get(11)');
    expect(u32s[33], 333, reason: 'u32s get(33)');
    expect(u32s[55], null, reason: 'u32s get(5) -> null');
  });

  // -----------------
  // signed ints
  // -----------------
  test('field_access: HashMap<i8, i8>', () {
    final store = Store.instance;
    final i8s = store.i8s;

    expect(i8s.length, 3, reason: 'i8s len');
    expect(i8s.containsKey(-1), true, reason: 'i8s containsKey -1 -> true');
    expect(i8s.containsKey(-2), true, reason: 'i8s containsKey -2 -> true');
    expect(i8s.containsKey(-5), false, reason: 'i8s containsKey -5 -> false');

    expect(i8s[-1], -11, reason: 'i8s get(-1)');
    expect(i8s[-3], -33, reason: 'i8s get(-3)');
    expect(i8s[-5], null, reason: 'i8s get(-5) -> null');
  });

  test('field_access: HashMap<i64, i64>', () {
    final store = Store.instance;
    final i64s = store.i64s;

    expect(i64s.length, 3, reason: 'i64s len');
    expect(i64s.containsKey(-11), true, reason: 'i64s containsKey -11 -> true');
    expect(i64s.containsKey(-22), true, reason: 'i64s containsKey -22 -> true');
    expect(i64s.containsKey(-55), false,
        reason: 'i64s containsKey -55 -> false');

    expect(i64s[-11], -111, reason: 'i64s get(-11)');
    expect(i64s[-33], -333, reason: 'i64s get(-33)');
    expect(i64s[-55], null, reason: 'i64s get-(55) -> null');
  });
}
