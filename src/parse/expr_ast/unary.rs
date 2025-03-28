use std::fmt;

use crate::tokenize::{tt, TokenType};

use super::ExprAst;

#[derive(Debug, Clone)]
pub struct Unary {
    pub op: UnaryOp,
    pub right: Box<ExprAst>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl UnaryOp {
    pub(crate) fn from_token_type(token_type: TokenType) -> Option<UnaryOp> {
        match token_type {
            tt!("!") => Some(Self::Bang),
            tt!("-") => Some(Self::Minus),
            _ => None,
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),
            Self::Minus => write!(f, "-"),
        }
    }
}
