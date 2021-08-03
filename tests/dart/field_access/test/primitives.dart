import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  test('field_access: primitives', () {
    final store = Store.instance;
    // unsigned ints
    expect(store.sId, 1, reason: 'u8');
    expect(store.mId, 10, reason: 'u16');
    expect(store.lId, 100, reason: 'u32');
    expect(store.xlId, 1000, reason: 'u64');

    // signed ints
    expect(store.sSigned, -1, reason: 'i8');
    expect(store.mSigned, -10, reason: 'i16');
    expect(store.lSigned, -100, reason: 'i32');
    expect(store.xlSigned, -1000, reason: 'i64');

    // bool
    expect(store.ok, true, reason: 'bool: true');
    expect(store.notOk, false, reason: 'bool: false');
  });
}
