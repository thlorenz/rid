extension PointerRidBindSimple on Pointer<ridBind.Simple> {
  @ffi.Int32() int get prim_u8 => rid_ffi.rid_simple_prim_u8(this);
  @ffi.Int32() int get prim_u16 => rid_ffi.rid_simple_prim_u16(this);
  @ffi.Int64() int get prim_u64 => rid_ffi.rid_simple_prim_u64(this);
  String get cstring => {
    int len = rid_ffi.rid_simple_cstring_len(this);
    return rid_ffi.rid_simple_cstring(this).toDartString(len);
  }
  String get string => {
    int len = rid_ffi.rid_simple_string_len(this);
    return rid_ffi.rid_simple_string(this).toDartString(len);
  }
  int get f => rid_ffi.rid_simple_f(this) != 0;
}
