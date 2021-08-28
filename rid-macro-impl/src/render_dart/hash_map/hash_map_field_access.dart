/// extension Rid_HashMap_ExtOn{hash_map_type} on {pointer_hash_map_type} {
///   int get length => {rid_ffi}.{fn_len_ident}(this);
///
///   bool contains({key_type} key) =>
///       {rid_ffi}.{fn_contains_key_ident}(this, key) != 0;
///
///   {val_return_type}? get({key_type} key) {
///     final ptr = {rid_ffi}.{fn_get_ident}(this, key);
///     return ptr.address == 0x0 ? null : ptr.value;
///   }
///   {dart_collection}.HashMap<{key_type}, {val_type}> toDart({bool autoDispose = true}) {
///     ridStoreLock();
///     final hashMap = new {dart_collection}.HashMap<{key_type}, {val_type}>();
///
///     final keys = {rid_ffi}.{fn_keys_ident}(this);
///     for (final key in keys.iter()) {
///       hashMap[key] = this.get(key)!;
///     }
///     ridStoreUnlock();
///     return hashMap;
///   }
/// }