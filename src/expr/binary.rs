use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use crate::{
    env::{Env, Evaluatable, RuntimeError},
    literal::{Literal, LoxValue},
    mac::tt,
    token::TokenType,
};

use super::{binding_power::BindingPower, ExprAst, ExprParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<ExprAst>,
    pub op: BinaryOp,
    pub right: Box<ExprAst>,
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

    /// Get the binary function for the given operator.
    /// LL stands for LazyLiteral.
    #[allow(clippy::type_complexity)]
    fn get_binary_function<W: Write>(
        &self,
    ) -> fn(ExprAst, ExprAst, Rc<RefCell<Env<W>>>) -> Result<Literal, RuntimeError> {
        match self {
            BinaryOp::Star => {
                |left, right, env| match (left.eval(env.clone())?, right.eval(env)?) {
                    (
                        LoxValue::Literal(Literal::Number(left)),
                        LoxValue::Literal(Literal::Number(right)),
                    ) => Ok(Literal::Number(left * right)),
                    _ => Err(RuntimeError::OperandMustBe("numbers")),
                }
            }
            BinaryOp::Slash => |left, right, env| match (left.eval(env.clone())?, right.eval(env)?)
            {
                (
                    LoxValue::Literal(Literal::Number(left)),
                    LoxValue::Literal(Literal::Number(right)),
                ) => Ok(Literal::Number(left / right)),
                _ => Err(RuntimeError::OperandMustBe("numbers")),
            },
            BinaryOp::Plus => {
                |left, right, env| match (left.eval(env.clone())?, right.eval(env)?) {
                    (
                        LoxValue::Literal(Literal::Number(left)),
                        LoxValue::Literal(Literal::Number(right)),
                    ) => Ok(Literal::Number(left + right)),
                    (
                        LoxValue::Literal(Literal::String(left)),
                        LoxValue::Literal(Literal::String(right)),
                    ) => Ok(Literal::String(format!("{}{}", left, right))),
                    _ => Err(RuntimeError::OperandMustBe("numbers or strings")),
                }
            }
            BinaryOp::Minus => |left, right, env| match (left.eval(env.clone())?, right.eval(env)?)
            {
                (
                    LoxValue::Literal(Literal::Number(left)),
                    LoxValue::Literal(Literal::Number(right)),
                ) => Ok(Literal::Number(left - right)),
                _ => Err(RuntimeError::OperandMustBe("numbers")),
            },
            BinaryOp::Greater => {
                |left, right, env| match (left.eval(env.clone())?, right.eval(env)?) {
                    (
                        LoxValue::Literal(Literal::Number(left)),
                        LoxValue::Literal(Literal::Number(right)),
                    ) => Ok(Literal::Boolean(left > right)),
                    _ => Err(RuntimeError::OperandMustBe("numbers")),
                }
            }
            BinaryOp::GreaterEqual => {
                |left, right, env| match (left.eval(env.clone())?, right.eval(env)?) {
                    (
                        LoxValue::Literal(Literal::Number(left)),
                        LoxValue::Literal(Literal::Number(right)),
                    ) => Ok(Literal::Boolean(left >= right)),
                    _ => Err(RuntimeError::OperandMustBe("numbers")),
                }
            }
            BinaryOp::Less => {
                |left, right, env| match (left.eval(env.clone())?, right.eval(env)?) {
                    (
                        LoxValue::Literal(Literal::Number(left)),
                        LoxValue::Literal(Literal::Number(right)),
                    ) => Ok(Literal::Boolean(left < right)),
                    _ => Err(RuntimeError::OperandMustBe("numbers")),
                }
            }
            BinaryOp::LessEqual => {
                |left, right, env| match (left.eval(env.clone())?, right.eval(env)?) {
                    (
                        LoxValue::Literal(Literal::Number(left)),
                        LoxValue::Literal(Literal::Number(right)),
                    ) => Ok(Literal::Boolean(left <= right)),
                    _ => Err(RuntimeError::OperandMustBe("numbers")),
                }
            }
            BinaryOp::EqualEqual => |left, right, env| {
                let val = left.eval(env.clone())? == right.eval(env)?;
                Ok(val.into())
            },
            BinaryOp::BangEqual => |left, right, env| {
                let val = left.eval(env.clone())? != right.eval(env)?;
                Ok(val.into())
            },
            BinaryOp::And => |left, right, env| {
                if !left.eval(env.clone())?.is_literal_and(|l| l.is_truthy()) {
                    Ok(Literal::Boolean(false))
                } else {
                    match right.eval(env)? {
                        LoxValue::Literal(l) => Ok(l),
                        LoxValue::RustFunction(_) => todo!("Error?"),
                    }
                }
            },
            BinaryOp::Or => |left, right, env| {
                let left = left.eval(env.clone())?;
                if left.is_literal_and(|l| l.is_truthy()) {
                    match left {
                        LoxValue::Literal(l) => Ok(l),
                        LoxValue::RustFunction(_) => todo!("Error?"),
                    }
                } else {
                    match right.eval(env)? {
                        LoxValue::Literal(l) => Ok(l),
                        LoxValue::RustFunction(_) => todo!("Error?"),
                    }
                }
            },
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

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

impl super::ExprParser<'_, '_> {
    /// If following token is a binary operator,
    /// parse the right operand and return the binary expression,
    /// consuming the operator and the right operand.
    ///
    /// Otherwise, it doesn't consume anything and returns `None`.
    pub(super) fn try_parse_binary(
        &mut self,
        lhs: ExprAst,
    ) -> Option<Result<Binary, ExprParseError>> {
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

impl Evaluatable for Binary {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        let Self { left, op, right } = self.clone();
        let function = op.get_binary_function();
        Ok(function(*left, *right, env)?.into())
    }
}
