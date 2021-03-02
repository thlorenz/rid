use rid_derive::Rid;

#[derive(Rid)]
pub struct Model {
    ids: Vec<u8>,
    todos: Vec<Todo>,
}

#[derive(Debug, Rid)]
pub struct Todo {
    id: u8,
}

#[no_mangle]
pub extern "C" fn init_model_ptr() -> *const Model {
    let todo1 = Todo { id: 1 };
    let todo2 = Todo { id: 2 };
    let todo3 = Todo { id: 3 };
    let model = Model {
        ids: vec![1, 2, 3, 4],
        todos: vec![todo1, todo2, todo3],
    };
    Box::into_raw(Box::new(model))
}
