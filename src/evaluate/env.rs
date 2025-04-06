use std::collections::HashMap;

use crate::literal::Literal;

pub(crate) struct Environment {
    pub(super) value_map: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            value_map: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Literal> {
        self.value_map.get(name)
    }

    pub fn set(&mut self, name: String, value: Literal) {
        self.value_map.insert(name, value);
    }
}
