import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:typed_data';
import '_isolate_binding.dart' show initIsolate;

const String RESPONSE_SEPARATOR = '^';

abstract class IReply {
  int? get reqId;
  Uint8List? get data;
}

typedef Decode<TReply> = TReply Function(int packedBase, Uint8List? data);

abstract class RidReplyChannel<TReply extends IReply> {
  Stream<TReply> get stream;
}

// TODO: error handling (could be part of Post data)
class RidReplyChannelInternal<TReply extends IReply> implements RidReplyChannel<TReply> {
  final _zone = Zone.current;
  final StreamController<TReply> _sink;
  final Decode<TReply> _decode;
  final DynamicLibrary _dl;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;
  int _lastReqId = 0;

  RidReplyChannelInternal._(this._dl, this._decode, bool isDebugMode) : _sink = StreamController.broadcast() {
    _receivePort = RawReceivePort(_onReceivedReply, 'rid::reply_channel::port');
    initIsolate(this._dl, 'rid_init_reply_isolate', _receivePort.sendPort.nativePort, isDebugMode);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedReply(Uint8List reply) {
    _zone.runUnary(_zonedAdd, reply);
  }

  void _add(Uint8List reply) {
    if (!_sink.isClosed) {
      final base_byte_list = reply.sublist(0, 8);
      int base = ByteData.view(base_byte_list.buffer).getInt64(0);
      final Uint8List? data = reply.sublist(8);
      _sink.add(_decode(base, data));
    }
  }

  Stream<TReply> get stream => _sink.stream;
  Future<TReply> reply(int reqId) {
    assert(reqId != 0, "Invalid requestID ");
    return stream.firstWhere((res) => res.reqId == reqId).onError((error, stackTrace) {
      print("The responseChannel was disposed while a message was waiting for a reply.\n"
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
  static RidReplyChannelInternal<TReply> instance<TReply extends IReply>(
    DynamicLibrary dl,
    Decode<TReply> decode,
    bool isDebugMode,
  ) {
    if (_initialized && !isDebugMode) {
      throw Exception("The reply channel can only be initialized once unless running in debug mode");
    }
    _initialized = true;
    return RidReplyChannelInternal<TReply>._(dl, decode, isDebugMode);
  }
}
