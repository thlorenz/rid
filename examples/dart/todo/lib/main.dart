import 'generated/rid_generated.dart';
import 'dart:io';

const LOG_VERBOSE = true;

messages() {
  final model = rid_ffi.init_model_ptr();
  model.msgAddTodo("Hello");
  model.msgAddTodo("World");

  print(model.debug(LOG_VERBOSE));

  model.msgCompleteTodo(1);
  print(model.debug(LOG_VERBOSE));

  model.msgRestartTodo(1);
  print(model.debug(LOG_VERBOSE));
}

interactive() {
  final model = rid_ffi.init_model_ptr();
  for (int i = 0; i < 100; i++) {
    stdin.readLineSync();
    model.msgAddTodo("todo_$i");
  }
}

main() {
  messages();
}
