#[rid::model]
#[derive(Debug)]
pub struct Model {
    last_added_id: u32,
    todos: Vec<Todo>,
}

#[rid::model]
#[derive(Debug)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[rid::message(to = "Model")]
pub enum Msg {
    AddTodo(String),
    RemoveTodo(u32),
    RemoveCompleted,

    CompleteTodo(u32),
    RestartTodo(u32),
    ToggleTodo(u32),
    CompleteAll,
    RestartAll,
}

impl Model {
    fn update(&mut self, msg: Msg) {
        use Msg::*;
        match msg {
            AddTodo(title) => {
                self.last_added_id += 1;
                let todo = Todo {
                    id: self.last_added_id,
                    title,
                    completed: false,
                };
                self.todos.push(todo);
            }
            RemoveTodo(id) => {
                let mut enumerated = self.todos.iter().enumerate();
                let idx = match enumerated.find(|(_, todo)| todo.id == id) {
                    Some((idx, _)) => idx,
                    None => return eprintln!("Could not find Todo with id '{}'", id),
                };
                self.todos.swap_remove(idx);
            }

            RemoveCompleted => self.todos.retain(|todo| !todo.completed),

            CompleteTodo(id) => self.update_todo(id, |todo| todo.completed = true),
            RestartTodo(id) => self.update_todo(id, |todo| todo.completed = false),
            ToggleTodo(id) => self.update_todo(id, |todo| todo.completed = !todo.completed),

            CompleteAll => self.todos.iter_mut().for_each(|x| x.completed = true),
            RestartAll => self.todos.iter_mut().for_each(|x| x.completed = false),
        };
    }

    fn update_todo<F: FnOnce(&mut Todo)>(&mut self, id: u32, update: F) {
        match self.todos.iter_mut().find(|x| x.id == id) {
            Some(todo) => update(todo),
            None => eprintln!("Could not find Todo with id '{}'", id),
        };
    }
}

#[no_mangle]
pub extern "C" fn init_model_ptr() -> *const Model {
    let model = Model {
        last_added_id: 0,
        todos: vec![],
    };
    Box::into_raw(Box::new(model))
}
