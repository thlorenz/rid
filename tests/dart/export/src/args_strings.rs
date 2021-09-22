use rid::RidStore;

// -----------------
// function exports
// -----------------
#[rid::export]
fn get_string_len(arg: String) -> usize {
    arg.len()
}

// -----------------
// method exports
// -----------------
#[rid::store]
pub struct Store {}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {}
    }

    fn update(&mut self, _req_id: u64, _msg: Msg) {
        todo!()
    }
}

#[rid::export]
impl Store {
    #[rid::export]
    fn get_string_len(&self, arg: String) -> usize {
        arg.len()
    }
}

enum Msg {}
