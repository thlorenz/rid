import 'dart:async';

import 'package:clock/keyboard_handler.dart';
import 'package:clock/stop_watch.dart';

Future<void> main() async {
  final stopWatch = StopWatch();
  final handler = new KeyboardHandler(stopWatch);
  handler.start();
  print("Waiting for the response, but life goes on\n");
}
