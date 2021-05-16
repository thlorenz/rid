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
    static ref RUNTIME: ::std::io::Result<Runtime> = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .thread_name("flutterust")
        .build();
}

pub async fn load_page_impl(url: &str) -> Result<String, TestError> {
    Ok(format!("loaded {}", url))
}

// -----------------
// load_page ffi wrapper
// -----------------
#[no_mangle]
pub extern "C" fn load_page(
    port: i64,
    url: *const ::std::os::raw::c_char,
) -> i32 {
    let runtime = match RUNTIME.as_ref() {
        Ok(x) => x,
        Err(err) => {
            eprintln!("Err: {:?}", err);
            return 0;
        }
    };
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };

    let t = Isolate::new(port).task(load_page_impl(url));
    runtime.spawn(t);
    1
}
