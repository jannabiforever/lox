use crate::{literal::Literal, parse::Binary};

use super::{error::EvaluateError, Evaluator};

impl Evaluator {
    pub(super) fn evaluate_binary(&self, _: &Binary) -> Result<Literal, EvaluateError> {
        todo!("self.evaluate_binary")
    }
}
