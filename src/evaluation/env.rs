use std::collections::HashMap;
use crate::object::Object;

pub(super) struct Env(HashMap<String, Object>);

impl Env {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &String) -> Option<Object> {
        self.0.get(key).cloned()
    }

    pub fn set(&mut self, key: String, val: &Object) {
        self.0.insert(key, val.clone());
    }
}