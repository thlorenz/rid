import 'generated/ffigen_binding.dart';
import 'generated/rid_generated.dart';
import 'dart:async';

import 'log.dart';

messages() async {
  final store = rid_ffi.createStore();
  await store.msgAddTodo("Hello");
  await store.msgAddTodo("World");
  await store.msgAddTodo("Hola");
  final lastAddRes = await store.msgAddTodo("Mundo");
  log.v(lastAddRes);

  log.v(store.debug(LOG_VERBOSE));

  await store.msgCompleteTodo(1);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgRestartTodo(1);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgRemoveTodo(1);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgToggleTodo(2);
  final lastToggleRes = await store.msgToggleTodo(3);
  log.d(lastToggleRes);
  log.v(store.debug(LOG_VERBOSE));

  final filterRes = await store.msgSetFilter(Filter.Completed);
  log.d(filterRes);
  log.v(store.debug(LOG_VERBOSE));

  final filteredTodos = store.filtered_todos();
  log.i("len: ${filteredTodos.length}, cap: ${filteredTodos.capacity}");

  final firstFiltered = filteredTodos[0];
  log.v(firstFiltered.debug(LOG_VERBOSE));
  final secondFiltered = filteredTodos[1];
  log.v(secondFiltered.debug(LOG_VERBOSE));
  filteredTodos.dispose();

  final completedRes = await store.msgRemoveCompleted();
  log.d(completedRes);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgCompleteAll();
  log.v(store.debug(LOG_VERBOSE));

  await store.msgRestartAll();
  log.v(store.debug(LOG_VERBOSE));

  log.d("restarting non-existent todo");
  await store.msgRestartTodo(5);

  store.dispose();
}

onError(Object error, StackTrace stack) {
  print(error);
  print(stack);
}

void main(List<String> args) async {
  await runZonedGuarded(messages, onError);
  log.i("App run completed successfully");
}
