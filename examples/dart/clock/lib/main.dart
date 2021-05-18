import 'dart:async';

import 'package:clock/generated/rid_generated.dart';
import 'package:clock/keyboard_handler.dart';
import 'package:clock/stop_watch.dart';

Future<void> main() async {
  final model = rid_ffi.initModel();
  final stopWatch = StopWatch(model);
  final handler = new KeyboardHandler(model, stopWatch);
  handler.start();
  print("Waiting for the response, but life goes on\n");
}
