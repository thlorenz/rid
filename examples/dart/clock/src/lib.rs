use isolate::Isolate;
use std::thread;

mod isolate;

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

pub fn handle_start(req_id: u64) {
    thread::spawn(move || Isolate::post(Topic::Started(req_id)));
}

pub fn handle_stop(req_id: u64) {
    thread::spawn(move || Isolate::post(Topic::Stopped(req_id)));
}

pub fn handle_reset(req_id: u64) {
    thread::spawn(move || Isolate::post(Topic::Reset(req_id)));
}

#[no_mangle]
pub extern "C" fn msgStart() -> u64 {
    let req_id = Isolate::next_id();
    handle_start(req_id);
    req_id
}

#[no_mangle]
pub extern "C" fn msgStop() -> u64 {
    let req_id = Isolate::next_id();
    handle_stop(req_id);
    req_id
}

#[no_mangle]
pub extern "C" fn msgReset() -> u64 {
    let req_id = Isolate::next_id();
    handle_reset(req_id);
    req_id
}
