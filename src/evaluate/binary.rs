use std::{ops::Deref, sync::LazyLock};

use crate::{
    literal::Literal,
    parse::{Binary, BinaryOp},
};

use super::{error::EvaluateError, Evaluator};

impl Evaluator {
    pub(super) fn evaluate_binary(&self, binary: &Binary) -> Result<Literal, EvaluateError> {
        let left = self.eval(&binary.left);
        // Lazily evaluate the right side of the binary expression, for short-circuiting.
        let function = self.get_binary_function(&binary.op);
        let right = LazyLock::new(|| self.eval(&binary.right));

        function(left?, right)
    }

    /// Get the binary function for the given operator.
    /// LL stands for LazyLiteral.
    fn get_binary_function<LL>(
        &self,
        op: &BinaryOp,
    ) -> fn(Literal, LL) -> Result<Literal, EvaluateError>
    where
        LL: Deref<Target = Result<Literal, EvaluateError>>,
    {
        match op {
            BinaryOp::Star => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left * right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::Slash => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left / right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::Plus => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left + right))
                    }
                    (Literal::String(left), Literal::String(right)) => {
                        Ok(Literal::String(format!("{}{}", left, right)))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers or strings")),
                })
            },
            BinaryOp::Minus => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left - right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::Greater => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Boolean(left > right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::GreaterEqual => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Boolean(left >= right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::Less => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Boolean(left < right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::LessEqual => |left, right| {
                right.deref().clone().and_then(|right| match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Boolean(left <= right))
                    }
                    _ => Err(EvaluateError::OperandMustBe("numbers")),
                })
            },
            BinaryOp::EqualEqual => |left, right| {
                right
                    .deref()
                    .clone()
                    .map(|right| Literal::Boolean(left == right))
            },
            BinaryOp::BangEqual => |left, right| {
                right
                    .deref()
                    .clone()
                    .map(|right| Literal::Boolean(left != right))
            },
            BinaryOp::And => |left, right| {
                if !left.is_truthy() {
                    Ok(Literal::Boolean(false))
                } else {
                    right
                        .deref()
                        .clone()
                        .map(|right| Literal::Boolean(right.is_truthy()))
                }
            },
            BinaryOp::Or => |left, right| {
                if left.is_truthy() {
                    Ok(Literal::Boolean(true))
                } else {
                    right
                        .deref()
                        .clone()
                        .map(|right| Literal::Boolean(right.is_truthy()))
                }
            },
        }
    }
}
