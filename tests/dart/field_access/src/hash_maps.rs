use std::collections::HashMap;

use rid::RidStore;

#[rid::store]
pub struct Store {
    // -----------------
    // Primitives key/val same type
    // -----------------
    u8s: HashMap<u8, u8>,
    u32s: HashMap<u32, u32>,
    i8s: HashMap<i8, i8>,
    i64s: HashMap<i64, i64>,
    // -----------------
    // Primitives key/val different types
    // -----------------
    u8_i8s: HashMap<u8, i8>,
    i64_u32s: HashMap<i64, u32>,
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

        let mut u8_i8s = HashMap::new();
        u8_i8s.insert(1, -11);
        u8_i8s.insert(2, -22);
        u8_i8s.insert(3, -33);

        let mut i64_u32s = HashMap::new();
        i64_u32s.insert(-1_000_000, 1);
        i64_u32s.insert(-2_000_000_000, 2);
        i64_u32s.insert(-3_000_000_000_000, 3);

        Self {
            u8s,
            u32s,
            i8s,
            i64s,
            u8_i8s,
            i64_u32s,
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
