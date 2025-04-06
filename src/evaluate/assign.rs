use crate::{
    literal::Literal,
    parse::{Assign, ExprAst},
};

use super::{EvaluateError, Evaluator};

impl Evaluator {
    pub(super) fn evaluate_assign(&self, assign: &Assign) -> Result<Literal, EvaluateError> {
        // Evaluate the value to assign.
        // TODO: trait assignable? for hanlding different types of assignable values, e.g. fields
        let name = match *assign.assignee.clone() {
            ExprAst::Variable(name) => name,
            rest => return Err(EvaluateError::InvalidAssignmentTarget(rest.to_string())),
        };
        let value = self.eval(&assign.value)?;

        self.env.borrow_mut().set(name, value.clone());

        // Return the value that was assigned.
        Ok(value)
    }
}
