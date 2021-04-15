use rid::RidVec;

#[macro_use]
extern crate log;

#[derive(Debug, rid::Model)]
#[rid(debug)]
pub struct Model {
    last_added_id: u32,
    #[rid(types = { Todo: Struct })]
    todos: Vec<Todo>,
    #[rid(types = { Filter: Enum })]
    filter: Filter,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, rid::Model)]
#[rid(debug)]
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

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Filter {
    Completed,
    Pending,
    All,
}

#[derive(Debug, rid::Message)]
#[rid(to = Model)]
pub enum Msg {
    AddTodo(String),
    RemoveTodo(u32),
    RemoveCompleted,

    CompleteTodo(u32),
    RestartTodo(u32),
    ToggleTodo(u32),
    CompleteAll,
    RestartAll,

    #[rid(types = { Filter: Enum })]
    SetFilter(Filter),
}

impl Model {
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

    #[rid::export]
    fn filtered_ids(&self) -> Vec<u32> {
        self.filtered_todos().iter().map(|x| x.id).collect()
    }
}

#[no_mangle]
pub extern "C" fn init_model_ptr() -> *const Model {
    env_logger::init();
    let model = Model {
        last_added_id: 0,
        todos: vec![],
        filter: Filter::All,
    };
    Box::into_raw(Box::new(model))
}

#[no_mangle]
pub extern "C" fn free_model_ptr(ptr: *mut Model) {
    let model = unsafe {
        assert!(!ptr.is_null());
        let ptr: *mut Model = &mut *ptr;
        let ptr = ptr.as_mut().unwrap();
        Box::from_raw(ptr)
    };
    info!("Freeing model: {:#?}", model);
    drop(model);
}
