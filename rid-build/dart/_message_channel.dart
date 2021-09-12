import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import '_isolate_binding.dart' show initIsolate;

const String MSG_SEPARATOR = '^';

enum RidMsgType { Severe, Error, LogWarn, LogInfo, LogDebug }

RidMsgType _ridMsgTypeFromString(String s) {
  switch (s.toLowerCase()) {
    case "err_severe":
      return RidMsgType.Severe;
    case "err_error":
      return RidMsgType.Error;
    case "log_warn":
      return RidMsgType.LogWarn;
    case "log_info":
      return RidMsgType.LogInfo;
    case "log_debug":
      return RidMsgType.LogDebug;
    default:
      throw ArgumentError.value(s);
  }
}

final _REMOVE_QUOTE_RX = RegExp(r'(^"|"$)');

class RidMsg {
  final RidMsgType type;
  late final String message;
  late final String? details;

  RidMsg._(this.type, String message, String? details) {
    this.message = message.replaceAll(_REMOVE_QUOTE_RX, '');
    this.details = details?.replaceAll(_REMOVE_QUOTE_RX, '');
  }

  @override
  String toString() {
    final detailsString = details == null ? '' : ', details: "$details"';
    return 'RidMsg{ type: $type, message: "$message"$detailsString }';
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RidMsg &&
          runtimeType == other.runtimeType &&
          type == other.type &&
          message == other.message &&
          details == other.details;

  @override
  int get hashCode => type.hashCode ^ message.hashCode ^ details.hashCode;
}

class RidMsgChannel {
  final _zone = Zone.current;
  final StreamController<RidMsg> _sink;
  final DynamicLibrary _dl;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;

  RidMsgChannel._(this._dl, bool isDebugMode)
      : _sink = StreamController.broadcast() {
    _receivePort =
        RawReceivePort(_onReceivedMsg, 'rid::messaging_channel::port');
    initIsolate(this._dl, 'rid_init_msg_isolate',
        _receivePort.sendPort.nativePort, isDebugMode);
    _zonedAdd = _zone.registerUnaryCallback(_add);
  }

  void _onReceivedMsg(String reply) {
    _zone.runUnary(_zonedAdd, reply);
  }

  void _add(String reply) {
    if (!_sink.isClosed) {
      _sink.add(_decode(reply));
    }
  }

  RidMsg _decode(String data) {
    int sepIdx = data.indexOf(MSG_SEPARATOR);
    final type = data.substring(0, sepIdx);
    final msgType = _ridMsgTypeFromString(type);

    final msg = data.substring(sepIdx + 1);
    sepIdx = msg.indexOf(MSG_SEPARATOR);
    if (sepIdx < 0) {
      // No details
      return RidMsg._(msgType, msg, null);
    } else {
      final message = msg.substring(0, sepIdx);
      final details = msg.substring(sepIdx + 1);
      return RidMsg._(msgType, message, details);
    }
  }

  Stream<RidMsg> get stream => _sink.stream;

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  Future<void> dispose() {
    _receivePort.close();
    return _sink.close();
  }

  static bool _initialized = false;
  static RidMsgChannel instance(
    DynamicLibrary dl,
    bool isDebugMode,
  ) {
    if (_initialized && !isDebugMode) {
      throw Exception(
          "The message channel can only be initialized once unless running in debug mode");
    }
    _initialized = true;
    return RidMsgChannel._(dl, isDebugMode);
  }
}
