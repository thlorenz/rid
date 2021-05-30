use std::{thread, time};

// -----------------
// Store
// -----------------

// #[rid::store]
#[rid::model]
#[derive(Debug, rid::Debug)]
pub struct Store {
    running: bool,
    elapsed_secs: u32,
}

impl Store {
    pub fn create_store() -> Self {
        Self {
            running: false,
            elapsed_secs: 0,
        }
    }

    pub fn update(&mut self, req_id: u64, msg: Msg) {
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
                self.elapsed_secs = 0;
                rid::post(Post::Reset(req_id));
            }
        }
    }

    fn start(&mut self) {
        self.running = true;
        thread::spawn(move || {
            while store::state().running {
                thread::sleep(time::Duration::from_secs(1));
                store::state().elapsed_secs += 1;
                rid::post(Post::Tick);
            }
        });
    }
}

// -----------------
// #[rid::store] attr will cause the below to be generated
// -----------------
pub mod store {
    use super::*;
    use std::sync::{Mutex, MutexGuard, Once};

    /// cbindgen:ignore
    static mut STORE_MUTEX: Option<Mutex<Store>> = None;
    /// cbindgen:ignore
    static mut STORE_ACCESS: Option<StoreAccess> = None;
    /// cbindgen:ignore
    static INIT_STORE: Once = Once::new();

    #[derive(rid::Debug)]
    pub struct StoreAccess {
        mutex: &'static Mutex<Store>,
    }

    // Generated when `rid::Debug` is present on Store along with the rid method
    // to expose it to Dart
    impl std::fmt::Debug for StoreAccess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let state = &*state();
            state.fmt(f)
        }
    }

    impl StoreAccess {
        fn instance() -> &'static StoreAccess {
            unsafe {
                INIT_STORE.call_once(|| {
                    STORE_MUTEX = Some(Mutex::new(Store::create_store()));
                    STORE_ACCESS = Some(StoreAccess {
                        mutex: &STORE_MUTEX.as_ref().unwrap(),
                    });
                });
                STORE_ACCESS.as_ref().unwrap()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn createStore() -> *const StoreAccess {
        StoreAccess::instance()
    }

    pub fn state() -> MutexGuard<'static, Store> {
        StoreAccess::instance().mutex.lock().unwrap()
    }

    // -----------------
    // Message handling
    // -----------------
    #[no_mangle]
    pub extern "C" fn msgStart(req_id: u64) {
        state().update(req_id, Msg::Start);
    }

    #[no_mangle]
    pub extern "C" fn msgStop(req_id: u64) {
        state().update(req_id, Msg::Stop);
    }

    #[no_mangle]
    pub extern "C" fn msgReset(req_id: u64) {
        state().update(req_id, Msg::Reset);
    }
}

// -----------------
// Msg
// -----------------
#[repr(C)]
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
    Reset(u64),
    Tick,
}

// -----------------
// Topic helper function that will be generated
// -----------------
impl ::allo_isolate::IntoDart for Post {
    fn into_dart(self) -> ::allo_isolate::ffi::DartCObject {
        let val = match self {
            Post::Started(id) => rid::_encode_with_id(0, id),
            Post::Stopped(id) => rid::_encode_with_id(1, id),
            Post::Reset(id) => rid::_encode_with_id(2, id),
            Post::Tick => rid::_encode_without_id(3),
        };
        ::allo_isolate::ffi::DartCObject {
            ty: ::allo_isolate::ffi::DartCObjectType::DartInt64,
            value: ::allo_isolate::ffi::DartCObjectValue { as_int64: val },
        }
    }
}
