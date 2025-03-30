use crate::{
    literal::Literal,
    parse::{Unary, UnaryOp},
};

use super::{error::EvaluateError, Evaluator};

impl Evaluator {
    pub(super) fn evaluate_unary(&self, unary: &Unary) -> Result<Literal, EvaluateError> {
        let right = self.eval(&unary.right);

        match unary.op {
            UnaryOp::Minus => {
                if let Literal::Number(num) = right? {
                    Ok(Literal::Number(-num))
                } else {
                    Err(EvaluateError::OperandMustBe("number"))
                }
            }
            UnaryOp::Bang => Ok(Literal::Boolean(!right?.is_truthy())),
        }
    }
}
