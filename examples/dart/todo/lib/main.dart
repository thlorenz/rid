import 'generated/ffigen_binding.dart' as ffigen_bind;
import 'generated/rid_generated.dart' as rid;
import 'dart:async';

import 'log.dart';

enum Filter {
  Completed,
  Pending,
  All,
}
Filter fromFfiFilter(int idx) {
  return Filter.values[idx];
}

onError(Object error, StackTrace stack) {
  print(error);
  print(stack);
}

void run() {
  final model = rid.rid_ffi.init_model_ptr();
  log.v(model.debug());
  model.msgSetFilter(ffigen_bind.Filter.Pending);
  log.v(model.debug());
  log.v("Filter: ${fromFfiFilter(model.filter)}");
}

void main(List<String> args) {
  runZonedGuarded(run, onError);
  log.i("App run completed successfully");
}
