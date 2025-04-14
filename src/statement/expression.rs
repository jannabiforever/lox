use std::{cell::RefCell, io::Write, rc::Rc};

use super::{RuntimeError, StmtParseError, StmtParser};
use crate::{env::Runnable, expr::ExprAst, literal::LoxValue, Env, Evaluatable};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Expression {
    pub(crate) expr: ExprAst,
}

impl Runnable for Expression {
    fn run<W: Write>(&self, env: Rc<RefCell<Env<W>>>) -> Result<Option<LoxValue>, RuntimeError> {
        self.expr.eval(env)?;
        Ok(None)
    }
}

impl StmtParser<'_, '_> {
    pub(super) fn parse_expression_stmt(&mut self) -> Result<Expression, StmtParseError> {
        let expr = self.parse_following_expression()?;
        self.expect_semicolon()?;
        Ok(Expression { expr })
    }
}
