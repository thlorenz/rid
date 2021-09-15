// -----------------
// Sub Crates
// -----------------
extern crate rid_ffi;
extern crate rid_macro;
pub use rid_ffi::{
    post, RidVec, _encode_with_id, _encode_without_id, _init_msg_isolate,
    _init_reply_isolate, _option_ref_to_pointer, _post_message,
    allo_isolate as _allo_isolate, error, log_debug, log_info, log_warn,
    severe,
};
pub use rid_macro::*;

// -----------------
// Modules
// -----------------
mod traits;
pub use traits::RidStore;
