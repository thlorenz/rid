use std::collections::HashMap;

use rid::RidStore;

#[rid::store]
#[rid::structs(Point)]
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

    // -----------------
    // Strings
    // -----------------
    string_u8s: HashMap<String, u8>,
    string_points: HashMap<String, Point>,
}

#[rid::model]
pub struct Point {
    x: i32,
    y: i32,
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

        let mut string_u8s = HashMap::new();
        string_u8s.insert(String::from("key1"), 1);
        string_u8s.insert(String::from("key2"), 2);
        string_u8s.insert(String::from("key3"), 3);

        let mut string_points = HashMap::new();
        string_points.insert(String::from("upper left"), Point { x: 0, y: 0 });
        string_points
            .insert(String::from("upper right"), Point { x: 100, y: 0 });
        string_points
            .insert(String::from("lower left"), Point { x: 0, y: 100 });
        string_points
            .insert(String::from("lower right"), Point { x: 100, y: 1000 });
        Self {
            u8s,
            u32s,
            i8s,
            i64s,
            u8_i8s,
            i64_u32s,
            string_u8s,
            string_points,
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
