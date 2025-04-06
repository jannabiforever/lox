use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::literal::Literal;

pub(crate) struct Environment {
    pub(crate) parent: Option<Rc<RefCell<Environment>>>,
    pub(crate) scope: HashMap<String, Literal>,
}

impl Environment {
    /// Creates a global environment,
    pub fn new() -> Self {
        Self {
            parent: None,
            scope: HashMap::new(),
        }
    }

    /// New child environment instance.
    pub fn from_parent(parent: &Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent.clone()),
            scope: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Literal> {
        if let Some(value) = self.scope.get(key) {
            Some(value.clone())
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow().get(key)
        } else {
            None
        }
    }

    /// Initializes the key-value pair at current scope.
    pub fn set(&mut self, key: &str, value: Literal) {
        self.scope.insert(key.to_string(), value);
    }

    /// Updates the value stored in the hashmap. If fails, returns false.
    pub fn update(&mut self, key: &str, value: Literal) -> bool {
        if let Some(existing_value) = self.scope.get_mut(key) {
            *existing_value = value.clone();
            true
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow_mut().update(key, value)
        } else {
            false
        }
    }
}
