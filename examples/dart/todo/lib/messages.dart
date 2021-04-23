import 'generated/ffigen_binding.dart';
import 'generated/rid_generated.dart';
import 'dart:async';

import 'log.dart';

messages() {
  final model = rid_ffi.rid_export_Model_new();
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

  final filteredTodos = model.filtered_todos();
  log.i("len: ${filteredTodos.length}, cap: ${filteredTodos.capacity}");

  final firstFiltered = filteredTodos[0];
  log.v(firstFiltered.debug(LOG_VERBOSE));
  final secondFiltered = filteredTodos[1];
  log.v(secondFiltered.debug(LOG_VERBOSE));
  filteredTodos.dispose();

  model.msgRemoveCompleted();
  log.v(model.debug(LOG_VERBOSE));

  model.msgCompleteAll();
  log.v(model.debug(LOG_VERBOSE));

  model.msgRestartAll();
  log.v(model.debug(LOG_VERBOSE));

  log.d("restarting non-existent todo");
  model.msgRestartTodo(5);

  rid_ffi.rid_free_Model(model);
}

onError(Object error, StackTrace stack) {
  print(error);
  print(stack);
}

void main(List<String> args) {
  runZonedGuarded(messages, onError);
  log.i("App run completed successfully");
}
