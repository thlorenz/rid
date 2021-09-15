import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import '_isolate_binding.dart' show initIsolate;

const String _MSG_SEPARATOR = '^';

enum RidMessageType { Severe, Error, LogWarn, LogInfo, LogDebug }

RidMessageType _ridMsgTypeFromString(String s) {
  switch (s.toLowerCase()) {
    case "err_severe":
      return RidMessageType.Severe;
    case "err_error":
      return RidMessageType.Error;
    case "log_warn":
      return RidMessageType.LogWarn;
    case "log_info":
      return RidMessageType.LogInfo;
    case "log_debug":
      return RidMessageType.LogDebug;
    default:
      throw ArgumentError.value(s);
  }
}

final _REMOVE_QUOTE_RX = RegExp(r'(^"|"$)');

class RidMessage {
  final RidMessageType type;
  late final String message;
  late final String? details;

  RidMessage._(this.type, String message, String? details) {
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
      other is RidMessage &&
          runtimeType == other.runtimeType &&
          type == other.type &&
          message == other.message &&
          details == other.details;

  @override
  int get hashCode => type.hashCode ^ message.hashCode ^ details.hashCode;
}

abstract class RidMessageChannel {
  Stream<RidMessage> get stream;
}

class RidMessageChannelInternal implements RidMessageChannel {
  final _zone = Zone.current;
  final StreamController<RidMessage> _sink;
  final DynamicLibrary _dl;
  late final RawReceivePort _receivePort;
  late final _zonedAdd;

  RidMessageChannelInternal._(this._dl, bool isDebugMode)
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

  RidMessage _decode(String data) {
    int sepIdx = data.indexOf(_MSG_SEPARATOR);
    final type = data.substring(0, sepIdx);
    final msgType = _ridMsgTypeFromString(type);

    final msg = data.substring(sepIdx + 1);
    sepIdx = msg.indexOf(_MSG_SEPARATOR);
    if (sepIdx < 0) {
      // No details
      return RidMessage._(msgType, msg, null);
    } else {
      final message = msg.substring(0, sepIdx);
      final details = msg.substring(sepIdx + 1);
      return RidMessage._(msgType, message, details);
    }
  }

  Stream<RidMessage> get stream => _sink.stream;

  int get nativePort {
    return _receivePort.sendPort.nativePort;
  }

  Future<void> dispose() {
    _receivePort.close();
    return _sink.close();
  }

  static bool _initialized = false;
  static RidMessageChannelInternal instance(
    DynamicLibrary dl,
    bool isDebugMode,
  ) {
    if (_initialized && !isDebugMode) {
      throw Exception(
          "The message channel can only be initialized once unless running in debug mode");
    }
    _initialized = true;
    return RidMessageChannelInternal._(dl, isDebugMode);
  }
}
