use rid_derive::Rid;
use std::ffi::CString;

#[derive(Debug, Rid)]
pub struct Todo {
    id: u8, // TODO: support usize,
    title: CString,
    completed: bool,
}
