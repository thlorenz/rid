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

// -----------------
// Impl Method Exports
// -----------------
#[rid::export]
impl Store {
    // -----------------
    // Exporting owned primitives
    // -----------------

    // unsigned
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

    // signed
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

    // bool
    #[rid::export]
    pub fn ok_owned(&self) -> bool {
        self.ok
    }

    #[rid::export]
    pub fn not_ok_owned(&self) -> bool {
        self.not_ok
    }

    // -----------------
    // Exporting primitives refs
    // -----------------

    // unsigned
    #[rid::export]
    pub fn s_id_ref(&self) -> &u8 {
        &self.s_id
    }
    #[rid::export]
    pub fn m_id_ref(&self) -> &u16 {
        &self.m_id
    }

    #[rid::export]
    pub fn l_id_ref(&self) -> &u32 {
        &self.l_id
    }

    #[rid::export]
    pub fn xl_id_ref(&self) -> &u64 {
        &self.xl_id
    }

    // signed
    #[rid::export]
    pub fn s_signed_ref(&self) -> &i8 {
        &self.s_signed
    }

    #[rid::export]
    pub fn m_signed_ref(&self) -> &i16 {
        &self.m_signed
    }

    #[rid::export]
    pub fn l_signed_ref(&self) -> &i32 {
        &self.l_signed
    }

    #[rid::export]
    pub fn xl_signed_ref(&self) -> &i64 {
        &self.xl_signed
    }

    // bool
    #[rid::export]
    pub fn ok_ref(&self) -> &bool {
        &self.ok
    }

    #[rid::export]
    pub fn not_ok_ref(&self) -> &bool {
        &self.not_ok
    }
}

// -----------------
// Function Exports
// -----------------

// -----------------
// Owned Primitives
// -----------------

// unsigned
#[rid::export]
pub fn fn_s_id_owned() -> u8 {
    8
}

#[rid::export]
pub fn fn_m_id_owned() -> u16 {
    16
}

#[rid::export]
pub fn fn_l_id_owned() -> u32 {
    32
}

#[rid::export]
pub fn fn_xl_id_owned() -> u64 {
    64
}

// signed
#[rid::export]
pub fn fn_s_signed_owned() -> i8 {
    -8
}

#[rid::export]
pub fn fn_m_signed_owned() -> i16 {
    -16
}

#[rid::export]
pub fn fn_l_signed_owned() -> i32 {
    -32
}

#[rid::export]
pub fn fn_xl_signed_owned() -> i64 {
    -64
}

// bool
#[rid::export]
pub fn fn_ok_owned() -> bool {
    true
}

#[rid::export]
pub fn fn_not_ok_owned() -> bool {
    false
}

// -----------------
// Refs
// -----------------
// TODO: get hold of something to reference, i.e. a struct and implement those tests

#[rid::message(Reply)]
pub enum Msg {
    NotUsed,
}
#[rid::reply]
pub enum Reply {
    NotUsed,
}
