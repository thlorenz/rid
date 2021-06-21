use std::{thread, time};

use rid::RidStore;

// -----------------
// Store
// -----------------
#[rid::store]
#[derive(Debug)]
pub struct Store {
    running: bool,
    elapsed_secs: u32,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            running: false,
            elapsed_secs: 0,
        }
    }

    fn update(&mut self, req_id: u64, msg: Msg) {
        match msg {
            Msg::Start => {
                self.start();
                rid::post(Reply::Started(req_id));
            }
            Msg::Stop => {
                self.running = false;
                rid::post(Reply::Stopped(req_id));
            }
            Msg::Reset => {
                let current_elapsed = format!("Elapsed: {}", self.elapsed_secs);
                self.elapsed_secs = 0;
                rid::post(Reply::Reset(req_id, current_elapsed));
            }
        }
    }
}

impl Store {
    fn start(&mut self) {
        if !self.running {
            self.running = true;
            thread::spawn(move || {
                while store::read().running {
                    store::write().elapsed_secs += 1;
                    rid::post(Reply::Tick);
                    thread::sleep(time::Duration::from_secs(1));
                }
            });
        }
    }
}

// -----------------
// Msg
// -----------------
#[rid::message(Reply)]
pub enum Msg {
    Start,
    Stop,
    Reset,
}

// -----------------
// Reply
// -----------------
#[rid::reply]
enum Reply {
    Started(u64),
    Stopped(u64),
    Reset(u64, String),
    Tick,
}
