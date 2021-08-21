import 'package:test/test.dart';
import 'package:tests_apps/generated/rid_api.dart';

void main() {
  test('field_access: enums', () async {
    RID_DEBUG_LOCK = null;
    RID_DEBUG_REPLY = null;

    final store = Store.instance;
    await store.msgAddTodo("Hello");
    await store.msgAddTodo("World");
    await store.msgAddTodo("Hola");
    await store.msgAddTodo("Mundo");

    expect(store.todos.length, 4);
    expect(store.todos.first.title, "Hello");
    expect(store.todos[2].title, "Hola");

    await store.msgCompleteTodo(1);
    expect(store.todos.firstWhere((x) => x.id == 1).completed, true);

    await store.msgRestartTodo(1);
    expect(store.todos.firstWhere((x) => x.id == 1).completed, false);

    await store.msgRemoveTodo(1);
    expect(store.todos.length, 3);
    expect(store.todos.map((x) => x.id).join(", "), "2, 3, 4");

    expect(store.todos.firstWhere((x) => x.id == 2).completed, false);
    expect(store.todos.firstWhere((x) => x.id == 3).completed, false);
    await store.msgToggleTodo(2);
    await store.msgToggleTodo(3);
    expect(store.todos.firstWhere((x) => x.id == 2).completed, true);
    expect(store.todos.firstWhere((x) => x.id == 3).completed, true);

    await store.msgSetFilter(Filter.Completed);

    final filteredTodos = store.filteredTodos();
    expect(filteredTodos.map((x) => x.id).join(", "), "2, 3");

    await store.msgRemoveCompleted();
    expect(store.todos.length, 1);

    await store.msgCompleteAll();
    expect(store.todos[0].completed, true);

    await store.msgRestartAll();
    expect(store.todos[0].completed, false);

    // restarting non-existent todo does not crash
    await store.msgRestartTodo(5);

    store.dispose();
  });
}
