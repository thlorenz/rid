use std::ffi::CString;

#[derive(rid::Model)]
pub struct Foo {
    prim_u8: u8,
    prim_u16: u16,
}

#[derive(rid::Model)]
pub struct Bar {
    f: bool,
}

#[derive(rid::Model)]
pub struct Baz {
    name: CString,
}
