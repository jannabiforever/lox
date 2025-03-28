use std::fmt;

use super::ExprAst;

#[derive(Debug, Clone)]
pub struct Assign {
    pub assignee: Box<ExprAst>,
    pub value: Box<ExprAst>,
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(= {} {})", self.assignee, self.value)
    }
}
