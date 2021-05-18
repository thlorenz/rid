use std::{thread, time};

// -----------------
// Model
// -----------------
#[rid::model]
#[derive(Debug, rid::Debug)]
pub struct Model {
    running: bool,
    elapsed_secs: u32,
}

#[rid::export]
impl Model {
    #[rid::export(initModel)]
    pub fn new() -> Self {
        Self {
            running: false,
            elapsed_secs: 0,
        }
    }

    // TODO: multiple methods can get a hold of the mutable model via different
    // FFI entry points. Assignments/reads are not synced and thus this code is not
    // thread safe.
    // Here we are dealing with primitive values, but modifying vecs like this would
    // be a problem.
    // Messages all would come from the Dart UI thread and thus never run in parallel,
    // however we're creating threads for each which could race.
    // Also dedicated threads which tick like the below pose a problem.
    // We may need to look into a proper runtime or similar ...
    fn start(&'static mut self) {
        thread::spawn(move || {
            self.running = true;
            while self.running {
                thread::sleep(time::Duration::from_secs(1));
                self.elapsed_secs += 1;
                rid::post(Topic::Tick);
            }
        });
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn reset(&mut self) {
        self.elapsed_secs = 0;
    }

    fn handle_start(&'static mut self, req_id: u64) {
        self.start();
        rid::post(Topic::Started(req_id));
    }

    fn handle_stop(&mut self, req_id: u64) {
        self.stop();
        rid::post(Topic::Stopped(req_id));
    }

    fn handle_reset(&mut self, req_id: u64) {
        self.reset();
        rid::post(Topic::Reset(req_id));
    }
}

// -----------------
// Response
// -----------------

// Variants with an id are responses for Dart initiated messages.
//   - that id is returned to Dart synchronously when the message is sent
//   - a subscription for that topic:id is then made via the StreamChannel in
//     order to wait for a response from Rust
// Variants without and id are events generated from inside Rust.
//   - subscriptions are only for the topic since ids would have no meaning
//     on the Dart side
//
// An Error(id) variant could be used to transmit errors that occur when handling
// a message. The StreamChannel would subscribe to that and someone pass it along
// to subscribers.
#[repr(C)]
#[allow(unused)]
enum Topic {
    Started(u64),
    Stopped(u64),
    Reset(u64),
    Tick,
}

fn encode_with_id(topic: i64, id: u64) -> i64 {
    let val: i128 = (i64::MIN + (id << 16) as i64) as i128;
    let val = (val | topic as i128) as i64;
    val
}
fn encode_without_id(topic: i64) -> i64 {
    let val: i128 = i64::MIN as i128;
    let val = (val | topic as i128) as i64;
    val
}

impl ::allo_isolate::IntoDart for Topic {
    fn into_dart(self) -> ::allo_isolate::ffi::DartCObject {
        let val = match self {
            Topic::Started(id) => encode_with_id(0, id),
            Topic::Stopped(id) => encode_with_id(1, id),
            Topic::Reset(id) => encode_with_id(2, id),
            Topic::Tick => encode_without_id(3),
        };
        ::allo_isolate::ffi::DartCObject {
            ty: ::allo_isolate::ffi::DartCObjectType::DartInt64,
            value: ::allo_isolate::ffi::DartCObjectValue { as_int64: val },
        }
    }
}

#[no_mangle]
pub extern "C" fn msgStart(req_id: u64, ptr: *mut Model) {
    let model = unsafe {
        if !!ptr.is_null() {
            panic!("assertion failed: !ptr.is_null()")
        };
        let ptr: *mut Model = &mut *ptr;
        ptr.as_mut().unwrap()
    };
    model.handle_start(req_id);
}

#[no_mangle]
pub extern "C" fn msgStop(req_id: u64, ptr: *mut Model) {
    let model = unsafe {
        if !!ptr.is_null() {
            panic!("assertion failed: !ptr.is_null()")
        };
        let ptr: *mut Model = &mut *ptr;
        ptr.as_mut().unwrap()
    };
    model.handle_stop(req_id);
}

#[no_mangle]
pub extern "C" fn msgReset(req_id: u64, ptr: *mut Model) {
    let model = unsafe {
        if !!ptr.is_null() {
            panic!("assertion failed: !ptr.is_null()")
        };
        let ptr: *mut Model = &mut *ptr;
        ptr.as_mut().unwrap()
    };
    model.handle_reset(req_id);
}
