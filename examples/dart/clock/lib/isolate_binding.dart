import 'dart:ffi';
import 'dart:io';

import 'package:clock/generated/ffigen_binding.dart';

///
/// Rid generated dynamic library open and export
///
DynamicLibrary _open() {
  if (Platform.isLinux)
    return DynamicLibrary.open(
        '/Volumes/d/dev/fluster/rid/rid/examples/dart/clock/../../../target/debug/libclock.so');
  if (Platform.isMacOS)
    return DynamicLibrary.open(
        '/Volumes/d/dev/fluster/rid/rid/examples/dart/clock/../../../target/debug/libclock.dylib');
  throw UnsupportedError(
      'Platform "${Platform.operatingSystem}" is not supported.');
}

final DynamicLibrary _dl = _open();
final rid_ffi = NativeLibrary(_dl);

///
/// Binding to `allo-isolate` crate
///
void store_dart_post_cobject(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
) {
  _store_dart_post_cobject(ptr);
}

final _store_dart_post_cobject_Dart _store_dart_post_cobject = _dl
    .lookupFunction<_store_dart_post_cobject_C, _store_dart_post_cobject_Dart>(
        'store_dart_post_cobject');
typedef _store_dart_post_cobject_C = Void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
typedef _store_dart_post_cobject_Dart = void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
