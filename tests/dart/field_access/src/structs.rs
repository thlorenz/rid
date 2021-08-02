use rid::RidStore;

#[rid::store]
#[rid::structs(Todo)]
pub struct Store {
    todo: Todo,
}

#[rid::model]
pub struct Todo {
    id: u8,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            todo: Todo { id: 1 },
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
