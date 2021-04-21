#[rid::model]
#[rid::structs(Todo)]
#[derive(Debug)]
pub struct Model {
    todos: Vec<Todo>,
}
#[rid::model]
#[derive(Debug)]
pub struct Todo {
    id: u32,
    title: String,
}

#[rid::export]
impl Model {
    #[rid::export]
    #[rid::structs(Todo)]
    fn filter_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|x| x.id > 0).collect()
    }
}

#[no_mangle]
pub extern "C" fn init_model_ptr() -> *const Model {
    env_logger::init();
    let model = Model {
        todos: vec![
            Todo {
                id: 0,
                title: "first todo".to_string(),
            },
            Todo {
                id: 1,
                title: "second todo".to_string(),
            },
        ],
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
    drop(model);
}
