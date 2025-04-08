use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    env::{Env, Evaluatable, EvaluateError},
    literal::Literal,
};

use super::{binding_power::BindingPower, ExprAst, ExprParseError};

#[derive(Debug, Clone, PartialEq)]
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

impl Evaluatable for Assign {
    fn eval(&self, env: Rc<RefCell<Env>>) -> Result<Literal, EvaluateError> {
        let name = match *self.assignee.clone() {
            ExprAst::Variable(var) => var.name,
            rest => return Err(EvaluateError::InvalidAssignmentTarget(rest.to_string())),
        };
        let value = (*self.value).eval(env.clone())?;

        if env.borrow_mut().update(&name, value.clone()) {
            Ok(value)
        } else {
            Err(EvaluateError::UndefinedVariable(name))
        }
    }
}
