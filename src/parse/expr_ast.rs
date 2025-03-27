use std::fmt;

use crate::literal::Literal;

pub enum ExprAst {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
}

impl From<Literal> for ExprAst {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<Grouping> for ExprAst {
    fn from(grouping: Grouping) -> Self {
        Self::Grouping(grouping)
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
            ExprAst::Literal(v) => write!(f, "{}", v),
            ExprAst::Grouping(v) => write!(f, "(group {})", v.inner),
            ExprAst::Unary(v) => write!(f, "({} {})", v.op, v.right),
        }
    }
}

pub struct Grouping {
    pub inner: Box<ExprAst>,
}

pub struct Unary {
    pub op: UnaryOp,
    pub right: Box<ExprAst>,
}

pub enum UnaryOp {
    Minus,
    Bang,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        }
    }
}
