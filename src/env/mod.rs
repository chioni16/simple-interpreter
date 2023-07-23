use std::{cell::RefCell, collections::HashMap, rc::Rc};
// use crate::object::Object;

// pub(crate) type Env = Rc<RefCell<InnerEnv>>;

#[derive(Debug, Clone)]
pub struct Env<T> {
    store: HashMap<String, T>,
    outer: Option<Rc<RefCell<Env<T>>>>,
}

impl<T> Env<T> {
    pub fn new() -> Rc<RefCell<Self>> {
        let env = Self {
            store: HashMap::new(),
            outer: None,
        };
        Rc::new(RefCell::new(env))
    }

    pub fn extend(older: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let env = Self {
            store: HashMap::new(),
            outer: Some(older),
        };
        Rc::new(RefCell::new(env))
    }

    pub fn set(&mut self, key: String, val: T) {
        self.store.insert(key, val);
    }

    // pub fn add_new_context(&mut self) {
    //     let new_env = Env {
    //         store: HashMap::new(),
    //         outer: None,
    //     };
    //     let old_env = std::mem::replace(self, new_env);
    //     self.outer = Some(Rc::new(RefCell::new(old_env)));
    // }

    // pub fn get_prev_context(&mut self) {
    //     // can safely unwrap as this is only called after `add_new_context`
    //     // i.e, in the "epilogue" and `add_new_context` comes in the "prologue"
    //     let old_env = self.outer.take().unwrap();
    //     let old_env = old_env.into_inner();
    //     *self = old_env;
    // }
}

impl<T: Clone> Env<T> {
    pub fn get(&self, key: &String) -> Option<T> {
        self.store.get(key).cloned().or(self
            .outer
            .as_ref()
            .and_then(|outer| outer.borrow().get(key)))
    }
}
