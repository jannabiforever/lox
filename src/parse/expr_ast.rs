use std::fmt;

use crate::literal::Literal;

pub enum ExprAst {
    Literal(Literal),
}

impl fmt::Display for ExprAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprAst::Literal(v) => write!(f, "{}", v),
        }
    }
}
