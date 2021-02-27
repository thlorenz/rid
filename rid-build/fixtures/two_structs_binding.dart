import 'dart:ffi' as dart_ffi;
import 'dart:io' as dart_io;
import 'package:ffi/ffi.dart' as package_ffi;
import './ffigen_binding.dart' as ffigen_bind;

// Forwarding Dart Types for Rust structs
export './ffigen_binding.dart' show Bar, Foo;

//
// Open Dynamic Library
//
dart_ffi.DynamicLibrary _open() {
  if (dart_io.Platform.isLinux)
    return dart_ffi.DynamicLibrary.open('target/debug/libapp_todo.so');
  if (dart_io.Platform.isMacOS)
    return dart_ffi.DynamicLibrary.open('target/debug/libapp_todo.dylib');
  throw UnsupportedError('This platform is not supported.');
}

//
// Extensions to provide an API into FFI calls to Rust
//
extension Rid_ExtOnPointerFoo on dart_ffi.Pointer<ffigen_bind.Foo> {
  @dart_ffi.Int32()
  int get prim_u8 => rid_ffi.rid_foo_prim_u8(this);
}

extension Rid_ExtOnPointerBar on dart_ffi.Pointer<ffigen_bind.Bar> {
  bool get f => rid_ffi.rid_bar_f(this) != 0;
}

extension Rid_ExtOnPointerInt8 on dart_ffi.Pointer<dart_ffi.Int8> {
  String toDartString([int? len]) {
    final dart_ffi.Pointer<package_ffi.Utf8> stringPtr = this.cast();
    return stringPtr.toDartString(length: len);
  }
}

//
// Exporting Native Library to call Rust functions directly
//
final dart_ffi.DynamicLibrary _dl = _open();
final rid_ffi = ffigen_bind.NativeLibrary(_dl);
