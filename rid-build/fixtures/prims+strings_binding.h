/* Generated with cbindgen:0.17.0 */

#include "stdint.h"

typedef struct Simple Simple;

/**
 * FFI access methods generated for struct 'Simple'.
 *
 * Below is the dart extension to call those methods.
 *
 * ```dart
 * extension Rid_ExtOnPointerSimple on Pointer<ffigen_bind.Simple> {
 * @ffi.Int32() int get prim_u8 => rid_ffi.rid_simple_prim_u8(this);
 * @ffi.Int32() int get prim_u16 => rid_ffi.rid_simple_prim_u16(this);
 * @ffi.Int64() int get prim_u64 => rid_ffi.rid_simple_prim_u64(this);
 * String get cstring => {
 *   int len = rid_ffi.rid_simple_cstring_len(this);
 *   return rid_ffi.rid_simple_cstring(this).toDartString(len);
 * }
 * String get string => {
 *   int len = rid_ffi.rid_simple_string_len(this);
 *   return rid_ffi.rid_simple_string(this).toDartString(len);
 * }
 * int get f => rid_ffi.rid_simple_f(this) != 0;
 * }
 * ```
 */
uint8_t rid_simple_prim_u8(struct Simple *ptr);

uint16_t rid_simple_prim_u16(struct Simple *ptr);

uint64_t rid_simple_prim_u64(struct Simple *ptr);

const char *rid_simple_cstring(struct Simple *ptr);

uintptr_t rid_simple_cstring_len(struct Simple *ptr);

const char *rid_simple_string(struct Simple *ptr);

uintptr_t rid_simple_string_len(struct Simple *ptr);

bool rid_simple_f(struct Simple *ptr);
