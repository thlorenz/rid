import 'generated/rid_generated.dart';
import 'dart:io';

const LOG_VERBOSE = true;

messages() {
  final model = rid_ffi.init_model_ptr();
  model.msgAddTodo("Hello");
  model.msgAddTodo("World");
  model.msgAddTodo("Hola");
  model.msgAddTodo("Mundo");

  print(model.debug(LOG_VERBOSE));

  print("completing 1");
  model.msgCompleteTodo(1);
  print(model.debug(LOG_VERBOSE));

  print("restarting 1");
  model.msgRestartTodo(1);
  print(model.debug(LOG_VERBOSE));

  print("removing 1");
  model.msgRemoveTodo(1);
  print(model.debug(LOG_VERBOSE));

  print("toggling 2 and 3");
  model.msgToggleTodo(2);
  model.msgToggleTodo(3);
  print(model.debug(LOG_VERBOSE));

  print("removing completed");
  model.msgRemoveCompleted();
  print(model.debug(LOG_VERBOSE));

  print("completing all");
  model.msgCompleteAll();
  print(model.debug(LOG_VERBOSE));

  print("restarting all");
  model.msgRestartAll();
  print(model.debug(LOG_VERBOSE));

  // Does not exist and logs warning from rust
  model.msgRestartTodo(5);
}

interactive() {
  final model = rid_ffi.init_model_ptr();
  for (int i = 0; i < 100; i++) {
    stdin.readLineSync;
    model.msgAddTodo("todo_$i");
  }
}

main() {
  messages();
}
