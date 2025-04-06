use std::collections::HashMap;

use crate::literal::Literal;

use super::EvaluateError;

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

    pub fn update(&mut self, name: String, value: Literal) -> Result<(), EvaluateError> {
        if let Some(existing_value) = self.value_map.get_mut(&name) {
            *existing_value = value.clone();
            Ok(())
        } else {
            Err(EvaluateError::UndefinedVariable(name))
        }
    }
}
