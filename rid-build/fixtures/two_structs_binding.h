/* Generated with cbindgen:0.17.0 */

#include "stdint.h"

typedef struct Bar Bar;

typedef struct Foo Foo;

/**
 * FFI access methods generated for struct 'Foo'.
 *
 * Below is the dart extension to call those methods.
 *
 * ```dart
 * extension Rid_ExtOnPointerFoo on dart_ffi.Pointer<ffigen_bind.Foo> {
 * @dart_ffi.Int32()
 * int get prim_u8 => rid_ffi.rid_foo_prim_u8(this);
 * }
 * ```
 */
uint8_t rid_foo_prim_u8(struct Foo *ptr);

/**
 * FFI access methods generated for struct 'Bar'.
 *
 * Below is the dart extension to call those methods.
 *
 * ```dart
 * extension Rid_ExtOnPointerBar on dart_ffi.Pointer<ffigen_bind.Bar> {
 * bool get f => rid_ffi.rid_bar_f(this) != 0;
 * }
 * ```
 */
bool rid_bar_f(struct Bar *ptr);
