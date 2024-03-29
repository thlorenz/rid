import 'dart:async';

import 'package:clock/generated/rid_api.dart';
import 'package:clock/keyboard_handler.dart';
import 'package:clock/stop_watch.dart';

Future<void> main() async {
  final store = Store.instance;
  final stopWatch = StopWatch(store);
  final handler = new KeyboardHandler(store, stopWatch);
  handler.start();
  print("Waiting for the response, but life goes on\n");
}
