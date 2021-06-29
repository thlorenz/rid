use crate::Reply;

/// cbindgen:ignore
static mut REPLIES_LOCK: Option<::std::sync::RwLock<Vec<ReplyStruct>>> = None;
/// cbindgen:ignore
static mut REPLIES_ACCESS: Option<RidRepliesAccess> = None;
/// cbindgen:ignore
static INIT_REPLIES: ::std::sync::Once = ::std::sync::Once::new();

struct RidRepliesAccess {
    lock: &'static ::std::sync::RwLock<Vec<ReplyStruct>>,
}

impl RidRepliesAccess {
    fn instance() -> &'static RidRepliesAccess {
        unsafe {
            INIT_REPLIES.call_once(|| {
                REPLIES_LOCK = Some(::std::sync::RwLock::new(vec![]));
                REPLIES_ACCESS = Some(RidRepliesAccess {
                    lock: REPLIES_LOCK.as_ref().unwrap(),
                });
            });
            REPLIES_ACCESS.as_ref().unwrap()
        }
    }
}

// -----------------
// API used by rid internally when we poll replies for wasm support
// -----------------
pub fn post(reply: Reply) {
    replies_write().push(reply.into())
}

pub fn replies_read() -> ::std::sync::RwLockReadGuard<'static, Vec<ReplyStruct>>
{
    RidRepliesAccess::instance().lock.read().unwrap()
}

pub fn replies_write(
) -> ::std::sync::RwLockWriteGuard<'static, Vec<ReplyStruct>> {
    RidRepliesAccess::instance().lock.write().unwrap()
}

#[no_mangle]
pub extern "C" fn rid_poll_reply() -> *const ReplyStruct {
    rid::_option_ref_to_pointer(replies_read().iter().next())
}

#[no_mangle]
pub extern "C" fn rid_handled_reply(req_id: u64) {
    replies_write().retain(|x| x.req_id != req_id)
}

#[rid::model]
#[derive(Debug, Clone)]
pub struct ReplyStruct {
    ty: u8,
    req_id: u64,
}

impl From<Reply> for ReplyStruct {
    fn from(reply: Reply) -> Self {
        match reply {
            Reply::Inced(req_id) => Self { ty: 0, req_id },
            Reply::Dumped(req_id) => Self { ty: 1, req_id },
        }
    }
}
