// Constants used in code generated via rid-macro and bindgen as well as the higher level dart
// wrapper generated by rid-build.

/// Bindings generated by ffigen are imported into the build wrapper as this id.
pub const FFI_GEN_BIND: &str = "ffigen_bind";

/// The built in 'dart:ffi' library is imported into the build wrapper as this id.
pub const DART_FFI: &str = "dart_ffi";

/// The built in 'dart:async' library is imported into the build wrapper as this id.
pub const DART_ASYNC: &str = "dart_async";

/// The built in 'dart:collection' library is imported into the build wrapper as this id.
pub const DART_COLLECTION: &str = "dart_collection";

/// The low level wrappers for the Rust FFI functions are imported into the dart
/// wrapper as this id.
pub const RID_FFI: &str = "rid_ffi";

/// Name of extension method defined on a dart string to convert it to `Pointer<Int8>`.
pub const STRING_TO_NATIVE_INT8: &str = "toNativeInt8";

/// Method invoked to free a CString by resolving and dropping it.
pub const CSTRING_FREE: &str = "rid_cstring_free";

pub const STRING_REF_ACCESS: &str = "rid_access_string_ref";

/// Name of the module containing all of global rid util methods like string free.
pub const UTILS_MODULE: &str = "__rid_utils_module";

/// Function set to debug rid store locking
pub const RID_DEBUG_LOCK: &str = "rid.debugLock";

/// Function set to debug posted replies
pub const RID_DEBUG_REPLY: &str = "rid.debugReply";

/// Duration set to specify default message timeout
pub const RID_MSG_TIMEOUT: &str = "rid.msgTimeout";

/// Access to reply channel user facing API
pub const RID_REPLY_CHANNEL: &str = "rid.replyChannel";

/// Access to internal reply channel API
pub const _RID_REPLY_CHANNEL: &str = "_replyChannel";

/// Dart method name to create the Rust store
pub const RID_CREATE_STORE: &str = "_createStore";

/// Name of the Rust store. The convention is to name it 'Store'.
///
/// This makes a lot of things possible or easier that otherwise weren't.
/// For instance the #[rid::message] needs not be passed the Store type.
///
/// Additionally #[rid::export] instance methods can be limited to only be present on the store
/// which is good practice and avoids memory race conditions.
///
/// These exports can then be re-exported on the higher level API, extending the Dart 'Store'
/// class as well.
pub const STORE: &str = "Store";
