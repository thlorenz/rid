import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'package:clock/isolate_binding.dart' show initIsolate;

// -----------------
// Response Type (will be generated from Rust enum)
// -----------------

enum Topic {
  Started,
  Stopped,
  Reset,
  Tick,
}

class Response {
  final Topic topic;
  final int id;

  Response(this.topic, this.id);

  @override
  String toString() {
    return '''Response {
  topic: ${this.topic.toString().substring('Topic.'.length)}
  id:    $id
}
''';
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

// -----------------
// Stream Channel
// -----------------

// TODO: error handling
class ResponseChannel {
  final _zone = Zone.current;
  final StreamController<Response> _sink;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;
  int _lastReqId = 0;

  ResponseChannel._() : _sink = StreamController.broadcast() {
    _receivePort =
        RawReceivePort(_onReceivedResponse, 'rid::stream_channel::port');
    initIsolate(_receivePort.sendPort.nativePort);
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
  Future<Response> response(int reqID) {
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

  // TODO: do we need this? We should never dispose this singleton
  void _dispose() {
    _receivePort.close();
    _sink.close();
  }

  static ResponseChannel? _instance;
  static ResponseChannel get instance {
    if (_instance == null) {
      _instance = ResponseChannel._();
    }
    return _instance!;
  }
}
