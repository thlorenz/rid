static mut RID_ISOLATE: Option<Isolate> = None;

struct Isolate {
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

    fn isolate<'a>() -> &'a ::allo_isolate::Isolate {
        unsafe {
            &RID_ISOLATE
                .as_ref()
                .expect("Need to `init_isolate` first")
                ._isolate
        }
    }
}

/// Used by rid internally to post request results with associated topic
pub fn _encode_with_id(enum_idx: i64, id: u64) -> i64 {
    let val: i128 = (i64::MIN + (id << 16) as i64) as i128;
    let val = (val | enum_idx as i128) as i64;
    val
}

/// Used by rid internally to post request results with associated topic
pub fn _encode_without_id(enum_idx: i64) -> i64 {
    let val: i128 = i64::MIN as i128;
    let val = (val | enum_idx as i128) as i64;
    val
}

#[no_mangle]
pub extern "C" fn rid_init_isolate(port: i64) {
    // SAFETY: called once from the main dart thread
    unsafe {
        // Flutter hot reload may cause this to get called again.
        // TODO(thlorenz): just ignoring causes dart app to get disconnected, i.e. posts don't go
        // through after hot restart anymore.
        // May be able to solve this via `reassemble`: https://stackoverflow.com/a/55282550/97443
        if RID_ISOLATE.is_none() {
            RID_ISOLATE = Some(Isolate::new(port));
        }
    }
}

pub fn post(msg: impl ::allo_isolate::IntoDart) {
    Isolate::isolate().post(msg);
}
