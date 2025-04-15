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
pub(crate) struct VarDecl<'a> {
    pub(crate) var: ExprAst<'a>,
    pub(crate) value: Option<ExprAst<'a>>,
}

impl Runnable for VarDecl<'_> {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        let var = match &self.var {
            ExprAst::Variable(variable) => Ok(variable.var.clone()),
            rest => Err(RuntimeError::InvalidAssignmentTarget(rest.to_string())),
        }?;

        let value = match self.value.as_ref() {
            Some(value) => value.eval(env.clone())?,
            None => Literal::Nil.into(),
        };

        env.borrow_mut().set(&var.src, value);
        Ok(None)
    }
}

impl<'a> StmtParser<'a, '_> {
    pub fn parse_var_decl(&mut self) -> Result<VarDecl<'a>, StmtParseError> {
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
