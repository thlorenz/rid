import 'generated/rid_generated.dart' as ffi;

todo_vecs() {
  final model = ffi.rid_ffi.init_model_ptr();
  print("""
todos len:  ${model.todos.length}
todos:      [ ${model.todos[0].id}, ${model.todos[1].id},  ${model.todos[2].id} ]
""");

  for (final todo in model.todos.iter()) {
    print("todo.id: ${todo.id}");
  }
  for (final id in model.ids.iter()) {
    print("id: $id");
  }
  print("crash: ${model.todos[4]}");
}

main() {
  todo_vecs();
}
