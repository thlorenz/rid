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
// Isolate
// -----------------
static mut RID_ISOLATE: Option<Isolate> = None;

pub struct Isolate {
    _port: i64,
    _isolate: ::allo_isolate::Isolate,
}

impl Isolate {
    fn new(port: i64) -> Self {
        let isolate = ::allo_isolate::Isolate::new(port);
        Self {
            _port: port,
            _isolate: isolate,
        }
    }

    #[allow(unused)]
    fn get<'a>() -> &'a Self {
        unsafe { RID_ISOLATE.as_ref().expect("Need to `init_isolate` first") }
    }

    #[allow(unused)]
    fn port() -> i64 {
        unsafe {
            RID_ISOLATE
                .as_ref()
                .expect("Need to `init_isolate` first")
                ._port
        }
    }

    pub fn isolate<'a>() -> &'a ::allo_isolate::Isolate {
        unsafe {
            &RID_ISOLATE
                .as_ref()
                .expect("Need to `init_isolate` first")
                ._isolate
        }
    }

    pub fn post(msg: impl ::allo_isolate::IntoDart) {
        Isolate::isolate().post(msg);
    }
}

#[no_mangle]
pub extern "C" fn init_isolate(port: i64) {
    // SAFETY: called once from the main dart thread
    unsafe {
        if RID_ISOLATE.is_some() {
            // TODO: could also just ignore
            panic!("Isolate already initialized, only do this once!");
        }
        RID_ISOLATE = Some(Isolate::new(port));
    }
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

// -----------------
// load_page ffi wrapper
// -----------------
#[no_mangle]
pub extern "C" fn load_page(url: *const ::std::os::raw::c_char) -> i32 {
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
    Isolate::post("hello world");
    Isolate::post("hola  mundo");

    1
}
