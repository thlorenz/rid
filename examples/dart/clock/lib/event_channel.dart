import 'dart:ffi';
import 'dart:isolate';

import 'package:clock/isolate_binding.dart';

class EventChannel<TEvent> {
  final List<void Function(TEvent)> _subscriptions = List.empty(growable: true);
  late final RawReceivePort receivePort;

  EventChannel() {
    receivePort = RawReceivePort(_onReceivedResponse);
  }

  void subscribe(void Function(TEvent) cb) {
    _subscriptions.add(cb);
  }

  void _onReceivedResponse(TEvent response) {
    for (final cb in _subscriptions) {
      cb(response);
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
