pub trait RidStore<TMsg> {
    fn create() -> Self;
    fn update(&mut self, req_id: u64, msg: TMsg);
}
