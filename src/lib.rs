// -----------------
// Sub Crates
// -----------------
extern crate rid_ffi;
extern crate rid_macro;
pub use rid_ffi::{
    post, RidVec, _encode_with_id, _encode_without_id, _option_ref_to_pointer,
    allo_isolate as _allo_isolate,
};
pub use rid_macro::*;

// -----------------
// Modules
// -----------------
mod traits;
pub use traits::RidStore;
