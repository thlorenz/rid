use std::collections::HashMap;

use rid::RidStore;

#[rid::store]
pub struct Store {
    u8s: HashMap<u8, u8>,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        let mut u8s = HashMap::new();
        u8s.insert(1, 11);
        u8s.insert(2, 22);
        u8s.insert(3, 33);
        Self { u8s }
    }

    fn update(&mut self, _req_id: u64, _msg: Msg) {
        todo!()
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
