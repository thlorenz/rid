use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use quote::format_ident;
use syn::Ident;

pub struct ExpandState {
    initialized: bool,
    /// Implementations for supporting functions, i.e. frees or collection accesses
    /// that have been emitted
    emitted_implementations: Option<HashSet<String>>,

    /// Identifiers emitted, i.e. to name wrapping modules.
    /// Used to ensure unique identifier names.
    emitted_idents: Option<HashMap<Ident, u8>>,

    /// Function/Method exports that have been processed as part of an impl.
    /// Needed to avoid processing a method export inside an impl twice, once
    /// as part of the impl and then again separately as a function.
    handled_impl_method_exports: Option<HashSet<String>>,
}

pub enum ImplementationType {
    CollectionAccess,
    DartEnum,
    UtilsModule,
    Free,
}

impl fmt::Display for ImplementationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImplementationType::CollectionAccess => {
                write!(f, "CollectionAccess")
            }
            ImplementationType::DartEnum => {
                write!(f, "DartEnum")
            }
            ImplementationType::Free => {
                write!(f, "Free")
            }
            ImplementationType::UtilsModule => {
                write!(f, "UtilsModel")
            }
        }
    }
}

impl ExpandState {
    fn init(&mut self) {
        if !self.initialized {
            self.initialized = true;
            self.emitted_implementations = Some(HashSet::new());
            self.emitted_idents = Some(HashMap::new());
            self.handled_impl_method_exports = Some(HashSet::new());
        }
    }

    // -----------------
    // Implementations
    // -----------------
    pub fn needs_implementation(
        &mut self,
        impl_type: &ImplementationType,
        implementation: &str,
    ) -> bool {
        let key = format!("{} {}", impl_type, implementation);
        // We can just unwrap here since the only way anyone accesses the private STATE is via
        // get_state which ensures that the state is initialized
        if self
            .emitted_implementations
            .as_ref()
            .unwrap()
            .contains(&key)
        {
            false
        } else {
            self.emitted_implementations.as_mut().unwrap().insert(key);
            true
        }
    }

    pub fn need_implemtation<K: fmt::Display, V>(
        &mut self,
        impl_type: &ImplementationType,
        all: HashMap<K, V>,
    ) -> Vec<V> {
        all.into_iter()
            .filter_map(|(k, v)| {
                if self.needs_implementation(impl_type, &k.to_string()) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }

    // -----------------
    // Idents
    // -----------------
    #[cfg(not(test))]
    pub fn unique_ident(&mut self, ident: Ident) -> Ident {
        let idents = self.emitted_idents.as_mut().unwrap();
        let count: u8 = idents.get(&ident).unwrap_or(&0_u8) + 1;
        let id = format_ident!("{}_{}", ident, count);
        idents.insert(ident, count);
        id
    }

    // Test results need to be the same, no matter if we run just one or all in whichever order
    #[cfg(test)]
    pub fn unique_ident(&mut self, ident: Ident) -> Ident {
        let id = format_ident!("{}_1", ident);
        id
    }

    // -----------------
    // Exports
    // -----------------
    pub fn register_handled_impl_method_export(&mut self, ident: &Ident) {
        self.handled_impl_method_exports
            .as_mut()
            .unwrap()
            .insert(ident.to_string());
    }

    pub fn handled_impl_method_export(&self, ident: &Ident) -> bool {
        self.handled_impl_method_exports
            .as_ref()
            .unwrap()
            .contains(&ident.to_string())
    }
}

static mut STATE: ExpandState = ExpandState {
    initialized: false,
    emitted_implementations: None,
    emitted_idents: None,
    handled_impl_method_exports: None,
};

pub fn get_state() -> &'static mut ExpandState {
    // SAFETY: for now we assume that rust expansion doesn't work in parallel, otherwise we'd need
    // to make this thread safe (see Arc)
    unsafe {
        STATE.init();
        &mut STATE
    }
}
