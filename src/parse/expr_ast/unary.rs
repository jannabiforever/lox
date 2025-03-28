use std::fmt;

use super::{BindingPower, ExprAst, Operator};

pub struct Unary {
    pub op: UnaryOp,
    pub right: Box<ExprAst>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}

pub enum UnaryOp {
    Bang,
    Minus,
}

impl Operator for UnaryOp {
    fn binding_power(&self) -> BindingPower {
        match self {
            Self::Bang => BindingPower::Bang,
            Self::Minus => BindingPower::UnaryMinus,
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
