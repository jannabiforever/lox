use std::fmt;

use crate::literal::Literal;

pub enum ExprAst {
    Literal(Literal),
    Grouping(Grouping),
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

impl fmt::Display for ExprAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprAst::Literal(v) => write!(f, "{}", v),
            ExprAst::Grouping(v) => write!(f, "(group {})", v.inner),
        }
    }
}

pub struct Grouping {
    pub inner: Box<ExprAst>,
}
