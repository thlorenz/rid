import 'dart:ffi' as dart_ffi;

import 'package:wasm/wasm.dart';
import 'package:wasm_example/wasm_binding.dart';

bool _initializedIsolate = false;

// -----------------
// Binding to `allo-isolate` crate
// -----------------
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

// -----------------
// Binding to initIsolate defined in rid-ffi
// -----------------
typedef _c_rid_init_isolate = dart_ffi.Void Function(
  dart_ffi.Int64 port,
);

void initIsolate(
  WasmLibrary dl,
  int port,
  bool isDebugMode,
) {
  // allo isolate crate initialization
  final WasmFunction _store_dart_post_cobject = dl.lookupFunction<
      _store_dart_post_cobject_C,
      _store_dart_post_cobject_Dart>('store_dart_post_cobject');

  final WasmFunction _rid_init_isolate = dl
      .lookup<dart_ffi.NativeFunction<_c_rid_init_isolate>>('rid_init_isolate');

  if (!_initializedIsolate || isDebugMode) {
    _initializedIsolate = true;
    _store_dart_post_cobject.apply([dart_ffi.NativeApi.postCObject.address]);
    _rid_init_isolate.apply([port]);
  } else if (_initializedIsolate) {
    throw Exception(
        "The isolate can only be initialized once when not run in debug mode");
  }
}
