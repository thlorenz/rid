//
// Exporting Native Library to call Rust functions directly
//
final dart_ffi.DynamicLibrary _dl = _open();

//
// Expose a rid instance which initializes and provides access to various features facilitating Dart/Rust interaction
//
class Rid {
  final RidMsgChannel _messageChannel;
  Duration? msgTimeout;

  Rid._(dart_ffi.DynamicLibrary dl, bool isDebugMode)
      : _messageChannel = RidMsgChannel.instance(dl, isDebugMode),
        msgTimeout = const Duration(milliseconds: 200);

  RidMsgChannel get messageChannel => _messageChannel;
}

final rid = Rid._(_dl, _isDebugMode);

// Dart evaluates code lazily and won't initialize some parts in time for Rust to
// properly use it. Therefore when rid_ffi is accessed we enforce initialization of everything
// it might need like the message channel by forcing evaluation of the Rid constructor.
ffigen_bind.NativeLibrary _initRidFFI() {
  // ignore: unnecessary_null_comparison
  if (rid == null) {}
  return ffigen_bind.NativeLibrary(_dl);
}

final rid_ffi = _initRidFFI();
