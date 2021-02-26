extension PointerRidBindFoo on Pointer<ridBind.Foo> {
  @ffi.Int32() int get prim_u8 => rid_ffi.rid_foo_prim_u8(this);
  @ffi.Int32() int get prim_u16 => rid_ffi.rid_foo_prim_u16(this);
}
extension PointerRidBindBar on Pointer<ridBind.Bar> {
  int get f => rid_ffi.rid_bar_f(this) != 0;
}
extension PointerRidBindBaz on Pointer<ridBind.Baz> {
  String get name => {
    int len = rid_ffi.rid_baz_name_len(this);
    return rid_ffi.rid_baz_name(this).toDartString(len);
  }
}
