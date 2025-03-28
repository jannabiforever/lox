use std::fmt;

use crate::tokenize::{tt, TokenType};

use super::ExprAst;

#[derive(Debug, Clone)]
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
