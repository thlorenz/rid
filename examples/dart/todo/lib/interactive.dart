import 'generated/ffigen_binding.dart';
import 'generated/rid_generated.dart';
import 'dart:io';
import 'dart:async';

onError(Object error, StackTrace stack) {
  print(error);
  print(stack);
}

// TODO: generate this from the rust type as well as a converter to rust enum
// via an extension
enum DartFilter { Completed, Pending, All }

// TODO: extract that list and free in generated code
List<Pointer<Todo>> filteredTodos(Pointer<Model> model) {
  final matching = rid_ffi.filtered_todos(model);
  final List<Pointer<Todo>> list = [];
  for (int i = 0; i < matching.len; i++) {
    final todo = rid_ffi.todos_get_idx(matching, i);
    list.add(todo);
  }
  return list;
}

// TODO: this could be .display()
String prettyStringTodo(Pointer<Todo> todo) {
  final status = todo.completed ? "✓" : " ";
  return "[$status] (${todo.id}) '${todo.title}'";
}

printStatus(Pointer<Model> model) {
  final todos = model.todos;
  final total = todos.length;
  final filter = DartFilter.values[model.filter].toString().substring(11);
  final matchingTodos = filteredTodos(model);

  print("Total Todos:     $total");
  print("Filter:          $filter");
  print("\nMatching Todos:");
  for (final todo in matchingTodos) {
    print("    ${prettyStringTodo(todo)}");
  }
}

bool handleCommand(Pointer<Model> model, String line) {
  String cmd;
  String payload;

  if (line.length > 2) {
    cmd = line.substring(0, 3);
    payload = line.substring(3).trim();
  } else {
    cmd = line.substring(0, 2);
    payload = "";
  }

  switch (cmd) {
    case "add":
      model.msgAddTodo(payload);
      break;
    case "del":
      model.msgRemoveTodo(int.parse(payload));
      break;
    case "cmp":
      model.msgCompleteTodo(int.parse(payload));
      break;
    case "tog":
      model.msgToggleTodo(int.parse(payload));
      break;
    case "rst":
      model.msgRestartTodo(int.parse(payload));
      break;
    case "fil":
      final filter = payload == "cmp"
          ? Filter.Completed
          : payload == "pen"
              ? Filter.Pending
              : Filter.All;
      model.msgSetFilter(filter);
      break;
    case "ca":
      model.msgCompleteAll();
      break;
    case "dc":
      model.msgRemoveCompleted();
      break;
    case "ra":
      model.msgRestartAll();
      break;

    default:
      print("\nUnknown command '$cmd'\n");
      return false;
  }
  return true;
}

printCommands() {
  print("\nPlease select one of the below:\n");
  print("  add <todo title>  -- to add a todo");
  print("  del <todo id>     -- to delete a todo by id");
  print("  cmp <todo id>     -- to complete a todo by id");
  print("  rst <todo id>     -- to restart a todo by id");
  print("  tog <todo id>     -- to toggle a todo by id");
  print("  fil all|cmp|pen   -- to set filter to");
  print("  ca                -- to completed all todos");
  print("  dc                -- to delete completed todos");
  print("  ra                -- to restart all todos");
  print("  q                 -- to quit");
}

interactive() {
  final model = rid_ffi.init_model_ptr();
  {
    model.msgAddTodo("Complete this Todo via:    cmp 1");
    model.msgAddTodo("Delete this Todo via:      del 2");
    model.msgAddTodo("Toggle this Todo via:      tog 3");
    model.msgAddTodo("Restart the first Tod via: rst 1");

    String? input;
    bool ok = true;

    while (true) {
      if (ok) {
        print("\x1B[2J\x1B[0;0H");
      }
      printStatus(model);
      printCommands();
      stdout.write("\n> ");
      input = stdin.readLineSync();
      if (input == "q") {
        break;
      }
      if (input != null && input.length > 1) {
        ok = handleCommand(model, input.trim());
      }
    }
  }
  rid_ffi.free_model_ptr(model);
}

void main(List<String> args) {
  runZonedGuarded(interactive, onError);
}
