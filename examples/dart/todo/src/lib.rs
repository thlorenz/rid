use rid_derive::Rid;
/*
use std::ffi::CString;

#[derive(Debug, Rid)]
pub struct Todo {
    id: usize,
    title: CString,
    completed: bool,
}
*/
#[derive(Debug, Rid)]
pub struct Model {
    ids: Vec<u8>,
}
