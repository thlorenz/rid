use std::ffi::CString;

use rid_derive::Rid;

#[derive(Rid)]
pub struct Foo {
    prim_u8: u8,
    prim_u16: u16,
}

#[derive(Rid)]
pub struct Bar {
    f: bool,
}

#[derive(Rid)]
pub struct Baz {
    name: CString,
}
fn main() {}
