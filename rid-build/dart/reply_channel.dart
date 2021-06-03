import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'isolate_binding.dart' show initIsolate;

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

  ReplyChannel._(this._dl, this._decode)
      : _sink = StreamController.broadcast() {
    _receivePort = RawReceivePort(_onReceivedReply, 'rid::reply_channel::port');
    initIsolate(this._dl, _receivePort.sendPort.nativePort);
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
    return stream.where((res) => res.reqId == reqId).first;
  }

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  int get reqId {
    _lastReqId++;
    return _lastReqId;
  }

  static bool _initialized = false;
  static ReplyChannel<TReply> instance<TReply extends IReply>(
      DynamicLibrary dl, Decode<TReply> decode) {
    assert(!_initialized, 'Can only initialize one ReplyChannel once');
    _initialized = true;
    return ReplyChannel<TReply>._(dl, decode);
  }
}
