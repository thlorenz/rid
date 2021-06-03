use std::{thread, time};

// -----------------
// Store
// -----------------
#[rid::model]
#[derive(Debug, rid::Debug)]
pub struct Store {
    running: bool,
    elapsed_secs: u32,
}

impl Store {
    fn create_store() -> Self {
        Self {
            running: false,
            elapsed_secs: 0,
        }
    }

    fn update(&mut self, req_id: u64, msg: Msg) {
        match msg {
            Msg::Start => {
                self.start();
                rid::post(Post::Started(req_id));
            }
            Msg::Stop => {
                self.running = false;
                rid::post(Post::Stopped(req_id));
            }
            Msg::Reset => {
                let current_elapsed = format!("Elapsed: {}", self.elapsed_secs);
                self.elapsed_secs = 0;
                rid::post(Post::Reset(req_id, current_elapsed));
            }
        }
    }

    fn start(&mut self) {
        self.running = true;
        thread::spawn(move || {
            while store::read().running {
                store::write().elapsed_secs += 1;
                rid::post(Post::Tick);
                thread::sleep(time::Duration::from_secs(1));
            }
        });
    }
}

// -----------------
// Msg
// -----------------
#[rid::message(Store)]
pub enum Msg {
    Start,
    Stop,
    Reset,
}

// -----------------
// Response
// -----------------
#[repr(C)]
enum Post {
    Started(u64),
    Stopped(u64),
    Reset(u64, String),
    Tick,
}

// -----------------
// Response helper function that will be generated
// -----------------
const POST_PAYLOAD_SEP: &str = "^"; // defined in rid::common::constants
impl ::allo_isolate::IntoDart for Post {
    fn into_dart(self) -> ::allo_isolate::ffi::DartCObject {
        let (base, data): (i64, String) = match self {
            Post::Started(id) => (rid::_encode_with_id(0, id), "".into()),
            Post::Stopped(id) => (rid::_encode_with_id(1, id), "".into()),
            Post::Reset(id, s) => (rid::_encode_with_id(2, id), s.into()),
            Post::Tick => (rid::_encode_without_id(3), "".into()),
        };
        format!("{}{}{}", base, POST_PAYLOAD_SEP, data).into_dart()
    }
}
