/* Generated with cbindgen:0.17.0 */

#include "stdint.h"

typedef struct Bar Bar;

typedef struct Baz Baz;

typedef struct Foo Foo;

/**
 * FFI access methods generated for struct 'Foo'.
 *
 * Below is the dart extension to call those methods.
 *
 * ```dart
 * extension Rid_ExtOnPointerFoo on Pointer<ffigen_bind.Foo> {
 * @ffi.Int32() int get prim_u8 => rid_ffi.rid_foo_prim_u8(this);
 * @ffi.Int32() int get prim_u16 => rid_ffi.rid_foo_prim_u16(this);
 * }
 * ```
 */
uint8_t rid_foo_prim_u8(struct Foo *ptr);

uint16_t rid_foo_prim_u16(struct Foo *ptr);

/**
 * FFI access methods generated for struct 'Bar'.
 *
 * Below is the dart extension to call those methods.
 *
 * ```dart
 * extension Rid_ExtOnPointerBar on Pointer<ffigen_bind.Bar> {
 * int get f => rid_ffi.rid_bar_f(this) != 0;
 * }
 * ```
 */
bool rid_bar_f(struct Bar *ptr);

/**
 * ```dart
 * extension Rid_ExtOnPointerBaz on Pointer<ffigen_bind.Baz> {
 * String get name => {
 *   int len = rid_ffi.rid_baz_name_len(this);
 *   return rid_ffi.rid_baz_name(this).toDartString(len);
 * }
 * }
 * ```
 */
const char *rid_baz_name(struct Baz *ptr);

uintptr_t rid_baz_name_len(struct Baz *ptr);
