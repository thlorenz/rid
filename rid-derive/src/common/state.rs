use std::collections::HashSet;

pub struct ExpandState {
    initialized: bool,
    emitted_implementations: Option<HashSet<String>>,
}

impl ExpandState {
    fn init(&mut self) {
        if !self.initialized {
            self.initialized = true;
            self.emitted_implementations = Some(HashSet::new());
        }
    }

    pub fn needs_implementation(&mut self, implementation: &str) -> bool {
        // We can just unwrap here since the only way anyone accesses the private STATE is via
        // get_state which ensures that the state is initialized
        if self
            .emitted_implementations
            .as_ref()
            .unwrap()
            .contains(implementation)
        {
            false
        } else {
            self.emitted_implementations
                .as_mut()
                .unwrap()
                .insert(implementation.to_string());
            true
        }
    }
}

static mut STATE: ExpandState = ExpandState {
    initialized: false,
    emitted_implementations: None,
};

pub fn get_state() -> &'static mut ExpandState {
    // SAFETY: for now we assume that rust expansion doesn't work in parallel, otherwise we'd need
    // to make this thread safe (see Arc)
    unsafe {
        STATE.init();
        &mut STATE
    }
}
