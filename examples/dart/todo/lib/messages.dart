import 'generated/rid_api.dart';
import 'dart:async';

import 'log.dart';

messages() async {
  RID_DEBUG_REPLY = (reply) => log.d('$reply');

  final store = Store.instance;
  await store.msgAddTodo("Hello");
  await store.msgAddTodo("World");
  await store.msgAddTodo("Hola");
  await store.msgAddTodo("Mundo");

  log.v(store.debug(LOG_VERBOSE));

  await store.msgCompleteTodo(1);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgRestartTodo(1);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgRemoveTodo(1);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgToggleTodo(2);
  await store.msgToggleTodo(3);
  log.v(store.debug(LOG_VERBOSE));

  await store.msgSetFilter(Filter.Completed);
  log.v(store.debug(LOG_VERBOSE));

  final filteredTodos = store.filteredTodos();
  log.i("len: ${filteredTodos.length}");

  final firstFiltered = filteredTodos[0];
  log.v(firstFiltered.toString());
  final secondFiltered = filteredTodos[1];
  log.v(secondFiltered.toString());

  await store.msgRemoveCompleted();
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
