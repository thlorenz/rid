#![allow(dead_code)]
#[macro_use]
extern crate log;

use std::fmt::Display;

// -----------------
// Main Model
// -----------------
#[rid::model]
#[rid::structs(Todo)]
#[rid::enums(Filter)]
#[derive(Debug)]
pub struct Model {
    last_added_id: u32,
    todos: Vec<Todo>,
    filter: Filter,
}

#[rid::export]
impl Model {
    #[rid::export(initModel)]
    fn new() -> Self {
        Self {
            last_added_id: 0,
            todos: vec![],
            filter: Filter::All,
        }
    }

    fn update(&mut self, msg: Msg) {
        info!("Msg: {:?}", msg);
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
                    None => {
                        return warn!("Could not find Todo with id '{}'", id)
                    }
                };
                self.todos.swap_remove(idx);
            }

            RemoveCompleted => self.todos.retain(|todo| !todo.completed),

            CompleteTodo(id) => {
                self.update_todo(id, |todo| todo.completed = true)
            }
            RestartTodo(id) => {
                self.update_todo(id, |todo| todo.completed = false)
            }
            ToggleTodo(id) => {
                self.update_todo(id, |todo| todo.completed = !todo.completed)
            }

            CompleteAll => {
                self.todos.iter_mut().for_each(|x| x.completed = true)
            }
            RestartAll => {
                self.todos.iter_mut().for_each(|x| x.completed = false)
            }

            SetFilter(filter) => self.filter = filter,
        };
    }

    fn update_todo<F: FnOnce(&mut Todo)>(&mut self, id: u32, update: F) {
        match self.todos.iter_mut().find(|x| x.id == id) {
            Some(todo) => update(todo),
            None => warn!("Could not find Todo with id '{}'", id),
        };
    }

    #[rid::export]
    #[rid::structs(Todo)]
    fn filtered_todos(&self) -> Vec<&Todo> {
        let mut vec: Vec<&Todo> = match self.filter {
            Filter::Completed => {
                self.todos.iter().filter(|x| x.completed).collect()
            }
            Filter::Pending => {
                self.todos.iter().filter(|x| !x.completed).collect()
            }
            Filter::All => self.todos.iter().collect(),
        };
        vec.sort();
        vec
    }
}

// -----------------
// Todo Model
// -----------------
#[rid::model]
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

// -----------------
// Filter
// -----------------
#[derive(Clone, Debug, rid::Display)]
#[repr(C)]
pub enum Filter {
    Completed,
    Pending,
    All,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::Completed => write!(f, "Completed"),
            Filter::Pending => write!(f, "Pending"),
            Filter::All => write!(f, "All"),
        }
    }
}

// -----------------
// Msg
// -----------------
#[rid::message(Model)]
#[rid::enums(Filter)]
#[derive(Debug)]
pub enum Msg {
    AddTodo(String),
    RemoveTodo(u32),
    RemoveCompleted,

    CompleteTodo(u32),
    RestartTodo(u32),
    ToggleTodo(u32),
    CompleteAll,
    RestartAll,

    SetFilter(Filter),
}
