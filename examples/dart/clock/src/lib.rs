use std::{error::Error, fmt};

use isolate::Isolate;
use lazy_static::lazy_static;
use tokio::runtime::{Builder, Runtime};

mod isolate;

// -----------------
// Test Error
// -----------------
#[derive(Copy, Clone, Debug)]
pub struct TestError;
impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while testing.")
    }
}
impl Error for TestError {}

// -----------------
// Tokio Runtime
// -----------------
lazy_static! {
    static ref RUNTIME: ::std::io::Result<Runtime> =
        Builder::new_multi_thread()
            .thread_name("rid_isolate")
            .worker_threads(2)
            .build();
}

// -----------------
// Load Page Impl
// -----------------
pub async fn load_page_impl(url: &str) -> Result<String, TestError> {
    let mut c = 0;
    for _ in 1..=100_000_000 {
        c += 1;
    }
    Ok(format!("loaded {} after {} iterations", url, c))
}

//
// Reponse Mapping
//
#[repr(C)]
#[allow(unused)]
enum Topic {
    Hello(u64),
    Loaded(u64),
}

fn encode(topic: i64, id: u64) -> i64 {
    let val: i128 = (i64::MIN + (id << 16) as i64) as i128;
    let val = (val | topic as i128) as i64;
    val
}

/* Implemented in Dart
fn decode(val: i64) -> (i64, u64) {
    let mask: i64 = 0x00_00_00_00_00_00_ff_ff;
    let n = val & mask;
    let id: i128 = (val as i128 - i64::MIN as i128) >> 16;
    (id as i64, n as u64)
}
*/

impl ::allo_isolate::IntoDart for Topic {
    fn into_dart(self) -> ::allo_isolate::ffi::DartCObject {
        let val = match self {
            Topic::Hello(id) => encode(0, id),
            Topic::Loaded(id) => encode(1, id),
        };
        ::allo_isolate::ffi::DartCObject {
            ty: ::allo_isolate::ffi::DartCObjectType::DartInt64,
            value: ::allo_isolate::ffi::DartCObjectValue { as_int64: val },
        }
    }
}

// -----------------
// load_page ffi wrapper
// -----------------
#[no_mangle]
pub extern "C" fn load_page(_url: *const ::std::os::raw::c_char) -> i32 {
    /*
    let runtime: &Runtime = match RUNTIME.as_ref() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Err: {:?}", err);
            return 0;
        }
    };
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };
    let task = Isolate::isolate().task(load_page_impl(url));
    runtime.spawn(task);
    */

    Isolate::post(Topic::Hello(1));
    Isolate::post(Topic::Hello(2));
    Isolate::post(Topic::Loaded(2222));

    1
}
