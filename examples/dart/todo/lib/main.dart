import 'generated/rid_generated.dart';

messages() {
  final model = rid_ffi.init_model_ptr();
  model.msgAddTodo(1);
  model.msgAddTodo(4);

  final allTodos = model.todos.iter().map((x) => x.id);
  print("""
todos len:  ${model.todos.length}
todos:      [ ${allTodos.join(', ')} ]
""");
}

main() {
  messages();
}
