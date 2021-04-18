use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use quote::format_ident;
use syn::Ident;

pub struct ExpandState {
    initialized: bool,
    emitted_implementations: Option<HashSet<String>>,
    emitted_idents: Option<HashMap<Ident, u8>>,
}

impl ExpandState {
    fn init(&mut self) {
        if !self.initialized {
            self.initialized = true;
            self.emitted_implementations = Some(HashSet::new());
            self.emitted_idents = Some(HashMap::new());
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

    pub fn unique_ident(&mut self, ident: Ident) -> Ident {
        let idents = self.emitted_idents.as_mut().unwrap();
        let count: u8 = idents.get(&ident).unwrap_or(&0_u8) + 1;
        let id = format_ident!("{}_{}", ident, count);
        idents.insert(ident, count);
        id
    }

    pub fn need_implemtation<K: Display, V>(
        &mut self,
        all: HashMap<K, V>,
    ) -> Vec<V> {
        all.into_iter()
            .filter_map(|(k, v)| {
                if self.needs_implementation(&k.to_string()) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }
}

static mut STATE: ExpandState = ExpandState {
    initialized: false,
    emitted_implementations: None,
    emitted_idents: None,
};

pub fn get_state() -> &'static mut ExpandState {
    // SAFETY: for now we assume that rust expansion doesn't work in parallel, otherwise we'd need
    // to make this thread safe (see Arc)
    unsafe {
        STATE.init();
        &mut STATE
    }
}
