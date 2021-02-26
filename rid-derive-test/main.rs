use std::ffi::CString;

use rid_derive::Rid;

#[derive(Rid)]
pub struct Simple {
    prim_u8: u8,
    prim_u16: u16,
    prim_u64: u64,
    cstring: CString,
    string: String,
    f: bool,
}
fn main() {}
