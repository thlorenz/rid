import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'package:clock/isolate_binding.dart';

//
// Response Type (will be generated from Rust enum)
//

enum Topic {
  Hello,
  Loaded,
}

class Response {
  final Topic topic;
  final int id;

  Response(this.topic, this.id);

  @override
  String toString() {
    return 'topic: ${this.topic.toString()}, id: $id';
  }
}

const int _TOPIC_MASK = 0x000000000000ffff;
const int _I64_MIN = -9223372036854775808;

Response decode(int packed) {
  final ntopic = packed & _TOPIC_MASK;
  final id = (packed - _I64_MIN) >> 16;

  final topic = Topic.values[ntopic];
  return Response(topic, id);
}

//
// Channel
//

// TODO: error handling
// TODO: rename to ResponseChannel?
class StreamChannel {
  final _zone = Zone.current;
  final StreamController<Response> _sink;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;

  StreamChannel._() : _sink = StreamController.broadcast() {
    _receivePort =
        RawReceivePort(_onReceivedResponse, 'rid::stream_channel::port');
    rid_ffi.init_isolate(_receivePort.sendPort.nativePort);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedResponse(int response) {
    _zone.runUnary(_zonedAdd, response);
  }

  void _add(int response) {
    if (!_sink.isClosed) {
      _sink.add(decode(response));
    }
  }

  Stream<Response> get stream => _sink.stream;

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  // TODO: do we need this? We should never dispose this singleton
  void _dispose() {
    _receivePort.close();
    _sink.close();
  }

  // TODO: for now we're assuming our messages are strings but that may change
  static StreamChannel? _instance;
  static StreamChannel instance<T>() {
    if (_instance == null) {
      store_dart_post_cobject(NativeApi.postCObject);
      _instance = StreamChannel._();
    }
    return _instance!;
  }
}
