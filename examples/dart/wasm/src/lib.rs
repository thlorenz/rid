use std::sync::RwLockWriteGuard;

use rid::RidStore;

#[rid::store]
#[rid::structs(ReplyStruct)]
#[derive(Debug, Clone)]
pub struct Store {
    count: u8,
    posted_replies: Vec<ReplyStruct>,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            count: 0,
            posted_replies: vec![],
        }
    }

    fn update(&mut self, req_id: u64, msg: Msg) {
        match msg {
            Msg::Inc => {
                self.count += 1;
                self.post(Reply::Inced(req_id));
            }
            Msg::Dump => {
                self.post(Reply::Dumped(req_id));
            }
        }
    }
}

#[rid::export]
#[rid::structs(ReplyStruct)]
impl Store {
    fn post(&mut self, reply: Reply) {
        self.posted_replies.push(reply.into())
    }

    #[rid::export]
    fn poll_reply(&self) -> Option<&ReplyStruct> {
        self.posted_replies.iter().next()
    }
}

fn write() -> RwLockWriteGuard<'static, Store> {
    store::write()
}

#[no_mangle]
pub extern "C" fn handled_reply(req_id: u64) {
    write().posted_replies.retain(|x| x.req_id != req_id)
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

#[rid::model]
#[derive(Debug, Clone)]
pub struct ReplyStruct {
    ty: u8,
    req_id: u64,
}

impl From<Reply> for ReplyStruct {
    fn from(reply: Reply) -> Self {
        match reply {
            Reply::Inced(req_id) => Self { ty: 0, req_id },
            Reply::Dumped(req_id) => Self { ty: 1, req_id },
        }
    }
}
