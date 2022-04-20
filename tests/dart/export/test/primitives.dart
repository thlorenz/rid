import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  rid.debugLock = null;

  test('export: impl methods primitives owned', () {
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

  test('export: impl methods primitives refs', () {
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

  test('export: functions primitives owned', () {
    // unsigned ints
    expect(rid_ffi.rid_export_fn_s_id_owned(), 8, reason: 'u8');
    expect(rid_ffi.rid_export_fn_m_id_owned(), 16, reason: 'u16');
    expect(rid_ffi.rid_export_fn_l_id_owned(), 32, reason: 'u32');
    expect(rid_ffi.rid_export_fn_xl_id_owned(), 64, reason: 'u64');

    // signed ints
    expect(rid_ffi.rid_export_fn_s_signed_owned(), -8, reason: 'i8');
    expect(rid_ffi.rid_export_fn_m_signed_owned(), -16, reason: 'i16');
    expect(rid_ffi.rid_export_fn_l_signed_owned(), -32, reason: 'i32');
    expect(rid_ffi.rid_export_fn_xl_signed_owned(), -64, reason: 'i64');

    // bool
    expect(rid_ffi.rid_export_fn_ok_owned(), 1, reason: 'bool: true');
    expect(rid_ffi.rid_export_fn_not_ok_owned(), 0, reason: 'bool: false');
  });
}
