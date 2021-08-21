use std::ffi::CString;

use rid::RidStore;

#[rid::store]
pub struct Store {
    title: String,
    ctitle: CString,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            title: "T-shirt Store".to_string(),
            ctitle: CString::new("C-shirt Store").unwrap(),
        }
    }

    fn update(&mut self, _req_id: u64, _msg: Msg) {
        unimplemented!()
    }
}

#[rid::export]
impl Store {
    // owned
    #[rid::export]
    pub fn title_owned(&self) -> String {
        self.title.to_string()
    }

    #[rid::export]
    pub fn ctitle_owned(&self) -> CString {
        self.ctitle.clone()
    }

    // references
    #[rid::export]
    pub fn title_ref(&self) -> &String {
        &self.title
    }

    #[rid::export]
    pub fn ctitle_ref(&self) -> &CString {
        &self.ctitle
    }

    #[rid::export]
    pub fn title_as_str(&self) -> &str {
        self.title.as_str()
    }
}

#[rid::message(Reply)]
pub enum Msg {
    NotUsed,
}
#[rid::reply]
pub enum Reply {
    NotUsed,
}
