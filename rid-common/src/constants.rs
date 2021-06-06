// Constants used in code generated via rid-macro and bindgen as well as the higher level dart
// wrapper generated by rid-build.

/// Bindings generated by ffigen are imported into the build wrapper as this id.
pub const FFI_GEN_BIND: &str = "ffigen_bind";

/// The built in 'dart:ffi' library is imported into the build wrapper as this id.
pub const DART_FFI: &str = "dart_ffi";
///
/// The built in 'dart:collection' library is imported into the build wrapper as this id.
pub const DART_COLLECTION: &str = "dart_collection";

/// The low level wrappers for the Rust FFI functions are imported into the dart
/// wrapper as this id.
pub const RID_FFI: &str = "rid_ffi";

/// Name of extension method defined on a dart string to convert it to `Pointer<Int8>`.
pub const STRING_TO_NATIVE_INT8: &str = "toNativeInt8";

/// Method invoked to free a CString by resolving and dropping it.
pub const CSTRING_FREE: &str = "rid_cstring_free";

/// Function set to debug rid store locking
pub const RID_DEBUG_LOCK: &str = "RID_DEBUG_LOCK";

/// Function set to debug posted replies
pub const RID_DEBUG_REPLY: &str = "RID_DEBUG_REPLY";
