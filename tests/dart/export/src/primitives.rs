use rid::RidStore;

#[rid::store]
pub struct Store {
    // unsigned integers
    s_id: u8,
    m_id: u16,
    l_id: u32,
    xl_id: u64,
    // signed integers
    s_signed: i8,
    m_signed: i16,
    l_signed: i32,
    xl_signed: i64,
    // bool
    ok: bool,
    not_ok: bool,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            s_id: 1,
            m_id: 10,
            l_id: 100,
            xl_id: 1000,

            s_signed: -1,
            m_signed: -10,
            l_signed: -100,
            xl_signed: -1000,

            ok: true,
            not_ok: false,
        }
    }

    fn update(&mut self, _req_id: u64, _msg: Msg) {
        unimplemented!()
    }
}

#[rid::export]
impl Store {
    // -----------------
    // Exporting owned primitives
    // -----------------
    #[rid::export]
    pub fn s_id_owned(&self) -> u8 {
        self.s_id
    }

    #[rid::export]
    pub fn m_id_owned(&self) -> u16 {
        self.m_id
    }

    #[rid::export]
    pub fn l_id_owned(&self) -> u32 {
        self.l_id
    }

    #[rid::export]
    pub fn xl_id_owned(&self) -> u64 {
        self.xl_id
    }
    #[rid::export]
    pub fn s_signed_owned(&self) -> i8 {
        self.s_signed
    }

    #[rid::export]
    pub fn m_signed_owned(&self) -> i16 {
        self.m_signed
    }

    #[rid::export]
    pub fn l_signed_owned(&self) -> i32 {
        self.l_signed
    }

    #[rid::export]
    pub fn xl_signed_owned(&self) -> i64 {
        self.xl_signed
    }

    #[rid::export]
    pub fn ok_owned(&self) -> bool {
        self.ok
    }

    #[rid::export]
    pub fn not_ok_owned(&self) -> bool {
        self.not_ok
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
