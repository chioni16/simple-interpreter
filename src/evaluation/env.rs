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

    // pub fn update_outer(&mut self, outer_env: Env) {
    //     self.outer = Some(Box::from(outer_env));
    // }

    pub fn get(&self, key: &String) -> Option<Object> {
        self
        .store.get(key).cloned()
        .or(self.outer.as_ref().map(|outer| outer.get(key)).flatten())
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }

    pub fn add_new_context(&mut self) {
        let new_env = Env::new();
        let old_env = std::mem::replace(self, new_env);
        self.outer = Some(Box::from(old_env));
        // self.update_outer(old_env);
    }

    pub fn get_prev_context(&mut self) {
        let old_env = *self.outer.take().unwrap();
        *self = old_env;
    }
}