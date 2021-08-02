use rid::RidStore;

#[rid::store]
#[rid::enums(Filter)]
pub struct Store {
    filter: Filter,
}

#[rid::model]

pub enum Filter {
    All,
    Completed,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            filter: Filter::Completed,
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
