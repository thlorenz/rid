import 'generated/ffigen_binding.dart';
import 'generated/rid_generated.dart';
import 'dart:io';
import 'dart:async';

import 'log.dart';

messages() {
  final model = rid_ffi.init_model_ptr();
  model.msgAddTodo("Hello");
  model.msgAddTodo("World");
  model.msgAddTodo("Hola");
  model.msgAddTodo("Mundo");

  log.v(model.debug(LOG_VERBOSE));

  model.msgCompleteTodo(1);
  log.v(model.debug(LOG_VERBOSE));

  model.msgRestartTodo(1);
  log.v(model.debug(LOG_VERBOSE));

  model.msgRemoveTodo(1);
  log.v(model.debug(LOG_VERBOSE));

  model.msgToggleTodo(2);
  model.msgToggleTodo(3);
  log.v(model.debug(LOG_VERBOSE));

  model.msgSetFilter(Filter.Completed);
  log.v(model.debug(LOG_VERBOSE));

  for (final filtered in model.filtered_todos.iter()) {
    log.v("filtered: ${filtered.debug(LOG_VERBOSE)}");
  }

  model.msgRemoveCompleted();
  log.v(model.debug(LOG_VERBOSE));

  model.msgCompleteAll();
  log.v(model.debug(LOG_VERBOSE));

  model.msgRestartAll();
  log.v(model.debug(LOG_VERBOSE));

  log.d("restarting non-existent todo");
  model.msgRestartTodo(5);

  rid_ffi.free_model_ptr(model);
}

interactive() {
  final model = rid_ffi.init_model_ptr();
  for (int i = 0; i < 100; i++) {
    stdin.readLineSync;
    model.msgAddTodo("todo_$i");
  }
}

onError(Object error, StackTrace stack) {
  print(error);
  print(stack);
}

void main(List<String> args) {
  runZonedGuarded(messages, onError);
  log.i("App run completed successfully");
}