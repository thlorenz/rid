import 'generated/rid_generated.dart';

main() {
  Pointer<Todo> todo = Pointer.fromAddress(1);
  print("hello ${todo.title}");
}
