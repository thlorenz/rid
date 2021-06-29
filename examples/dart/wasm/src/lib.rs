use rid::RidStore;
mod replies;

#[rid::store]
#[rid::structs(ReplyStruct)]
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
                eprintln!("rust: counting");
                replies::post(Reply::Inced(req_id));
            }
            Msg::Dump => {
                replies::post(Reply::Dumped(req_id));
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

// -----------------
// Reply
// -----------------
#[rid::reply]
#[derive(Debug, Clone)]
pub enum Reply {
    Inced(u64),
    Dumped(u64),
}
