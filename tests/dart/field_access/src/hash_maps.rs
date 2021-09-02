use std::collections::HashMap;

use rid::RidStore;

#[rid::store]
pub struct Store {
    u8s: HashMap<u8, u8>,
    u32s: HashMap<u32, u32>,
    i8s: HashMap<i8, i8>,
    i64s: HashMap<i64, i64>,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        let mut u8s = HashMap::new();
        u8s.insert(1, 11);
        u8s.insert(2, 22);
        u8s.insert(3, 33);

        let mut u32s = HashMap::new();
        u32s.insert(11, 111);
        u32s.insert(22, 222);
        u32s.insert(33, 333);

        let mut i8s = HashMap::new();
        i8s.insert(-1, -11);
        i8s.insert(-2, -22);
        i8s.insert(-3, -33);

        let mut i64s = HashMap::new();
        i64s.insert(-11, -111);
        i64s.insert(-22, -222);
        i64s.insert(-33, -333);
        Self {
            u8s,
            u32s,
            i8s,
            i64s,
        }
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
