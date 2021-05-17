import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'package:clock/isolate_binding.dart';

// TODO: error handling
// TODO: rename to ResponseChannel?
class StreamChannel<T> {
  final _zone = Zone.current;
  final StreamController<T> _sink;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;

  StreamChannel._() : _sink = StreamController.broadcast() {
    _receivePort =
        RawReceivePort(_onReceivedResponse, 'rid::stream_channel::port');
    rid_ffi.init_isolate(_receivePort.sendPort.nativePort);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedResponse(T response) {
    _zone.runUnary(_zonedAdd, response);
  }

  void _add(T item) {
    if (!_sink.isClosed) {
      _sink.add(item);
    }
  }

  Stream<T> get stream => _sink.stream;

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  // TODO: do we need this? We should never dispose this singleton
  void _dispose() {
    _receivePort.close();
    _sink.close();
  }

  // TODO: for now we're assuming our messages are strings but that may change
  static StreamChannel<String>? _instance;
  static StreamChannel<String> get instance {
    if (_instance == null) {
      store_dart_post_cobject(NativeApi.postCObject);
      _instance = StreamChannel<String>._();
    }
    return _instance!;
  }
}
