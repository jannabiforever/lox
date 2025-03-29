use crate::parse::{
    expr_ast::{Assign, ExprAst},
    ParseError,
};

use super::binding_power::BindingPower;

impl super::ExprParser<'_, '_> {
    pub(super) fn parse_assign(&mut self, left: ExprAst) -> Result<Assign, ParseError> {
        self.next(); // consume the '='

        let right = self.parse_within_binding_power(BindingPower::Assign)?;
        Ok(Assign {
            assignee: Box::new(left),
            value: Box::new(right),
        })
    }
}
