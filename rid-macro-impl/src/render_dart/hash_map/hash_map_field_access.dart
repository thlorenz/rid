/// extension Rid_HashMap_ExtOn{hash_map_type} on {pointer_hash_map_type} {
///   int get length => {rid_ffi}.rid_export_{fn_len_ident}(this);
///
///   bool contains({resolved_dart_key_type} key) =>
///       {rid_ffi}.rid_export_{fn_contains_key_ident}(this, {key_ffi_arg}) != 0;
///
///   {resolved_dart_val_type}? get({resolved_dart_key_type} key) {
///     final ptr = {rid_ffi}.rid_export_{fn_get_ident}(this, {key_ffi_arg});
///     return ptr.address == 0x0 ? null : ptr{val_to_dart};
///   }
///   {dart_collection}.HashMap<{resolved_dart_key_type}, {resolved_dart_val_type}> toDart({bool autoDispose = true}) {
///     ridStoreLock();
///     final hashMap = new {dart_collection}.HashMap<{resolved_dart_key_type}, {resolved_dart_val_type}>();
///
///     final keys = {rid_ffi}.rid_export_{fn_keys_ident}(this);
///     for (final key in keys.iter()) {
///       hashMap[key] = this.get(key)!;
///     }
///     keys.dispose();
///     ridStoreUnlock();
///     return hashMap;
///   }
/// }
