#[derive(Debug, rid::Model)]
#[rid(debug)]
pub struct Model {
    id: u8,
    #[rid(types = {Filter: Enum})]
    filter: Filter,
}
impl Model {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::SetFilter(filter) => {
                println!("Applying filter {:?}", filter);
            }
        };
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Filter {
    Completed,
    Pending,
    All,
}

#[derive(rid::Message)]
#[rid(to = Model)]
pub enum Msg {
    #[rid(types = {Filter: Enum})]
    SetFilter(Filter),
}

#[no_mangle]
pub extern "C" fn init_model_ptr() -> *const Model {
    let model = Model {
        id: 1,
        filter: Filter::All,
    };
    Box::into_raw(Box::new(model))
}
