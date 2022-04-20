use std::ffi::CString;

use rid::RidStore;

#[rid::store]
#[rid::structs(Todo)]
#[rid::enums(Filter)]
pub struct Store {
    todos: Vec<Todo>,
    u8s: Vec<u8>,
    filters: Vec<Filter>,
    strings: Vec<String>,
    cstrings: Vec<CString>,
}

#[rid::model]
pub enum Filter {
    All,
    Completed,
}

#[rid::model]
pub struct Todo {
    id: u8,
}
impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            todos: vec![Todo { id: 1 }, Todo { id: 2 }],
            u8s: vec![3, 4],
            filters: vec![Filter::Completed, Filter::All],
            strings: vec!["hello".to_string(), "world".to_string()],
            cstrings: vec![
                CString::new("hello").unwrap(),
                CString::new("world").unwrap(),
            ],
        }
    }

    fn update(&mut self, _req_id: u64, _msg: Msg) {
        unimplemented!()
    }
}

#[rid::message(Reply)]
pub enum Msg {
    NotUsed,
}
#[rid::reply]
pub enum Reply {
    NotUsed,
}
