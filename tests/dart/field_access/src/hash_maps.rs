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

#[rid::export]
impl Store {
    #[rid::export]
    pub fn u8_keys(&self) -> Vec<&u8> {
        self.u8s.keys().collect::<Vec<&u8>>()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rid_u8_keys(ptr: *const HashMap<u8, u8>) -> rid::RidVec<u8> {
    let map: &HashMap<u8, u8> = unsafe { ptr.as_ref().unwrap() };
    let ret = map.keys();
    let ret: Vec<u8> = ret.into_iter().map(|x| *x).collect();
    let ret_ptr = rid::RidVec::from(ret);
    ret_ptr
}
