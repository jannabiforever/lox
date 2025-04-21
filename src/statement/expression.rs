use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtParseError, StmtParser};
use crate::{env::Runnable, error::LoxError, expr::ExprAst, literal::LoxValue, Env, Evaluatable};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Expression<'a> {
    pub(crate) expr: ExprAst<'a>,
}

impl<'a> Runnable<'a> for Expression<'a> {
    fn run<W: Write>(
        &self,
        env: Rc<RefCell<Env<'a, W>>>,
    ) -> Result<Option<LoxValue<'a>>, LoxError<RuntimeError>> {
        self.expr.eval(env)?;
        Ok(None)
    }

    fn line(&self) -> usize {
        self.expr.line()
    }
}

impl<'a> StmtParser<'a, '_> {
    pub(super) fn parse_expression_stmt(&mut self) -> Result<Expression<'a>, StmtParseError> {
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;
        Ok(Expression { expr })
    }
}
