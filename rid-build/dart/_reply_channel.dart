import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import '_isolate_binding.dart' show initIsolate;

const String RESPONSE_SEPARATOR = '^';

abstract class IReply {
  int? get reqId;
  String? get data;
}

typedef Decode<TReply> = TReply Function(int packedBase, String? data);

// TODO: error handling (could be part of Post data)
class ReplyChannel<TReply extends IReply> {
  final _zone = Zone.current;
  final StreamController<TReply> _sink;
  final Decode<TReply> _decode;
  final DynamicLibrary _dl;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;
  int _lastReqId = 0;

  ReplyChannel._(this._dl, this._decode, bool isDebugMode)
      : _sink = StreamController.broadcast() {
    _receivePort = RawReceivePort(_onReceivedReply, 'rid::reply_channel::port');
    initIsolate(this._dl, _receivePort.sendPort.nativePort, isDebugMode);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedReply(String reply) {
    _zone.runUnary(_zonedAdd, reply);
  }

  void _add(String reply) {
    if (!_sink.isClosed) {
      final sepIdx = reply.indexOf(RESPONSE_SEPARATOR);
      final base = int.parse(reply.substring(0, sepIdx));
      final String? data =
          reply.length > sepIdx ? reply.substring(sepIdx + 1) : null;
      _sink.add(_decode(base, data));
    }
  }

  Stream<TReply> get stream => _sink.stream;
  Future<TReply> reply(int reqId) {
    assert(reqId != 0, "Invalid requestID ");
    return stream
        .firstWhere((res) => res.reqId == reqId)
        .onError((error, stackTrace) {
      print(
          "The responseChannel was disposed while a message was waiting for a reply.\n"
          "Did you forget to `await` the reply to the message with reqId: '$reqId'?\n"
          "Ignore the message further down about type 'Null'.\n"
          "The real problem is that no reply for the message was posted yet, but the reply \n"
          "stream is being disposed most likely via `store.dispose()` causing the following:.\n");
      print(error);
      print(stackTrace);
      return null as TReply;
    });
  }

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  int get reqId {
    _lastReqId++;
    return _lastReqId;
  }

  Future<void> dispose() {
    _receivePort.close();
    return _sink.close();
  }

  static bool _initialized = false;
  static ReplyChannel<TReply> instance<TReply extends IReply>(
    DynamicLibrary dl,
    Decode<TReply> decode,
    bool isDebugMode,
  ) {
    if (_initialized && !isDebugMode) {
      throw Exception(
          "The reply channel can only be initialized once unless running in debug mode");
    }
    _initialized = true;
    return ReplyChannel<TReply>._(dl, decode, isDebugMode);
  }
}
