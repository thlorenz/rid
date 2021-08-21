import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('export: primitives owned', () {
    final store = Store.instance;
    // unsigned ints
    expect(store.sIdOwned(), 1, reason: 'u8');
    expect(store.mIdOwned(), 10, reason: 'u16');
    expect(store.lIdOwned(), 100, reason: 'u32');
    expect(store.xlIdOwned(), 1000, reason: 'u64');

    // signed ints
    expect(store.sSignedOwned(), -1, reason: 'i8');
    expect(store.mSignedOwned(), -10, reason: 'i16');
    expect(store.lSignedOwned(), -100, reason: 'i32');
    expect(store.xlSignedOwned(), -1000, reason: 'i64');

    // bool
    expect(store.okOwned(), true, reason: 'bool: true');
    expect(store.notOkOwned(), false, reason: 'bool: false');
  });

  test('export: primitives refs', () {
    final store = Store.instance;
    // unsigned ints
    expect(store.sIdRef(), 1, reason: '&u8');
    expect(store.mIdRef(), 10, reason: '&u16');
    expect(store.lIdRef(), 100, reason: '&u32');
    expect(store.xlIdRef(), 1000, reason: '&u64');

    // signed ints
    expect(store.sSignedRef(), -1, reason: '&i8');
    expect(store.mSignedRef(), -10, reason: '&i16');
    expect(store.lSignedRef(), -100, reason: '&i32');
    expect(store.xlSignedRef(), -1000, reason: '&i64');

    // bool
    expect(store.okRef(), true, reason: 'bool: true');
    expect(store.notOkRef(), false, reason: 'bool: false');
  });
}
