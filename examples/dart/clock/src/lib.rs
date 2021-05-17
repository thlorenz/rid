use allo_isolate::Isolate;
use std::{error::Error, ffi::CStr, fmt};

use lazy_static::lazy_static;
use tokio::runtime::{Builder, Runtime};

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
    for _ in 1..=20_000_000 {
        c += 1;
    }
    Ok(format!("loaded {} after {} iterations", url, c))
}

// -----------------
// load_page ffi wrapper
// -----------------
#[no_mangle]
pub extern "C" fn load_page(
    port: i64,
    url: *const ::std::os::raw::c_char,
) -> i32 {
    let runtime: &Runtime = match RUNTIME.as_ref() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Err: {:?}", err);
            return 0;
        }
    };
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };

    let isolate = Isolate::new(port);
    let task = isolate.task(load_page_impl(url));
    runtime.spawn(task);
    isolate.post("hello world");
    isolate.post("hola  mundo");

    1
}
