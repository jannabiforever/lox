use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::{binding_power::BindingPower, ExprAst, ExprParseError, ExprParser};
use crate::{
    env::{Env, Evaluatable, RuntimeError},
    literal::LoxValue,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Assign<'a> {
    pub assignee: Box<ExprAst<'a>>,
    pub value: Box<ExprAst<'a>>,
}

impl fmt::Display for Assign<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(= {} {})", self.assignee, self.value)
    }
}

impl<'a> ExprParser<'a, '_> {
    pub(super) fn parse_assign(&mut self, left: ExprAst<'a>) -> Result<Assign<'a>, ExprParseError> {
        self.token_stream.next(); // consume the '='

        let right = self.parse_within_binding_power(BindingPower::AssignRight)?;
        Ok(Assign {
            assignee: Box::new(left),
            value: Box::new(right),
        })
    }
}

impl<'a> Evaluatable for Assign<'a> {
    fn eval<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<LoxValue, RuntimeError> {
        let name = match *self.assignee.clone() {
            ExprAst::Variable(var) => var.var,
            rest => return Err(RuntimeError::InvalidAssignmentTarget(rest.to_string())),
        };
        let value = (*self.value).eval(env.clone())?;

        if env.borrow_mut().update(name.src, value.clone()) {
            Ok(value)
        } else {
            Err(RuntimeError::UndefinedVariable(name.src.to_string()))
        }
    }
}
