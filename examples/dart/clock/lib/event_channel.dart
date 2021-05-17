import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import 'package:clock/isolate_binding.dart';

class EventChannel<TEvent> {
  final List<ZoneUnaryCallback<void, TEvent>> _subscriptions =
      List.empty(growable: true);

  final _zone = Zone.current;

  late final RawReceivePort receivePort;

  EventChannel() {
    receivePort = RawReceivePort(_onReceivedResponse);
  }

  void subscribe(void Function(TEvent) cb) {
    final zoned = _zone.registerUnaryCallback(cb);
    _subscriptions.add(zoned);
  }

  void _onReceivedResponse(TEvent response) {
    for (final zoned in _subscriptions) {
      _zone.runUnary(zoned, response);
    }
  }

  SendPort get sendPort {
    return receivePort.sendPort;
  }

  int get nativePort {
    return sendPort.nativePort;
  }

  static setup() {
    store_dart_post_cobject(NativeApi.postCObject);
  }
}
