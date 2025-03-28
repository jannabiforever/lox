mod binary;
mod field_call;
mod function_call;
mod grouping;
mod operator;
mod unary;

use std::fmt;

use crate::literal::Literal;

pub(crate) use self::binary::{Binary, BinaryOp};
pub(crate) use self::field_call::FieldCall;
pub(crate) use self::function_call::FunctionCall;
pub(crate) use self::grouping::Grouping;
pub(crate) use self::operator::{BindingPower, Operator};
pub(crate) use self::unary::{Unary, UnaryOp};

pub enum ExprAst {
    Binary(Binary),
    FieldCall(FieldCall),
    FunctionCall(FunctionCall),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl From<Binary> for ExprAst {
    fn from(value: Binary) -> Self {
        Self::Binary(value)
    }
}

impl From<Grouping> for ExprAst {
    fn from(grouping: Grouping) -> Self {
        Self::Grouping(grouping)
    }
}

impl From<Literal> for ExprAst {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<Unary> for ExprAst {
    fn from(unary: Unary) -> Self {
        Self::Unary(unary)
    }
}

impl fmt::Display for ExprAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary(v) => write!(f, "{}", v),
            Self::FieldCall(v) => write!(f, "{}", v),
            Self::FunctionCall(v) => write!(f, "{}", v),
            Self::Grouping(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Unary(v) => write!(f, "{}", v),
        }
    }
}
