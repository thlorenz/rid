use rid::RidStore;

#[rid::store]
#[derive(Debug, Clone)]
pub struct Store {
    count: u8,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, req_id: u64, msg: Msg) {
        match msg {
            Msg::Inc => {
                self.count += 1;
                rid::post(Reply::Inced(req_id));
            }
            Msg::Dump => {
                rid::post(Reply::Dumped(req_id));
            }
        }
    }
}

#[rid::message(Reply)]
#[derive(Debug, Clone)]
pub enum Msg {
    Inc,
    Dump,
}

#[rid::reply]
#[derive(Debug, Clone)]
pub enum Reply {
    Inced(u64),
    Dumped(u64),
}
