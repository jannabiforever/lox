use std::{cell::RefCell, fmt, io::Write, rc::Rc};

use super::{binding_power::BindingPower, ExprAst, ExprParseError, ExprParser};
use crate::{
    env::{
        Env, Evaluatable,
        RuntimeError::{self, *},
    },
    error::{IntoLoxError, LoxError},
    literal::LoxValue,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Assign<'src> {
    pub assignee: Box<ExprAst<'src>>,
    pub value: Box<ExprAst<'src>>,
}

impl fmt::Display for Assign<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(= {} {})", self.assignee, self.value)
    }
}

impl<'src> ExprParser<'src, '_> {
    pub(super) fn parse_assign(
        &mut self,
        left: ExprAst<'src>,
    ) -> Result<Assign<'src>, ExprParseError> {
        self.token_stream.next(); // consume the '='

        let right = self.parse_within_binding_power(BindingPower::AssignRight)?;
        Ok(Assign {
            assignee: Box::new(left),
            value: Box::new(right),
        })
    }
}

impl<'src> Evaluatable<'src> for Assign<'src> {
    fn eval<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<LoxValue<'src>, LoxError<RuntimeError>> {
        let name = match *self.assignee.clone() {
            ExprAst::Variable(var) => var.var,
            rest => return Err(InvalidAssignmentTarget(rest.to_string()).at(self.line())),
        };
        let value = (*self.value).eval(env.clone(), stdout)?;

        if env.borrow_mut().update(name.src, value.clone()) {
            Ok(value)
        } else {
            Err(UndefinedVariable(name.src.to_string()).at(self.line()))
        }
    }

    fn line(&self) -> usize {
        self.assignee.line()
    }
}
