use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtParser};
use crate::{
    env::Runnable,
    expr::{Assign, ExprAst},
    literal::{Literal, LoxValue},
    statement::error::StmtParseError,
    Env, Evaluatable,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VarDecl {
    pub(crate) var: ExprAst,
    pub(crate) value: Option<ExprAst>,
}

impl Runnable for VarDecl {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let var = match &self.var {
            ExprAst::Variable(variable) => Ok(variable.name.clone()),
            rest => Err(RuntimeError::InvalidAssignmentTarget(rest.to_string())),
        }?;

        let value = match self.value.as_ref() {
            Some(value) => value.eval(env.clone())?,
            None => Literal::Nil.into(),
        };

        env.borrow_mut().set(&var, value);
        Ok(None)
    }
}

impl StmtParser<'_, '_> {
    pub fn parse_var_decl(&mut self) -> Result<VarDecl, StmtParseError> {
        self.token_stream.next(); // consume the 'var' token.
        let following = self.parse_following_expression()?;

        let result = match following {
            // e.g. var x;
            ExprAst::Variable(_) => Ok(VarDecl {
                var: following,
                value: None,
            }),
            // e.g. var x = 1;
            ExprAst::Assign(Assign { assignee, value }) => Ok(VarDecl {
                var: *assignee.clone(),
                value: Some(*value.clone()),
            }),
            _ => Err(StmtParseError::InvalidVarDecl(following.to_string())),
        };

        self.expect_semicolon()?;

        result
    }
}
