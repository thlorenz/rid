/// Stubbing the store lock method until a store is implemented via
/// `#[rid::message(Store, Reply)]`
void ridStoreLock({String? request}) {}

/// Stubbing the store unlock method until a store is implemented via
/// `#[rid::message(Store, Reply)]`
void ridStoreUnlock() {}
