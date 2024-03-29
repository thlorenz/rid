#[rid::model]
#[rid::structs(Todo)]
#[derive(Debug, rid::Debug)]
pub struct Model {
    todos: Vec<Todo>,
}

#[rid::model]
#[derive(Debug, rid::Debug)]
pub struct Todo {
    id: u32,
    title: String,
}

#[rid::export]
impl Model {
    #[rid::export(initModel)]
    fn new() -> Model {
        Self {
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
        }
    }

    #[rid::export]
    #[rid::structs(Todo)]
    fn filtered_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|x| x.id > 0).collect()
    }
}
