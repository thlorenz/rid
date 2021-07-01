use rid::RidStore;
mod replies;

#[rid::store]
#[rid::structs(ReplyStruct)]
#[derive(Debug, Clone)]
pub struct Store {
    count: u32,
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
            Msg::Add(n) => {
                self.count += n;
                replies::post(Reply::Added(req_id, n.to_string()));
            }
        }
    }
}

#[rid::message(Reply)]
#[derive(Debug, Clone)]
pub enum Msg {
    Inc,
    Add(u32),
}

// -----------------
// Reply
// -----------------
#[rid::reply]
#[derive(Debug, Clone)]
pub enum Reply {
    Inced(u64),
    Added(u64, String),
}
