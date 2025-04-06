use std::fmt;

use super::{binding_power::BindingPower, ExprAst, ExprParseError};

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

impl super::ExprParser<'_, '_> {
    pub(super) fn parse_assign(&mut self, left: ExprAst) -> Result<Assign, ExprParseError> {
        self.token_stream.next(); // consume the '='

        let right = self.parse_within_binding_power(BindingPower::AssignRight)?;
        Ok(Assign {
            assignee: Box::new(left),
            value: Box::new(right),
        })
    }
}
