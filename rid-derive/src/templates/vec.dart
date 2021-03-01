/// extension Rid_ExtOnPointer{vec_type} on {dart_ffi}.Pointer<{ffigen_bind}.{vec_type}> {
///   int get length => {rid_ffi}.{fn_len_ident}(this);
///   int operator [](int idx) {
///     final len = this.length;
///     if (!(0 <= idx && idx < len)) {
///       throw AssertionError("Out of range access [$idx] on List of length $len");
///     }
///     return {rid_ffi}.{fn_get_ident}(this, idx);
///   }
/// }
