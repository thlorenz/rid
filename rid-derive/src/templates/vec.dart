/// extension Rid_ExtOnPointer{vec_type} on {dart_ffi}.Pointer<{ffigen_bind}.{vec_type}> {
///   int get length => {rid_ffi}.{fn_len_ident}(this);
///   int operator [](int idx) {
///     final len = this.length;
///     if (!(0 <= idx && idx < len)) {
///       throw AssertionError("Out of range access [$idx] on List of length $len");
///     }
///     return {rid_ffi}.{fn_get_ident}(this, idx);
///   }
///   Rid_{vec_type}_Iterable iter() => Rid_{vec_type}_Iterable(this);
/// }
/// 
/// class Rid_{vec_type}_Iterator implements Iterator<{dart_item_type}> {
///   int _currentIdx = -1;
///   final {dart_ffi}.Pointer<ffigen_bind.{vec_type}> _vec;
///   final int _limit;
/// 
///   Rid_{vec_type}_Iterator(this._vec) : _limit = _vec.length - 1;
/// 
///   int get current => _vec[_currentIdx];
/// 
///   bool moveNext() {
///     if (_currentIdx >= _limit) return false;
///     _currentIdx++;
///     return true;
///   }
/// }
/// 
/// class Rid_{vec_type}_Iterable with {dart_collection}.IterableMixin<{dart_item_type}> {
///   final {dart_ffi}.Pointer<ffigen_bind.{vec_type}> _vec;
///   Rid_Vec_u8_Iterable(this._vec);
/// 
///   Iterator<{dart_item_type}> get iterator => Rid_{vec_type}_Iterator(this._vec);
/// }
