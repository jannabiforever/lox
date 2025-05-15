use std::{cell::RefCell, io::Write, rc::Rc};

use super::{
    RuntimeError::{self, *},
    StmtParser,
};
use crate::{
    env::Runnable,
    error::{IntoLoxError, LoxError},
    expr::{Assign, ExprAst},
    literal::{Literal, LoxValue},
    statement::error::StmtParseError::{self, *},
    token::Token,
    Env, Evaluatable,
};

/// NOTE: lifetime 'a denotes the lifetime of source code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VarDecl<'src> {
    pub(crate) var: ExprAst<'src>,
    pub(crate) value: Option<ExprAst<'src>>,
}

impl<'src> Runnable<'src> for VarDecl<'src> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'src>>>,
        stdout: &mut W,
    ) -> Result<Option<LoxValue<'src>>, LoxError<RuntimeError>> {
        let var: Token<'src> = match &self.var {
            ExprAst::Variable(variable) => Ok(variable.var.clone()),
            rest => Err(InvalidAssignmentTarget(rest.to_string()).at(self.line())),
        }?;

        let value = match self.value.as_ref() {
            Some(value) => value.eval(env.clone(), stdout)?,
            None => Literal::Nil.into(),
        };

        env.borrow_mut().set(var.src, value);
        Ok(None)
    }

    fn line(&self) -> usize {
        if let Some(value) = self.value.as_ref() {
            value.line()
        } else {
            self.var.line()
        }
    }
}

impl<'src> StmtParser<'src, '_> {
    pub fn parse_var_decl(&mut self) -> Result<VarDecl<'src>, StmtParseError> {
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
            _ => Err(InvalidVarDecl(following.to_string())),
        };

        self.expect_semicolon()?;

        result
    }
}
