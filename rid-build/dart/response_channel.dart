import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'isolate_binding.dart' show initIsolate;

const String RESPONSE_SEPARATOR = '^';

abstract class IResponse {
  int get id;
  String? get data;
}

typedef Decode<TResponse> = TResponse Function(int packedBase, String? data);

// TODO: error handling (could be part of Post data)
class ResponseChannel<TResponse extends IResponse> {
  final _zone = Zone.current;
  final StreamController<TResponse> _sink;
  final Decode<TResponse> _decode;
  final DynamicLibrary _dl;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;
  int _lastReqId = 0;

  ResponseChannel._(this._dl, this._decode)
      : _sink = StreamController.broadcast() {
    _receivePort =
        RawReceivePort(_onReceivedResponse, 'rid::response_channel::port');
    initIsolate(this._dl, _receivePort.sendPort.nativePort);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedResponse(String response) {
    _zone.runUnary(_zonedAdd, response);
  }

  void _add(String response) {
    if (!_sink.isClosed) {
      final sepIdx = response.indexOf(RESPONSE_SEPARATOR);
      final base = int.parse(response.substring(0, sepIdx));
      final String? data =
          response.length > sepIdx ? response.substring(sepIdx + 1) : null;
      _sink.add(_decode(base, data));
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
      DynamicLibrary dl, Decode<TResponse> decode) {
    assert(!_initialized, 'Can only initialize one ResponseChannel once');
    _initialized = true;
    return ResponseChannel<TResponse>._(dl, decode);
  }
}
