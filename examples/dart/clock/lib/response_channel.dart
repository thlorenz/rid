import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'package:clock/isolate_binding.dart' show initIsolate;

abstract class IResponse {
  int get id;
}

// TODO: error handling (could be part of Post data)
class ResponseChannel<TResponse extends IResponse> {
  final _zone = Zone.current;
  final StreamController<TResponse> _sink;
  final TResponse Function(int packed) _decode;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;
  int _lastReqId = 0;

  ResponseChannel._(this._decode) : _sink = StreamController.broadcast() {
    _receivePort =
        RawReceivePort(_onReceivedResponse, 'rid::response_channel::port');
    initIsolate(_receivePort.sendPort.nativePort);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedResponse(int response) {
    _zone.runUnary(_zonedAdd, response);
  }

  void _add(int response) {
    if (!_sink.isClosed) {
      _sink.add(_decode(response));
    }
  }

  Stream<TResponse> get stream => _sink.stream;
  Future<TResponse> response(int reqID) {
    assert(reqID != 0, "Invalid requestID ");
    return stream.where((res) => res.id == reqID).first;
  }

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  int get reqId {
    _lastReqId++;
    return _lastReqId;
  }

  static bool _initialized = false;
  static ResponseChannel<TResponse> instance<TResponse extends IResponse>(
      TResponse Function(int packed) decode) {
    assert(!_initialized, 'Can only initialize one ResponseChannel once');
    _initialized = true;
    return ResponseChannel<TResponse>._(decode);
  }
}
