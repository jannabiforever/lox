use std::fmt;

use super::ExprAst;

pub struct FieldCall {
    pub object: Box<ExprAst>,
    pub field: String,
}

impl fmt::Display for FieldCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.object, self.field)
    }
}
