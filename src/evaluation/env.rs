use std::collections::HashMap;
use crate::object::Object;

pub(super) struct Env {
    store: HashMap<String, Object>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, key: &String) -> Option<Object> {
        self
        .store.get(key).cloned()
        .or(self.outer.as_ref().map(|outer| outer.get(key)).flatten())
    }

    pub fn set(&mut self, key: String, val: &Object) {
        self.store.insert(key, val.clone());
    }
}