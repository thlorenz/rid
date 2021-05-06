#[rid::model]
#[derive(Debug, rid::Debug)]
pub struct Model {
    count: u32,
}

#[rid::export]
impl Model {
    #[rid::export(initModel)]
    fn new() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Inc => self.count += 1,
        }
    }
}

#[rid::message(Model)]
#[derive(Debug)]
pub enum Msg {
    Inc,
}
