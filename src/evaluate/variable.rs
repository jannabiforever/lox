use crate::literal::Literal;

use super::{EvaluateError, Evaluator};

impl Evaluator {
    pub(super) fn evaluate_variable(&self, name: &str) -> Result<Literal, EvaluateError> {
        if let Some(value) = self.env.borrow().get(name) {
            Ok(value.clone())
        } else {
            Err(EvaluateError::UndefinedVariable(name.to_string()))
        }
    }
}
