use std::ffi::CString;

use rid::RidStore;

#[rid::store]
#[rid::structs(Todo)]
#[rid::enums(Filter)]
pub struct Store {
    todos: Vec<Todo>,
    u8s: Vec<u8>,
    u16s: Vec<u16>,
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
            u16s: vec![5, 6],
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

#[rid::export]
#[rid::structs(Todo)]
#[rid::enums(Filter)]
impl Store {
    // references
    #[rid::export]
    pub fn todos_ref(&self) -> Vec<&Todo> {
        self.todos.iter().collect()
    }

    #[rid::export]
    pub fn u8s_ref(&self) -> Vec<&u8> {
        self.u8s.iter().collect()
    }

    #[rid::export]
    pub fn u16s_ref(&self) -> Vec<&u16> {
        self.u16s.iter().collect()
    }

    #[rid::export]
    pub fn filters_ref(&self) -> Vec<&Filter> {
        self.filters.iter().collect()
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
