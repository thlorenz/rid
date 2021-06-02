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
            while store::read().running {
                store::write().elapsed_secs += 1;
                rid::post(Post::Tick);
                thread::sleep(time::Duration::from_secs(1));
            }
        });
    }
}

// -----------------
// #[rid::message(Store)] causes below to be generated
// -----------------
pub mod store {
    use super::*;

    /// cbindgen:ignore
    static mut STORE_LOCK: Option<::std::sync::RwLock<Store>> = None;
    /// cbindgen:ignore
    static mut STORE_ACCESS: Option<StoreAccess> = None;
    /// cbindgen:ignore
    static INIT_STORE: ::std::sync::Once = ::std::sync::Once::new();
    /// cbindgen:ignore
    static mut LOCK_READ_GUARD: Option<
        ::std::sync::RwLockReadGuard<'static, Store>,
    > = None;

    pub struct StoreAccess {
        lock: &'static ::std::sync::RwLock<Store>,
    }

    impl StoreAccess {
        fn instance() -> &'static StoreAccess {
            unsafe {
                INIT_STORE.call_once(|| {
                    STORE_LOCK =
                        Some(::std::sync::RwLock::new(Store::create_store()));
                    STORE_ACCESS = Some(StoreAccess {
                        lock: STORE_LOCK.as_ref().unwrap(),
                    });
                });
                STORE_ACCESS.as_ref().unwrap()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn createStore() -> *const Store {
        let store = StoreAccess::instance().lock.read().unwrap();
        &*store as *const Store
    }

    #[no_mangle]
    pub extern "C" fn rid_store_lock() {
        if unsafe { LOCK_READ_GUARD.is_some() } {
            eprintln!("WARN trying to lock already locked store");
        } else {
            unsafe {
                LOCK_READ_GUARD = Some(store::read());
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn rid_store_unlock() {
        if unsafe { LOCK_READ_GUARD.is_none() } {
            eprintln!("WARN trying to unlock already unlocked store");
        } else {
            unsafe {
                LOCK_READ_GUARD = None;
            }
        }
    }

    pub fn read() -> ::std::sync::RwLockReadGuard<'static, Store> {
        StoreAccess::instance().lock.read().unwrap()
    }

    pub fn write() -> ::std::sync::RwLockWriteGuard<'static, Store> {
        StoreAccess::instance().lock.write().unwrap()
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
    Reset(u64),
    Tick,
}

// -----------------
// Response helper function that will be generated
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
