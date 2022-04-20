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

        #[cfg(test)]
        eprintln!("WARN: {}", res);
        #[cfg(not(test))]
        rid::_post_message(format!("log_warn^{}", res));
    }}
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);

        #[cfg(test)]
        eprintln!("INFO: {}", res);
        #[cfg(not(test))]
        rid::_post_message(format!("log_info^{}", res));
    }}
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);

        #[cfg(test)]
        eprintln!("DEBG: {}", res);
        #[cfg(not(test))]
        rid::_post_message(format!("log_debug^{}", res));
    }}
}
// -----------------
// Err error/severe
// -----------------
#[macro_export]
macro_rules! error {
    ($msg:expr) => {{
        #[cfg(test)]
        eprintln!("ERR:  {:?}", $msg);
        #[cfg(not(test))]
        rid::_post_message(format!("err_error^{:?}", $msg));
    }};
    ($msg:expr, $details:expr) => {{
        #[cfg(test)]
        eprintln!("ERR:  {:?} {:?}", $msg, $details);
        #[cfg(not(test))]
        rid::_post_message(format!("err_error^{:?}^{:?}", $msg, $details));
    }};
}

#[macro_export]
macro_rules! severe {
    ($msg:expr) => {{
        #[cfg(test)]
        eprintln!("ERR!: {:?}", $msg);
        #[cfg(not(test))]
        rid::_post_message(format!("err_severe^{:?}", $msg));
    }};
    ($msg:expr, $details:expr) => {{
        #[cfg(test)]
        eprintln!("ERR!: {:?} {:?}", $msg, $details);
        #[cfg(not(test))]
        rid::_post_message(format!("err_severe^{:?}^{:?}", $msg, $details));
    }};
}

// -----------------
// User Messages
// -----------------
#[macro_export]
macro_rules! msg_warn {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);

        #[cfg(test)]
        eprintln!("MSGW: {}", res);
        #[cfg(not(test))]
        rid::_post_message(format!("msg_warn^{}", res));
    }}
}

#[macro_export]
macro_rules! msg_info {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);

        #[cfg(test)]
        eprintln!("MSGI: {}", res);
        #[cfg(not(test))]
        rid::_post_message(format!("msg_info^{}", res));
    }}
}
