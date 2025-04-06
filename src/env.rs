use std::collections::HashMap;

use crate::literal::Literal;

pub(crate) struct Environment {
    pub(crate) value_map: HashMap<String, Literal>,
}

impl Environment {
    /// Creates a global environment,
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Literal> {
        self.value_map.get(name)
    }

    /// Initializes the key-value pair.
    pub fn set(&mut self, name: &str, value: Literal) {
        self.value_map.insert(name.to_string(), value);
    }

    /// Updates the value stored in the hashmap. If fails, returns false.
    pub fn update(&mut self, name: &str, value: Literal) -> bool {
        if let Some(existing_value) = self.value_map.get_mut(name) {
            *existing_value = value.clone();
            true
        } else {
            false
        }
    }
}
