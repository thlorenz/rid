/// extension Rid_Vec_ExtOn{vec_type} on {ffigen_bind}.{vec_type} {
///   {dart_raw_item_type} operator [](int idx) {
///     final len = this.length;
///     if (!(0 <= idx && idx < len)) {
///       throw AssertionError("Out of range access on List<{dart_raw_item_type}>[$idx] of length $len");
///     }
///     return {rid_ffi}.{fn_get_ident}(this, idx);
///   }
///
///   /// **WARNING**: You cannot use this Vec pointer anymore after this call
///   /// completes unless you set [autoDispose] to [false].
///   ///
///   /// Converts this Vec pointer into a Dart [List&lt;{dart_item_type}&gt;] and disposes the
///   /// underlying Rust Vec unless [autoDispose] is set to [false].
///   /// As a result if [autoDispose] is [true] you cannot use the underlying
///   /// Vec anymore after this call completes.
///   List<{dart_item_type}> toDart({bool autoDispose = true}) {
///     ridStoreLock();
///     final list = this.iter(){map_to_dart}.toList();
///     if (autoDispose) dispose();
///     ridStoreUnlock();
///     return list;
///   }
///   void dispose() {
///     {rid_ffi}.{fn_free_ident}(this);
///   }
///
///   Rid_{vec_type}_Iterable iter() => Rid_{vec_type}_Iterable(this);
/// }
/// 
/// class Rid_{vec_type}_Iterator implements Iterator<{dart_raw_item_type}> {
///   int _currentIdx = -1;
///   final ffigen_bind.{vec_type} _vec;
///   final int _limit;
/// 
///   Rid_{vec_type}_Iterator(this._vec) : _limit = _vec.length - 1;
/// 
///   {dart_raw_item_type} get current => _vec[_currentIdx];
/// 
///   bool moveNext() {
///     if (_currentIdx >= _limit) return false;
///     _currentIdx++;
///     return true;
///   }
/// }
/// 
/// class Rid_{vec_type}_Iterable with
///     {dart_collection}.IterableMixin<{dart_raw_item_type}> {
///   final ffigen_bind.{vec_type} _vec;
///   Rid_{vec_type}_Iterable(this._vec);
/// 
///   Iterator<{dart_raw_item_type}> get iterator =>
///     Rid_{vec_type}_Iterator(this._vec);
/// }
