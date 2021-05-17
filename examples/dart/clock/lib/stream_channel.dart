import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'package:clock/isolate_binding.dart';

// TODO: error handling
class StreamChannel<T> {
  final _zone = Zone.current;
  final StreamController<T> _sink;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;

  StreamChannel() : _sink = StreamController.broadcast() {
    _receivePort = RawReceivePort(_onReceivedResponse);
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

  void trythings() {}

  void dispose() {
    _receivePort.close();
    _sink.close();
  }

  static setup() {
    store_dart_post_cobject(NativeApi.postCObject);
  }
}
