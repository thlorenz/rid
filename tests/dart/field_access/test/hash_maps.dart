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

  // -----------------
  // Primitives key/val different types
  // -----------------
  test('field_access: HashMap<u8, i8>', () {
    final store = Store.instance;
    final u8_i8s = store.u8I8s;

    expect(u8_i8s.length, 3, reason: 'u8_i8s len');
    expect(u8_i8s.containsKey(1), true, reason: 'u8_i8s containsKey 1 -> true');
    expect(u8_i8s.containsKey(2), true, reason: 'u8_i8s containsKey 2 -> true');
    expect(u8_i8s.containsKey(5), false,
        reason: 'u8_i8s containsKey 5 -> false');

    expect(u8_i8s[1], -11, reason: 'u8_i8s get(1)');
    expect(u8_i8s[3], -33, reason: 'u8_i8s get(3)');
    expect(u8_i8s[5], null, reason: 'u8_i8s get(5) -> null');
  });

  test('field_access: HashMap<i64, u32>', () {
    final store = Store.instance;
    final i64_u32s = store.i64U32s;

    expect(i64_u32s.length, 3, reason: 'i64_u32s len');
    expect(i64_u32s.containsKey(-1E6), true,
        reason: 'i64_u32s containsKey -1E6-> true');
    expect(i64_u32s.containsKey(-2E9), true,
        reason: 'i64_u32s containsKey -2E9 -> true');
    expect(i64_u32s.containsKey(-3E12), true,
        reason: 'i64_u32s containsKey -3E12 -> true');
    expect(i64_u32s.containsKey(5), false,
        reason: 'i64_u32s containsKey 5 -> false');

    expect(i64_u32s[-1E6], 1, reason: 'i64_u32s get(-1E6)');
    expect(i64_u32s[-3E12], 3, reason: 'i64_u32s get(-3E12)');
    expect(i64_u32s[5], null, reason: 'i64_u32s get(5) -> null');
  });

  // -----------------
  // String keys
  // -----------------
  test('field_access: HashMap<String, u8>', () {
    final store = Store.instance;
    final string_u8s = store.stringU8s;

    expect(string_u8s.length, 3, reason: 'string_u8s len');
    expect(string_u8s.containsKey('key1'), true,
        reason: 'string_u8s containsKey key1 -> true');
    expect(string_u8s.containsKey('key2'), true,
        reason: 'string_u8s containsKey key2 -> true');
    expect(string_u8s.containsKey('key5'), false,
        reason: 'string_u8s containsKey key5 -> false');

    expect(string_u8s['key1'], 1, reason: 'string_u8s get(key1)');
    expect(string_u8s['key3'], 3, reason: 'string_u8s get(key3)');
    expect(string_u8s['key5'], null, reason: 'string_u8s get(key5) -> null');
  });

  test('field_access: HashMap<String, Point>', () {
    final store = Store.instance;
    final string_points = store.stringPoints;

    expect(string_points.length, 4, reason: 'string_points len');
    expect(string_points.containsKey('upper left'), true,
        reason: 'string_points containsKey upper left -> true');
    expect(string_points.containsKey('lower right'), true,
        reason: 'string_points containsKey lower right -> true');
    expect(string_points.containsKey('center'), false,
        reason: 'string_points containsKey center -> false');

    expect(string_points['upper left'].toString(), 'Point{x: 0, y: 0}',
        reason: 'string_points get(upper left)');
    expect(string_points['upper right'].toString(), 'Point{x: 100, y: 0}',
        reason: 'string_points get(upper right)');
    expect(string_points['center'], null,
        reason: 'string_points get(center) -> null');
  });
}
