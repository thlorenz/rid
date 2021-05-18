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
