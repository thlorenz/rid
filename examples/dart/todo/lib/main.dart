import 'generated/ffigen_binding.dart';
import 'generated/rid_generated.dart';
import 'dart:async';

import 'log.dart';

onError(Object error, StackTrace stack) {
  print(error);
  print(stack);
}

void run() {
  final model = rid_ffi.init_model_ptr();
  log.v(model.debug());
  model.msgSetFilter(Filter.Pending);
  log.v(model.debug());
}

void main(List<String> args) {
  runZonedGuarded(run, onError);
  log.i("App run completed successfully");
}
