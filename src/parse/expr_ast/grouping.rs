use std::fmt;

use super::ExprAst;

#[derive(Debug, Clone)]
pub struct Grouping {
    pub inner: Box<ExprAst>,
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.inner)
    }
}
