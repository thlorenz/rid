import 'dart:collection';
import 'dart:ffi' as dart_ffi;

final _initializedIsolates = HashSet<String>();

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

typedef _dart_rid_init_isolate = void Function(
  int port,
);

void initIsolate(
  dart_ffi.DynamicLibrary dl,
  String initFunctionName,
  int port,
  bool isDebugMode,
) {
  // allo isolate crate initialization
  final _store_dart_post_cobject_Dart _store_dart_post_cobject =
      dl.lookupFunction<_store_dart_post_cobject_C,
          _store_dart_post_cobject_Dart>('store_dart_post_cobject');

  final _rid_init_isolate_ptr =
      dl.lookup<dart_ffi.NativeFunction<_c_rid_init_isolate>>(initFunctionName);
  final _dart_rid_init_isolate _rid_init_isolate =
      _rid_init_isolate_ptr.asFunction<_dart_rid_init_isolate>();

  final initializedIsolate = _initializedIsolates.contains(initFunctionName);
  if (!initializedIsolate || isDebugMode) {
    _initializedIsolates.add(initFunctionName);
    _store_dart_post_cobject(dart_ffi.NativeApi.postCObject);
    _rid_init_isolate(port);
  } else if (initializedIsolate) {
    throw Exception(
        "The isolate can only be initialized once when not run in debug mode");
  }
}
