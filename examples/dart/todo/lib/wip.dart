import 'generated/ffigen_binding.dart';
import 'generated/rid_generated.dart';
import 'dart:io';
import 'dart:async';

import 'log.dart';

printFiltered(Pointer<Model> model) {
  final filtered = model.filter_todos();
  final list = filtered.iter().toList();
  filtered.dispose();
  log.i("\n${list.first.debug(true)}");
}

main() {
  final model = rid_ffi.init_model_ptr();
  log.i("\n${model.debug(true)}");
  printFiltered(model);
  printFiltered(model);
  rid_ffi.free_model_ptr(model);
}
