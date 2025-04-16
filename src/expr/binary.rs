use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::{binding_power::BindingPower, ExprAst, ExprParseError, ExprParser};
use crate::{
    env::{
        Env, Evaluatable,
        RuntimeError::{self, *},
    },
    error::{IntoLoxError, LoxError},
    literal::{Literal, LoxValue},
    mac::tt,
    token::TokenType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Binary<'a> {
    pub left: Box<ExprAst<'a>>,
    pub op: BinaryOp,
    pub right: Box<ExprAst<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

impl BinaryOp {
    pub(crate) fn from_token_type(token_type: TokenType) -> Option<BinaryOp> {
        match token_type {
            tt!("+") => Some(Self::Plus),
            tt!("-") => Some(Self::Minus),
            tt!("*") => Some(Self::Star),
            tt!("/") => Some(Self::Slash),
            tt!("==") => Some(Self::EqualEqual),
            tt!("!=") => Some(Self::BangEqual),
            tt!("<") => Some(Self::Less),
            tt!("<=") => Some(Self::LessEqual),
            tt!(">") => Some(Self::Greater),
            tt!(">=") => Some(Self::GreaterEqual),
            tt!("and") => Some(Self::And),
            tt!("or") => Some(Self::Or),
            _ => None,
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::EqualEqual => write!(f, "=="),
            Self::BangEqual => write!(f, "!="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}

impl fmt::Display for Binary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

impl<'a> ExprParser<'a, '_> {
    /// If following token is a binary operator,
    /// parse the right operand and return the binary expression,
    /// consuming the operator and the right operand.
    ///
    /// Otherwise, it doesn't consume anything and returns `None`.
    pub(super) fn try_parse_binary(
        &mut self,
        lhs: ExprAst<'a>,
    ) -> Option<Result<Binary<'a>, ExprParseError>> {
        let op = self.eat_binary_op()?;

        let binding_power: (BindingPower, BindingPower) = op.into();
        let right_binding_power = binding_power.1;

        let right = match self.parse_within_binding_power(right_binding_power) {
            Ok(inner) => Box::new(inner),
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(Binary {
            left: Box::new(lhs),
            op,
            right,
        }))
    }

    fn eat_binary_op(&mut self) -> Option<BinaryOp> {
        let token_type = self.token_stream.peek().token_type;
        BinaryOp::from_token_type(token_type).inspect(|_| {
            self.token_stream.next();
        })
    }
}

/// Casts given expressions to Number, and do operation.
macro_rules! number_operation {
    ($left:expr, $right:expr, $env:expr, $func:expr) => {{
        let left = eval_and_cast_to_literal(&$left, $env.clone())?
            .number_or(OperandMustBe("number").error_at($left.line()))?;

        let right = eval_and_cast_to_literal(&$right, $env.clone())?
            .number_or(OperandMustBe("number").error_at($right.line()))?;

        Ok(LoxValue::Literal($func(left, right).into()))
    }};
}

/// Casts given expressions to String, and do operation.
macro_rules! string_operation {
    ($left:expr, $right:expr, $env:expr, $func:expr) => {{
        let left = eval_and_cast_to_literal(&$left, $env.clone())?
            .string_or(OperandMustBe("string").error_at($left.line()))?;

        let right = eval_and_cast_to_literal(&$right, $env.clone())?.string_or(
            $crate::env::RuntimeError::OperandMustBe("string").error_at($right.line()),
        )?;

        Ok($crate::literal::LoxValue::Literal(
            $func(left, right).into(),
        ))
    }};
}

impl Evaluatable for Binary<'_> {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, LoxError<RuntimeError>> {
        let Self { left, op, right } = self.clone();

        match op {
            BinaryOp::Star => {
                number_operation!(left, right, env, |l, r| l * r)
            }

            BinaryOp::Slash => {
                number_operation!(left, right, env, |l, r| l / r)
            }

            BinaryOp::Plus => match eval_and_cast_to_literal(&left, env.clone())? {
                Literal::Number(_) => number_operation!(left, right, env, |l, r| l + r),
                Literal::String(_) => {
                    string_operation!(left, right, env, |l: String, r: String| l + &r)
                }
                _ => Err(OperandMustBe("two numbers or two strings").error_at(left.line())),
            },

            BinaryOp::Minus => {
                number_operation!(left, right, env, |l, r| l - r)
            }

            BinaryOp::Greater => {
                number_operation!(left, right, env, |l, r| l > r)
            }

            BinaryOp::GreaterEqual => {
                number_operation!(left, right, env, |l, r| l >= r)
            }

            BinaryOp::Less => {
                number_operation!(left, right, env, |l, r| l < r)
            }

            BinaryOp::LessEqual => {
                number_operation!(left, right, env, |l, r| l <= r)
            }

            // Operations below does not require each side to be literal.
            BinaryOp::EqualEqual => {
                let val = left.eval(env.clone())? == right.eval(env)?;
                Ok(LoxValue::Literal(Literal::Boolean(val)))
            }

            BinaryOp::BangEqual => {
                let val = left.eval(env.clone())? != right.eval(env)?;
                Ok(LoxValue::Literal(Literal::Boolean(val)))
            }

            BinaryOp::And => {
                if !left.eval(env.clone())?.is_literal_and(|l| l.is_truthy()) {
                    Ok(LoxValue::Literal(Literal::Boolean(false)))
                } else {
                    right.eval(env)
                }
            }

            BinaryOp::Or => {
                let left = left.eval(env.clone())?;
                if left.is_literal_and(|l| l.is_truthy()) {
                    Ok(left)
                } else {
                    right.eval(env)
                }
            }
        }
    }

    fn line(&self) -> usize {
        self.right.line()
    }
}

fn eval_and_cast_to_literal<W: Write>(
    expr: &ExprAst,
    env: Rc<RefCell<Env<W>>>,
) -> Result<Literal, LoxError<RuntimeError>> {
    expr.eval(env)?
        .literal_or(OperandMustBe("literal").error_at(expr.line()))
}
