#[rid::model]
#[derive(Debug)]
pub struct Model {
    todos: Vec<Todo>,
}

impl Model {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::AddTodo(id) => {
                self.todos.push(Todo { id });
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn init_model_ptr() -> *const Model {
    let model = Model { todos: vec![] };
    Box::into_raw(Box::new(model))
}

#[rid::model]
#[derive(Debug)]
pub struct Todo {
    id: u8,
}

#[rid::message(to = "Model")]
#[derive(Debug)]
pub enum Msg {
    AddTodo(u8),
}
