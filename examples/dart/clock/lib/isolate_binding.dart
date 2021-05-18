import 'dart:ffi' as dart_ffi;
import 'dart:io';

import 'package:clock/generated/ffigen_binding.dart';

// -----------------
// Rid generated dynamic library open and export
// -----------------
dart_ffi.DynamicLibrary _open() {
  if (Platform.isLinux)
    return dart_ffi.DynamicLibrary.open(
        '/Volumes/d/dev/fluster/rid/rid/examples/dart/clock/../../../target/debug/libclock.so');
  if (Platform.isMacOS)
    return dart_ffi.DynamicLibrary.open(
        '/Volumes/d/dev/fluster/rid/rid/examples/dart/clock/../../../target/debug/libclock.dylib');
  throw UnsupportedError(
      'Platform "${Platform.operatingSystem}" is not supported.');
}

final dart_ffi.DynamicLibrary _dl = _open();
final rid_ffi = NativeLibrary(_dl);

bool _initializedIsolate = false;

// -----------------
// Binding to initIsolate defined in rid-ffi
// -----------------
void initIsolate(
  int port,
) {
  if (!_initializedIsolate) {
    _initializedIsolate = true;
    _store_dart_post_cobject(dart_ffi.NativeApi.postCObject);
    _rid_init_isolate(
      port,
    );
  }
}

late final _rid_init_isolate_ptr = _dl
    .lookup<dart_ffi.NativeFunction<_c_rid_init_isolate>>('rid_init_isolate');
late final _dart_rid_init_isolate _rid_init_isolate =
    _rid_init_isolate_ptr.asFunction<_dart_rid_init_isolate>();

typedef _c_rid_init_isolate = dart_ffi.Void Function(
  dart_ffi.Int64 port,
);

typedef _dart_rid_init_isolate = void Function(
  int port,
);

// -----------------
// Binding to `allo-isolate` crate
// -----------------

final _store_dart_post_cobject_Dart _store_dart_post_cobject = _dl
    .lookupFunction<_store_dart_post_cobject_C, _store_dart_post_cobject_Dart>(
        'store_dart_post_cobject');
typedef _store_dart_post_cobject_C = dart_ffi.Void Function(
  dart_ffi.Pointer<
          dart_ffi.NativeFunction<
              dart_ffi.Int8 Function(
                  dart_ffi.Int64, dart_ffi.Pointer<dart_ffi.Dart_CObject>)>>
      ptr,
);
typedef _store_dart_post_cobject_Dart = void Function(
  dart_ffi.Pointer<
          dart_ffi.NativeFunction<
              dart_ffi.Int8 Function(
                  dart_ffi.Int64, dart_ffi.Pointer<dart_ffi.Dart_CObject>)>>
      ptr,
);
