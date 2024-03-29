use std::ffi::CString;

use rid::RidStore;

#[rid::store]
pub struct Store {
    title: String,
    ctitle: CString,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            title: "T-shirt Store".to_string(),
            ctitle: CString::new("C-shirt Store").unwrap(),
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
