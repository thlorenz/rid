#[derive(Debug, rid::Model)]
#[rid(debug)]
pub struct Model {
    id: u8,
}
impl Model {
    fn update(&mut self, _: Msg) {}
}

#[derive(rid::Message)]
#[rid(to = "Model")]
pub enum Msg {
    Any,
}

/* impl Model {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::SetFilter(filter) => self.filter = filter,
        };
    }
} */
/*
#[derive(rid::Message)]
pub enum Msg {
    SetFilter(Filter),
}

#[repr(C)]
pub enum Filter {
    Completed,
    Pending,
    All,
} */
