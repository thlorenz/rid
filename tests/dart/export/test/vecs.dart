import 'package:test/test.dart';

import '../lib/generated/rid_api.dart';

void main() {
  final store = Store.instance;

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
}
