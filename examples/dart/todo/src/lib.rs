use rid_derive::Rid;

#[derive(Debug, Rid)]
pub struct Model {
    id: u8,
    ids: Vec<u8>,
}
#[derive(Debug, Rid)]
pub struct Todo {
    id: u8,
    completed: bool,
}

#[no_mangle]
pub extern "C" fn init_model_ptr(id: u8) -> *const Model {
    let model = Model {
        id,
        ids: vec![id, id + 1, id + 2],
    };
    Box::into_raw(Box::new(model))
}
