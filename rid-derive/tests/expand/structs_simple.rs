#[macro_use]
extern crate rid_derive;
use rid_derive::Rid;

#[Derive(Rid)]
struct Empty;

#[Derive(Rid, Debug)]
struct SingleU8 {
    field_u8: u8,
}

fn test() {}
