static mut RID_MSG_ISOLATE: Option<MsgIsolate> = None;

// NOTE: Mostly duplicated from ./post.rs, but hard to avoid since each is using
// a separate global.
struct MsgIsolate {
    _port: i64,
    _isolate: ::allo_isolate::Isolate,
}

impl MsgIsolate {
    fn new(port: i64) -> Self {
        let isolate = ::allo_isolate::Isolate::new(port);
        Self {
            _port: port,
            _isolate: isolate,
        }
    }

    #[allow(unused)]
    fn get<'a>() -> &'a Self {
        unsafe {
            RID_MSG_ISOLATE
                .as_ref()
                .expect("Need to `init_msg_isolate` first")
        }
    }

    #[allow(unused)]
    fn port() -> i64 {
        unsafe {
            RID_MSG_ISOLATE
                .as_ref()
                .expect("Need to `init_isolate` first")
                ._port
        }
    }

    #[allow(unused)]
    fn isolate<'a>() -> &'a ::allo_isolate::Isolate {
        unsafe {
            &RID_MSG_ISOLATE
                .as_ref()
                .expect("Need to `init_msg_isolate` first")
                ._isolate
        }
    }
}

pub fn _init_msg_isolate(port: i64) {
    unsafe {
        if RID_MSG_ISOLATE.is_some() {
            // It is very likely that the old isolate is leaking, but this is acceptable to support
            // hot restart while debugging the application
            eprintln!(
                "[rid] WARN: reinitializing internal message isolate. OK when hot reloading."
            );
        }
        RID_MSG_ISOLATE = Some(MsgIsolate::new(port));
    }
}

pub fn _post_message(msg: impl ::allo_isolate::IntoDart) {
    MsgIsolate::isolate().post(msg);
}

// -----------------
// Log warn/info/debug
// -----------------
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        let res = format!("log_warn^{}", res);
        rid::_post_message(res);
    }}
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        let res = format!("log_info^{}", res);
        rid::_post_message(res);
    }}
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        let res = format!("log_debug^{}", res);
        rid::_post_message(res);
    }}
}

// -----------------
// Err error/severe
// -----------------
#[macro_export]
macro_rules! error {
    ($msg:expr) => {{
        let msg_res = format!("err_error^{:?}", $msg);
        rid::_post_message(msg_res);
    }};
    ($msg:expr, $details:expr) => {{
        let msg_res = format!("err_error^{:?}^{:?}", $msg, $details);
        rid::_post_message(msg_res);
    }};
}

#[macro_export]
macro_rules! severe {
    ($msg:expr) => {{
        let msg_res = format!("err_severe^{:?}", $msg);
        rid::_post_message(msg_res);
    }};
    ($msg:expr, $details:expr) => {{
        let msg_res = format!("err_severe^{:?}^{:?}", $msg, $details);
        rid::_post_message(msg_res);
    }};
}
